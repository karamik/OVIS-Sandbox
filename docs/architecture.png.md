flowchart TB
    subgraph Legal & Compliance
        A[Lawyer / Compliance Officer] -->|Writes policies in YAML| B(OVIS Policy Language - OPL)
    end

    subgraph OVIS_Compiler
        B --> C[Compiler]
        C -->|Generates| D[WASM Bytecode]
        C -->|Computes| E[Policy Hash SHA3-256]
    end

    subgraph TEE_Environment [Trusted Execution Environment - NVIDIA H100 CC / Intel TDX]
        F[Model Output Tensor] --> G{OVIS Runtime inside TEE}
        D -.->|Loaded into| G
        G -->|Executes policy| H{Decision}
        H -->|Green: within bounds| I[Encrypt output with ephemeral key K_sym]
        H -->|Red: violation| J[Block output, generate violation proof]
    end

    subgraph Sentinel_Hardware [Hardware Sentinel - FPGA / PCIe Card]
        K[Sentinel] -->|Signs hash| L[Attestation Signature]
        M[(Immutable Log)] -->|Merkle Root| N[Regulator Dashboard]
    end

    I -->|Sends encrypted output + hash| K
    J -->|Sends violation proof + hash| K
    K -->|After signature, releases K_sym| O[Bank / Client Application]
    N --> P[Regulator / Auditor]

    style A fill:#f9f,stroke:#333,stroke-width:2px
    style C fill:#58f,stroke:#333,stroke-width:2px
    style G fill:#6f6,stroke:#333,stroke-width:2px
    style K fill:#f66,stroke:#333,stroke-width:2px
    style N fill:#fc6,stroke:#333,stroke-width:2px
