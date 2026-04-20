#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct MedicalRecord {
    pub id: u64,
    pub patient_name: String,
    pub record_hash: String,
    pub timestamp: u64,
}

const MED_KEY: Symbol = symbol_short!("MED_REC");

#[contract]
pub struct MedicalContract;

#[contractimpl]
impl MedicalContract {
    pub fn get_history(env: Env) -> Vec<MedicalRecord> {
        env.storage()
            .instance()
            .get(&MED_KEY)
            .unwrap_or(Vec::new(&env))
    }

    pub fn add_record(env: Env, patient_name: String, record_hash: String) -> u64 {
        let mut history: Vec<MedicalRecord> = Self::get_history(env.clone());
        
        let new_id = (history.len() as u64) + 1;
        let new_record = MedicalRecord {
            id: new_id,
            patient_name,
            record_hash,
            timestamp: env.ledger().timestamp(),
        };

        history.push_back(new_record);
        env.storage().instance().set(&MED_KEY, &history);
        
        new_id
    }

    pub fn verify_record(env: Env, record_hash: String) -> bool {
        let history: Vec<MedicalRecord> = Self::get_history(env.clone());
        for i in 0..history.len() {
            if history.get(i).unwrap().record_hash == record_hash {
                return true;
            }
        }
        false
    }
}