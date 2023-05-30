#![no_std]
use soroban_sdk::{contractimpl, contracttype, Address, Env};

mod token {
    soroban_sdk::contractimport!(file = "../token/soroban_token_contract.wasm");
}

// mod test;
// mod testutils;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Token,
    User(Address),
}


fn get_token(e: &Env) -> Address {
    e.storage()
        .get(&DataKey::Token)
        .expect("not initialized")
        .unwrap()
}

fn get_user_deposited(e: &Env, user: &Address) -> i128 {
    e.storage()
        .get(&DataKey::User(user.clone()))
        .unwrap_or(Ok(0))
        .unwrap()
}

fn set_user_deposited(e: &Env, user: &Address, amount: &i128) {
    e.storage().set(&DataKey::User(user.clone()), amount);
}

// Transfer tokens from the contract to the recipient
fn transfer(e: &Env, to: &Address, amount: &i128) {
    let token_contract_id = &get_token(e);
    let client = token::Client::new(e, token_contract_id);
    client.transfer(&e.current_contract_address(), to, amount);
}

struct VulnerableBank;


#[contractimpl]
#[allow(clippy::needless_pass_by_value)]
impl VulnerableBank {
    pub fn initialize(
        e: Env,
        token: Address,
    ) {
        e.storage().set(&DataKey::Token, &token);
    }

    pub fn token(e: Env) -> Address {
        get_token(&e)
    }

    pub fn balance(e: Env, user: Address) -> i128 {
        get_user_deposited(&e, &user)
    }

    pub fn deposit(e: Env, user: Address, amount: i128) {
        user.require_auth();
        assert!(amount > 0, "amount must be positive");

        let balance = get_user_deposited(&e, &user);
        set_user_deposited(&e, &user, &(balance + amount));

        //let client = token::Client::new(&e, &get_token(&e));
        //client.transfer(&user, &e.current_contract_address(), &amount);
    }

    pub fn withdraw(e: Env, user: Address, amount: i128) {
        assert!(amount > 0, "amount must be positive");
        let balance = get_user_deposited(&e, &user);

        assert!(balance >= amount, "balance should be greater than the amount requested");

        set_user_deposited(&e, &user, &(balance - amount));
        transfer(&e, &user, &balance);
}

}