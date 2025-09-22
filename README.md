# ALETHEIA PROVER NETWORK FOR OPTIMISTIC ROLLUPS  
**_"Don’t just assume, verify."_**

> Why should we wait seven days if we can confirm immediately?

---

## 📑 Table of Contents  

- [📌 Problem Statement](#-problem-statement)  
  - [⚠️ Challenges](#️-challenges)  
- [📝 Problem Summary](#-problem-summary)  
- [✅ Solution](#-solution)  
- [🏗️ High-Level Design](#️-high-level-design)  
  - [🔹 Sequencing Layer](#-sequencing-layer)  
  - [🔹 Prover Network](#-prover-network)  
  - [🔹 L1 Verifier Contracts](#-l1-verifier-contracts)  
- [🚀 zkVerify Integration (Horizen Labs)](#-zkverify-integration-horizen-labs)  
  - [🔹 Why zkVerify?](#-why-zkverify)  
  - [🔹 Our Approach with zkVerify](#-our-approach-with-zkverify)  
- [🔎 Understanding Optimistic Rollups](#-understanding-optimistic-rollups)  
- [🛡️ Roles of Ethereum L1 for Rollups](#️-roles-of-ethereum-l1-for-rollups)  
- [⚙️ How Optimistic Rollups Build Blocks](#️-how-optimistic-rollups-build-blocks)  
- [📚 Key Study Areas](#-key-study-areas)  
- [🧩 zkVM Choice](#-zkvm-choice)  
- [📌 Note on Competition](#-note-on-competition)  
- [🔬 Active Research](#-active-research)  
- [📖 Project Summary](#-project-summary)  
- [▶️ How to Run the Project](#️-how-to-run-the-project)  
- [📊 System Architecture (Diagram)](#-system-architecture-diagram)  
-  [🚀 Roadmap](#-roadmap)
- [📜 License](#-license)  
- [🤝 Contributing](#-contributing)  

---

## 📌 Problem Statement  

Optimistic rollups are powerful for scaling Ethereum, but they come with tradeoffs and challenges.  

### ⚠️ Challenges  

- **Fraud proofs & challenge period**  
  - Optimistic rollups rely on a **7-day challenge period** where anyone can submit a fraud proof if a batch is incorrect.  
- **Centralization risk with sequencer**  
  - Many rollups rely on a **single sequencer** (a central operator) to order transactions.  
- **Capital inefficiency**  
  - Protocols built on optimistic rollups (DEXes, lending, bridges) may struggle with **capital locked for 7 days**.  
- **Computation overhead**  
  - Fraud proofs require **re-executing transactions** to verify correctness, which is resource-heavy.  

---

## 📝 Problem Summary  

Optimistic rollups solve **Ethereum scalability** but introduce:  

- Long withdrawal times  
- Reliance on fraud proofs  
- Centralization risks  

---

## ✅ Solution  

We propose a **Decentralized Prover Network** so L2 blocks can be directly proven, enabling:  

- **Faster finality times**  
- **Improved security**  
- **Decentralized trust minimization**  

---

## 🏗️ High-Level Design  

### 🔹 Sequencing Layer  
- Orders L2 transactions into batches  
- Builds an L2 block and publishes it  
- Outputs a **witness commitment** (everything provers need)  

### 🔹 Prover Network  
- Many independent provers fetch DA + witnesses  
- Execute the block in a **zkEVM** (or custom zkVM)  
- Produce **validity proofs**  

### 🔹 L1 Verifier Contracts  
- Minimal verifier contract on Ethereum (SNARK/STARK verifier)  
- Checks proofs and finalizes batches  

---

## 🚀 zkVerify Integration (Horizen Labs)  

We use **zkVerify** as the backbone of our proof verification system.  

### 🔹 Why zkVerify?  
- Purpose-built for **verifiable computation**  
- Offers a **modular and decentralized proof verification marketplace**  
- Enables **onchain verification at minimal cost**  
- Supports **multiple proof systems** (e.g., SNARKs, STARKs)  

### 🔹 Our Approach with zkVerify  
1. **Provers generate proofs** for L2 execution.  
2. Proofs are submitted to the **zkVerify network**.  
3. zkVerify validates proofs off-chain in a decentralized manner.  
4. A **lightweight verifier contract** on Ethereum L1 accepts the verification outcome.  

This removes the need for heavy fraud-proof re-execution and enables **instant finality**.  

---

## 🔎 Understanding Optimistic Rollups  

Optimistic rollups use **calldata, blobs, or external DA layers (e.g., Celestia)** for data availability.  

They function as offchain execution environments that:  

- Execute transactions **offchain (cheaper & faster)**  
- Post transaction data + state roots to Ethereum L1  
- Rely on **fraud proofs** to ensure correctness (optimistic assumption of validity)  
- Use Ethereum as the **settlement and security layer**  

---

## 🛡️ Roles of Ethereum L1 for Rollups  

- **Data Availability (DA)**  
  - Rollups must publish transaction data so anyone can reconstruct state.  
- **Settlement & Finality**  
  - Rollup state roots are periodically posted to Ethereum.  
  - Once the challenge period closes, state is finalized.  
- **Security via Fraud Proofs**  
  - If an invalid state root is posted, fraud proofs ensure correction.  

---

## ⚙️ How Optimistic Rollups Build Blocks  

From first principles:  

1. Sequencer orders transactions → produces block  
2. Posts calldata + root to Ethereum  
3. Challenge window opens  
4. If fraud proof is submitted, Ethereum verifies & reverts if needed  

⚠️ **This process introduces long delays & inefficiency.**  

Our **zkVerify-backed prover network** eliminates this by enabling **direct proof-based verification**.  

---

## 📚 Key Study Areas  

- Compare **Optimistic Rollups vs ZK Rollups**  
- Analyze fraud-proof vs validity-proof security models  
- Explore **zkVM-based execution models**  

---

## 🧩 zkVM Choice  

We utilize **zkVerify**’s ecosystem for verifiable computation because:  

- It provides **plug-and-play verifiers**  
- Reduces **onchain verification cost**  
- Supports **scalable proof systems**  
- Aligns with our mission of **faster, trust-minimized rollups**  

---

## 📌 Note on Competition  

⚠️ At the time of writing, we have not seen any **production-ready decentralized prover networks** that leverage zkVerify in this manner.  

This positions **Aletheia Prover Network** as a **pioneering infrastructure project**.  

---

## 🔬 Active Research  

- Deep dive into **optimistic rollup implementations** (at the codebase level)  
- Exploration of **zk rollup architectures** for hybrid models  
- Benchmarking **zkVerify integration performance**  

---

## 📖 Project Summary  

- **Problem**: Rollups today suffer from long withdrawal times, fraud proofs, and centralization.  
- **Solution**: Decentralized Prover Network powered by zkVerify for direct proof validation.  
- **Impact**: Faster finality, improved security, and decentralized verification marketplace.  

---


## 📊 System Architecture (Diagram)

                                    ALETHEIA PROVER NETWORK ARCHITECTURE
                                    ===================================

            ┌─────────────────┐         ┌──────────────────┐         ┌─────────────────────┐
            │ ROLLUP          │         │ ALETHEIA         │         │ zkVERIFY +          │
            │ SEQUENCER       │────────▶│ PROVER NETWORK   │────────▶│ ETHEREUM L1         │
            │                 │         │                  │         │                     │
            ├─────────────────┤         ├──────────────────┤         ├─────────────────────┤
            │• Execute txs    │         │• Extract data    │         │• Aggregate proofs   │
            │• Build blocks   │         │• Generate proofs │         │• Verify on L1       │
            │• Post to DA     │         │• Verify state    │         │• Instant finality   │
            │• Output witness │         │• Compete for     │         │• No challenge       │
            │  data           │         │  rewards         │         │  period needed      │
            └─────────────────┘         └──────────────────┘         └─────────────────────┘
                    │                           │                             │
                    │                           │                             │
            ┌────▼───┐                  ┌────▼───┐                   ┌────▼───┐
            │ Block  │                  │ Proof  │                   │ Final  │
            │ Data   │                  │ Gen    │                   │ State  │
            └────────┘                  └────────┘                   └────────┘

                    ▼                           ▼                             ▼
            ┌─────────────────────────────────────────────────────────────────────────────┐
            │                         DATA AVAILABILITY LAYER                             │
            │                   (Calldata / Blobs / Celestia / EigenDA)                   │
            └─────────────────────────────────────────────────────────────────────────────┘

                                        DATA FLOW DETAIL
                                        ================

            ┌─────────────────┐    ┌──────────────────┐    ┌─────────────────────┐
            │ 1. EXTRACTION   │───▶│ 2. PROVING       │───▶│ 3. VERIFICATION     │
            │                 │    │                  │    │                     │
            │                 │    │ • zkVM execution │    │ • Proof aggregation │
            │ • Block data    │    │ • Circuit logic  │    │ • L1 settlement     │
            │ • Account state │    │ • RISC Zero      │    │ • zkVerify          │
            │ • Merkle proofs │    │ • State checks   │    │ • Gas optimization  │
            └─────────────────┘    └──────────────────┘    └─────────────────────┘

                                    PROVER NETWORK DETAIL
                                    =====================

                                    ┌─────────────────────┐
                                    │                     │
                                    │                     │
                                    │       Aletheia      │
                                    │                     │
                                    │                     │
                                    └─────────────────────┘
                                                │
                        ┌───────────────────────┼───────────────────────┐
                        │                       │                       │
                        ▼                       ▼                       ▼
            ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
            │ PROVER NODE 1   │    │ PROVER NODE 2   │    │ PROVER NODE 3   │
            │                 │    │                 │    │                 │
            │ • Data fetcher  │    │ • Data fetcher  │    │ • Data fetcher  │
            │ • zkVM runner   │    │ • zkVM runner   │    │ • zkVM runner   │
            │ • Proof gen     │    │ • Proof gen     │    │ • Proof gen     │
            │ • Stake & earn  │    │ • Stake & earn  │    │ • Stake & earn  │
            └─────────────────┘    └─────────────────┘    └─────────────────┘



---

## ▶️ How to Run the Project  

### Prerequisites  
- Node.js >= 18  
- Rust
- pnpm or npm  

### Installation  

To test and run the **Aletheia Prover Network**, we recommend using a **local Optimistic Rollup setup**.  
👉 Follow the official Optimism docs: [Create an L2 Rollup](https://docs.optimism.io/operators/chain-operators/tutorials/create-l2-rollup)  

This avoids rate limits and disabled RPC methods from deployed rollups (e.g., Base, OP Mainnet).  


```bash
# Clone the repo
git clone https://github.com/Aletheia-Prover-Network/Aletheia.git

cd Aletheia

### 🚀 Run the Modules  

#### 🔹 `data_extractor` (Rust)  
Extracts L2 block data & witnesses.  

```bash
cd data_extractor
cargo run

cd aletheia
RISC0_DEV_MODE=0 cargo run --release

cd proof_verification
node index.js -- for zkverify relayer
node app.js -- for zkverifyjs



```

## 🚀 Roadmap

## 🗺️ Roadmap  

### Phase 1 — Research & Prototyping ✅  
- Study Optimistic Rollups and zkRollups at a codebase level  
- Build core modules (`data_extractor`, `aletheia_proof_generation`, `proof_verification`)  
- Integrate with **zkVerify** for proof validation  

### Phase 2 — Local Rollup Testing 🧪  
- Run against a **local Optimistic Rollup** environment  
- Benchmark proof generation & verification performance  
- Ensure seamless end-to-end flow (extract → prove → verify)  

### Phase 3 — Network Integration 🌐  
- Connect to public L2 testnets (Optimism, Base, Scroll)  
- Implement lightweight Ethereum L1 verifier contract  
- Validate proofs with **zkVerify marketplace**  

### Phase 4 — Decentralized Prover Network 🔗  
- Expand to a **distributed prover network**  
- Incentivize participation (staking/rewards model)  
- Enhance fault tolerance & prover diversity  

### Phase 5 — Production Launch 🚀  
- Deploy fully decentralized Aletheia Prover Network  
- Provide SDKs & APIs for dApps and rollups  
- Continuous monitoring, upgrades, and optimizations  


## 📜 License  

[MIT License](./LICENSE)  

---

## 🤝 Contributing  

We welcome contributions! Please open issues or submit PRs.  

---

