# KasKita: Transparent Treasury Manager

**KasKita** - A blockchain-powered community platform built on the **Stellar Network** using **Soroban Smart Contracts**. This project aims to eliminate community treasury corruption by requiring open, multi-signature approvals for organizational expenditures.

---

## 📌 Problem Statement
Opaque management of organization treasury accounts by leaders or student unions often fuels community distrust and corruption allegations.

Currently, this issue faces several hurdles:
1. **Mismanagement Risks:** Single individuals control physical bank accounts of organization funds.
2. **Lack of Audit Trail:** Paper bills and manual ledgers are easily forged or hidden.
3. **Slow Approvals:** Coordinating paper signatures delay organization project spending.

## 🚀 Urgency
Non-profit organizations, clubs, and student bodies need public accountability. An on-chain, multi-signature treasury prevents unilateral abuse of communal cash reserves.
* **Anti-Corruption by Design:** Restricts money transfers to verified multi-signature rules.
* **Real-Time Transparency:** Community members see where every penny is spent.

## ✨ Key Features
* **Multi-Signature Control:** Transfers require digital approval votes from designated committee members.
* **Immutability:** All transaction records, expense proposals, and approvals are stored permanently on-chain.
* **Auditable Ledger:** Complete transparent visibility of cash flow history.

---

## 🛠 Technical Stack
* **Blockchain:** Stellar Network
* **Smart Contract Engine:** Soroban
* **Language:** Rust
* **Development Environment:** Soroban CLI / Rust toolchain

## 📋 Smart Contract Overview
The contract handles organizational proposals, member approvals, and executions:
1. `get_proposals()`: Fetch all budget proposals created under the treasury.
2. `propose_expense(proposer, recipient, amount, purpose)`: Create a new budget spend proposal.
3. `approve_proposal(approver, id)`: Sign and approve an active budget proposal.
4. `execute_proposal(executor, id, min_approvals)`: Execute and transfer funds for a proposal that meets the approval quorum.

---

## 💡 Future Roadmap
- [ ] **Public Audit Dashboard:** Create a beautiful UI that visualizes treasury trends and details.
- [ ] **Sponsor Tracking:** Allow donors to trace which projects their specific contributions funded.
- [ ] **Governance Voting:** Integrate DAO proposals to allow general members to veto expenditures.

---
## Screenshots
<img width="1920" height="1080" alt="image" src="screenshot.png" />

---
Stellar ID: GNJ7PEX6QFUJQZIKFD4LIJCU6YKELI37CQV7EERG4VZRCWEFB4CNVT7M
*Developed for the Stellar Community and the advancement of Decentralized Social Economies.*
