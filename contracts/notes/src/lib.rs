#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct TreasuryProposal {
    pub id: u64,
    pub recipient: Address,
    pub amount: i128,
    pub purpose: String,
    pub approvals: Vec<Address>,
    pub is_executed: bool,
}

const PROPOSAL_KEY: Symbol = symbol_short!("PROP_KEY");

#[contract]
pub struct KasKitaContract;

#[contractimpl]
impl KasKitaContract {
    pub fn get_proposals(env: Env) -> Vec<TreasuryProposal> {
        env.storage().instance().get(&PROPOSAL_KEY).unwrap_or(Vec::new(&env))
    }

    pub fn propose_expense(env: Env, proposer: Address, recipient: Address, amount: i128, purpose: String) -> u64 {
        proposer.require_auth();

        let mut proposals: Vec<TreasuryProposal> = env.storage().instance().get(&PROPOSAL_KEY).unwrap_or(Vec::new(&env));
        let id = env.prng().gen::<u64>();
        let new_proposal = TreasuryProposal {
            id,
            recipient,
            amount,
            purpose,
            approvals: Vec::new(&env),
            is_executed: false,
        };

        proposals.push_back(new_proposal);
        env.storage().instance().set(&PROPOSAL_KEY, &proposals);

        id
    }

    pub fn approve_proposal(env: Env, approver: Address, id: u64) -> String {
        approver.require_auth();

        let mut proposals: Vec<TreasuryProposal> = env.storage().instance().get(&PROPOSAL_KEY).unwrap_or(Vec::new(&env));
        let mut found = false;

        for i in 0..proposals.len() {
            let mut prop = proposals.get(i).unwrap();
            if prop.id == id && !prop.is_executed {
                // Ensure voter hasn't already approved
                let mut already_voted = false;
                for v in 0..prop.approvals.len() {
                    if prop.approvals.get(v).unwrap() == approver {
                        already_voted = true;
                        break;
                    }
                }
                if !already_voted {
                    prop.approvals.push_back(approver.clone());
                    proposals.set(i, prop);
                    env.storage().instance().set(&PROPOSAL_KEY, &proposals);
                    found = true;
                }
                break;
            }
        }

        if found {
            String::from_str(&env, "Proposal approved successfully")
        } else {
            String::from_str(&env, "Proposal not found, already executed, or already approved")
        }
    }

    pub fn execute_proposal(env: Env, executor: Address, id: u64, min_approvals: u32) -> String {
        executor.require_auth();

        let mut proposals: Vec<TreasuryProposal> = env.storage().instance().get(&PROPOSAL_KEY).unwrap_or(Vec::new(&env));
        let mut found = false;

        for i in 0..proposals.len() {
            let mut prop = proposals.get(i).unwrap();
            if prop.id == id && !prop.is_executed {
                if prop.approvals.len() >= min_approvals {
                    prop.is_executed = true;
                    proposals.set(i, prop);
                    env.storage().instance().set(&PROPOSAL_KEY, &proposals);
                    found = true;
                }
                break;
            }
        }

        if found {
            String::from_str(&env, "Proposal executed successfully")
        } else {
            String::from_str(&env, "Proposal cannot be executed (insufficient approvals)")
        }
    }
}

mod test;
