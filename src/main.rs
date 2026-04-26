use anyhow::{anyhow, Result};
use ed25519_dalek::{Keypair, Signer, Verifier, Signature};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha3::{Sha3_256, Digest};
use std::sync::{Arc, Mutex};
use wasm_encoder::{CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction, Module, TypeSection, ValType};
use wasmtime::{Engine, Linker, Module as WasmModule, Store, TypedFunc};
use uuid::Uuid;

// ---------- Компилятор OPL ----------
#[derive(Debug, Clone)]
pub enum Rule {
    OutputRange { id: String, output_index: u32, min: f32, max: f32 },
}

impl Rule {
    fn compile_to_wasm(&self) -> Result<Vec<u8>> {
        let mut module = Module::new();
        // Типы: функция (f32) -> i32
        let mut types = TypeSection::new();
        types.function([ValType::F32], [ValType::I32]);
        module.section(&types);
        // Функции: одна функция, ссылается на тип 0
        let mut functions = FunctionSection::new();
        functions.function(0);
        module.section(&functions);
        // Экспорт: "validate" указывает на функцию index 0
        let mut exports = ExportSection::new();
        exports.export("validate", ExportKind::Func, 0);
        module.section(&exports);

        let mut code = CodeSection::new();
        let mut f = Function::new([]);
        match self {
            Rule::OutputRange { max, .. } => {
                // Локальная переменная 0 = параметр rate
                f.instruction(&Instruction::LocalGet(0));
                f.instruction(&Instruction::F32Const(*max));
                f.instruction(&Instruction::F32Gt);
                f.instruction(&Instruction::If(wasm_encoder::BlockType::Value(ValType::I32)));
                f.instruction(&Instruction::I32Const(1)); // violation
                f.instruction(&Instruction::Else);
                f.instruction(&Instruction::I32Const(0)); // ok
                f.instruction(&Instruction::End);
            }
        }
        f.instruction(&Instruction::End);
        code.function(&f);
        module.section(&code);
        Ok(module.finish())
    }
}

pub fn compile_rule_from_yaml(yaml_str: &str) -> Result<Vec<u8>> {
    let value: serde_yaml::Value = serde_yaml::from_str(yaml_str)?;
    let rule_type = value["rules"][0]["type"].as_str().ok_or(anyhow!("missing rule type"))?;
    if rule_type != "output_range" {
        return Err(anyhow!("only output_range supported in demo"));
    }
    let id = value["rules"][0]["id"].as_str().unwrap_or("rule").to_string();
    let max = value["rules"][0]["max"].as_f64().unwrap_or(25.0) as f32;
    let min = value["rules"][0]["min"].as_f64().unwrap_or(0.0) as f32;
    let rule = Rule::OutputRange { id, output_index: 0, min, max };
    rule.compile_to_wasm()
}

// ---------- Симулятор Sentinel ----------
#[derive(Clone)]
pub struct SentinelSim {
    keypair: Arc<Keypair>,
    log: Arc<Mutex<Vec<TransactionRecord>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionRecord {
    pub tx_id: String,
    pub timestamp: u64,
    pub status: String,
    pub policy_hash: String,
    pub encrypted_hash: String,
    pub signature: String,
    pub violation_proof: Option<ViolationProof>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ViolationProof {
    pub rule_id: String,
    pub threshold: f32,
    pub actual_value: f32,
    pub sentinel_sig: String,
}

impl SentinelSim {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let keypair = Keypair::generate(&mut csprng);
        Self { keypair: Arc::new(keypair), log: Arc::new(Mutex::new(Vec::new())) }
    }

    pub fn sign(&self, tx_id: &str, encrypted_hash: &str, status: &str, policy_hash: &str) -> String {
        let message = format!("{}:{}:{}:{}", tx_id, encrypted_hash, status, policy_hash);
        let signature = self.keypair.sign(message.as_bytes());
        hex::encode(signature.to_bytes())
    }

    pub fn log_transaction(&self, record: TransactionRecord) {
        self.log.lock().unwrap().push(record);
    }

    pub fn get_log(&self) -> Vec<TransactionRecord> {
        self.log.lock().unwrap().clone()
    }

    pub fn verify_proof(&self, proof: &ViolationProof) -> bool {
        let sig_bytes = hex::decode(&proof.sentinel_sig).unwrap();
        let signature = Signature::from_bytes(&sig_bytes).unwrap();
        let message = format!("violation:{}:{}", proof.rule_id, proof.threshold);
        self.keypair.verify(message.as_bytes(), &signature).is_ok()
    }
}

// ---------- TEE Host (рантайм) ----------
pub struct TeeHost {
    engine: Engine,
    sentinel: SentinelSim,
}

impl TeeHost {
    pub fn new(sentinel: SentinelSim) -> Self {
        Self { engine: Engine::default(), sentinel }
    }

    pub fn process_inference(&self, wasm_bytes: &[u8], model_output_value: f32, policy_hash: &str) -> Result<TransactionRecord> {
        let module = WasmModule::new(&self.engine, wasm_bytes)?;
        let mut store = Store::new(&self.engine, ());
        let instance = module.instantiate(&mut store, &Linker::new(&self.engine))?;
        let validate_func = instance.get_typed_func::<(f32,), i32>(&mut store, "validate")?;
        let result = validate_func.call(&mut store, model_output_value)?;

        let tx_id = Uuid::new_v4().to_string();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let status = if result == 0 { "green" } else { "red" };
        // Логическая блокировка: имитация шифрования
        let plain_output = format!("interest_rate: {}", model_output_value);
        let key = rand::random::<[u8; 32]>();
        let encrypted: Vec<u8> = plain_output.bytes().zip(key.iter().cycle()).map(|(b, k)| b ^ k).collect();
        let encrypted_hash = format!("sha3:{}", hex::encode(Sha3_256::digest(&encrypted)));

        let signature = self.sentinel.sign(&tx_id, &encrypted_hash, status, policy_hash);
        let violation_proof = if result != 0 {
            Some(ViolationProof {
                rule_id: "R003_usury".to_string(),
                threshold: 20.0, // из политики v2
                actual_value: model_output_value,
                sentinel_sig: signature.clone(),
            })
        } else {
            None
        };

        let record = TransactionRecord {
            tx_id,
            timestamp,
            status: status.to_string(),
            policy_hash: policy_hash.to_string(),
            encrypted_hash,
            signature,
            violation_proof,
        };
        self.sentinel.log_transaction(record.clone());
        Ok(record)
    }
}

// ---------- Dashboard CLI ----------
fn main() -> Result<()> {
    // Политика v1: max 25%
    let policy_yaml_v1 = r#"
id: "credit_policy_v1"
rules:
  - id: "R003_usury"
    type: "output_range"
    output_name: "interest_rate"
    min: 0.0
    max: 25.0
"#;
    let wasm_v1 = compile_rule_from_yaml(policy_yaml_v1)?;
    let hash_v1 = format!("0x{}", hex::encode(Sha3_256::digest(&wasm_v1)));
    println!("✅ Compiled policy v1 (max=25%). Hash: {}", hash_v1);

    let sentinel = SentinelSim::new();
    let host = TeeHost::new(sentinel.clone());

    // Зелёная транзакция
    let green = host.process_inference(&wasm_v1, 18.0, &hash_v1)?;
    println!("\n🟢 GREEN transaction: {}", green.tx_id);
    println!("   Signature: {}", &green.signature[..16]);

    // Политика v2: max 20%
    let policy_yaml_v2 = r#"
id: "credit_policy_v2"
rules:
  - id: "R003_usury"
    type: "output_range"
    output_name: "interest_rate"
    min: 0.0
    max: 20.0
"#;
    let wasm_v2 = compile_rule_from_yaml(policy_yaml_v2)?;
    let hash_v2 = format!("0x{}", hex::encode(Sha3_256::digest(&wasm_v2)));
    println!("\n🔄 Policy updated (max=20%). New hash: {}", hash_v2);

    // Красная транзакция
    let red = host.process_inference(&wasm_v2, 22.0, &hash_v2)?;
    println!("\n🔴 RED transaction: {}", red.tx_id);
    if let Some(p) = &red.violation_proof {
        println!("   Violation proof: rule={}, threshold={}, actual={}",
                 p.rule_id, p.threshold, p.actual_value);
    }

    // Верификация
    if let Some(p) = &red.violation_proof {
        let valid = sentinel.verify_proof(p);
        println!("\n🔍 Verifying violation proof: {}", if valid { "✅ VALID" } else { "❌ INVALID" });
    }

    // Дашборд
    println!("\n📋 Dashboard: last 3 transactions");
    for rec in sentinel.get_log().iter().rev().take(3) {
        println!("   {} | {} | {}", rec.timestamp, rec.tx_id, rec.status);
    }

    Ok(())
}
