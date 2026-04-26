

**Active Cryptographic Firewall for Regulated AI**

[![Rust](https://img.shields.io/badge/made%20with-Rust-red)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](LICENSE)

## The Problem

AI models (LLMs, credit scorers, medical diagnostic systems) are black boxes. Banks, insurers, and healthcare providers cannot prove to regulators that their models are **fair, legal, and non‑discriminatory** – even when they genuinely are.

- ❌ No mathematical guarantee that a decision wasn't based on age/gender/zip code.
- ❌ No way to audit a model without revealing proprietary weights.
- ❌ No active prevention – only post‑factum investigations.
- ❌ **Real‑world impact:** Unchecked algorithms can deny loans to certain ethnic groups **40-80% more often** at similar incomes (via hidden zip‑code correlations).
- ❌ **Regulatory fines:** Under EU AI Act, non‑compliance can reach **7% of global annual turnover** – billions for top banks.
- ❌ **Cost of manual compliance:** Banks spend **$2.5M – $5M per year** on AI compliance reports alone.

Regulators demand **accountability**. Enterprises demand **performance**. OVIS gives you both.

## The Solution

OVIS transforms legal requirements into **cryptographically enforced constraints** that run inside a hardware‑protected Trusted Execution Environment (TEE).

**Active blocking, not passive logging.** OVIS is the only solution that **BLOCKS illegal outputs before they leave the secure perimeter** – not just records them after the fact.

### How it works (30‑second pitch)

1. **Compliance as Code** – Write rules in simple YAML (e.g., *"interest rate ≤ 20%"*, or *"gender must have zero influence"*).  
2. **Compiler** – Translates YAML → WASM binary. The binary hash becomes an immutable **policy fingerprint**.  
3. **Active Firewall** – Inside the TEE, every model output is checked against the WASM rule.  
4. **Logical Lock** – No result leaves the TEE until the hardware **Sentinel** signs its hash. Violations are **blocked** in real time.  
5. **Zero‑Knowledge Audits** – For every blocked violation, OVIS generates a compact proof (e.g., *"rate 22% > limit 20%"*) that reveals nothing else.  
6. **Regulator Dashboard** – Aggregated, tamper‑proof logs with cryptographic proof of compliance.

## OVIS Sandbox – Demo Prototype

This repository contains a **complete, runnable prototype** that demonstrates the entire workflow:

- YAML policy compilation to WASM
- Virtual TEE (simulated) executing the policy
- Mock hardware Sentinel signing transactions
- Logical locking (encrypt‑until‑signed)
- Red‑zone violation proofs
- CLI dashboard with proof verification

> **No GPU or special hardware required** – runs on any laptop or in Docker.

## Getting Started

### Requirements
- Docker (recommended) or Rust (1.75+)

### Run with Docker (easiest)

```bash
git clone https://github.com/ovis-labs/ovis-sandbox.git
cd ovis-sandbox
docker build -t ovis-sandbox .
docker run ovis-sandbox
```

### Build from source

```bash
cargo build --release
./target/release/ovis-sandbox
```

## What you will see

```
✅ Compiled policy. Hash: 0x82f1a4b6...
🟢 GREEN transaction: 18.0% interest rate (max=25.0%) – SIGNED
🔄 Policy updated (max=20.0%). New hash: 0x9d3c...
🔴 RED transaction: 22.0% interest rate – BLOCKED
   Violation proof: { rule_id: "R003_usury", threshold: 20.0, actual_value: 22.0 }
🔍 Verifying violation proof: ✅ VALID
📋 Dashboard: All transactions logged with Sentinel signatures.
```

This small demo already proves the **three core properties** of OVIS:

1. **Integrity** – Policy hash commits to exact rules.  
2. **Active enforcement** – Violations are blocked, not just logged.  
3. **Verifiability** – Proofs can be checked by anyone (regulator, court, customer).

## Regulatory Compliance Made Concrete

**Equal Credit Opportunity Act (ECOA) / EU AI Act – Proving zero bias**

OVIS goes beyond simple output checks. It can mathematically prove that **protected attributes (race, gender, age) have zero influence** on a model's decision:

- The TEE extracts the contribution of protected inputs to the final output (e.g., integrated gradients).
- A ZK proof demonstrates that the sum of absolute influences is below a statistical noise threshold.
- Regulator receives **cryptographic guarantee** that discrimination did not happen – without seeing the model's weights.

## Architecture Overview

For the full diagram, see [`docs/architecture.png`](docs/architecture.png).  
**Color legend for your pitch deck:**
- 🔵 **Blue** – OVIS compiler & policy language (your IP)
- 🟢 **Green** – Trusted Execution Environment (NVIDIA H100 CC, Intel TDX)
- 🔴 **Red** – Hardware Sentinel (FPGA signing oracle)
- 🟡 **Gold** – Regulator dashboard & audit tools

## Roadmap to Production

| Phase | Deliverables | Timeline |
|-------|--------------|----------|
| **Pre‑Seed (now)** | OVIS Sandbox – working prototype with mock ZK | Q2 2026 |
| **Seed** | Real ZK proofs (Plonky2/Halo2); integration with NVIDIA H100 CC attestation | Q3‑Q4 2026 |
| **Series A** | Hardware Sentinel on FPGA; support for vLLM/Triton; bank pilot | 2027 |
| **Series B** | Open Verifiable Inference Standard (OVIS); regulator consortium | 2028 |

## Why It’s a Defensible Moat

- **First‑mover in “cryptographic compliance”** – no competitor offers active ZK‑blocking on model outputs.
- **Hardware agnostic** – runs on any TEE (NVIDIA CC, Intel TDX, AWS Nitro).
- **Policy language abstraction** – lawyers write rules, not cryptographers.
- **Open standard** – Apache 2.0 license ensures adoption; we become the “Visa for AI trust”.
- **Economic justification:** Banks spend $2.5‑5M/year on manual AI compliance. OVIS automates it for a fraction of that cost, while reducing regulatory fine risk by orders of magnitude.

## Use Cases

| Industry | Application | Business Impact |
|----------|-------------|------------------|
| **FinTech** | Credit scoring, loan approval | Prove non‑discrimination (ECOA, AI Act). Avoid fines up to **7% of global turnover**. |
| **HealthTech** | Diagnostic AI, insurance underwriting | Block decisions based on genetic or protected data; comply with GDPR. |
| **AdTech** | Targeted advertising | Prove no exploitation of vulnerable groups (children, addiction). |
| **HR Tech** | Resume screening, hiring algorithms | Guarantee fairness across demographics; defend against bias lawsuits. |

## Contributing

OVIS is in early stage. We welcome feedback, security audits, and pilot partnerships.  
Reach out: **totalprotocol@proton.me**

## License

Apache 2.0 – free for academic and commercial use. Open standard forever.

---

**Built with ❤️ to make AI accountable.**  
*For investors: ask us for the live demo or the full pitch deck.*
```

