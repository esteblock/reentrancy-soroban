#![no_std]
use soroban_sdk::{contractimpl, Env, Symbol, Address};
pub struct EventPublisher;

#[contractimpl]
impl EventPublisher {
    pub fn publish_event(env: Env, user: Address, amount: i128) {
        let topics = (Symbol::short("withdraw"), user);
        env.events().publish(topics, amount);
    }
}

mod test;
