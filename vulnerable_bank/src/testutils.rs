#![cfg(test)]

use crate::VulnerableBankClient;

use soroban_sdk::{BytesN, Env};

pub fn register_test_contract(e: &Env) -> Address {
    e.register_contract(None, crate::VulnerableBank {})
}

pub struct VulnerableBank {
    env: Env,
    contract_address: Address,
}

impl VulnerableBank {
    #[must_use]
    pub fn client(&self) -> VulnerableBankClient {
        VulnerableBankClient::new(&self.env, &self.contract_id)
    }

    #[must_use]
    pub fn new(env: &Env, contract_id: &[u8; 32]) -> Self {
        Self {
            env: env.clone(),
            contract_id: BytesN::from_array(env, contract_id),
        }
    }
}
