#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Events, vec, Env, IntoVal, testutils::Address as _, Address};

mod token {
    soroban_sdk::contractimport!(file = "../token/soroban_token_contract.wasm");
}

mod event_publisher {
    soroban_sdk::contractimport!(
        file = "../event_publisher/target/wasm32-unknown-unknown/release/event_publisher.wasm"
    );
}

mod vulnerable_bank {
    soroban_sdk::contractimport!(
        file = "../vulnerable_bank/target/wasm32-unknown-unknown/release/vulnerable_bank.wasm"
    );
}

use vulnerable_bank::Client as VulnerableBankClient;
use token::Client as TokenClient;




#[test]
fn test() {
    /*
    
    NOTE: This test will don't do any reentrancy
    
    */
    let env = Env::default();
    env.mock_all_auths();

    // Create the event_publisher contract
    let event_publisher_address = env.register_contract_wasm(None, event_publisher::WASM);

    // Create the vulnerable_bank contract
    let vulnerable_bank_address = env.register_contract_wasm(None, vulnerable_bank::WASM);
    let vulnerable_bank_client = VulnerableBankClient::new(&env, &vulnerable_bank_address);

    // Create the evil_event_publisher contract
    let evil_address = env.register_contract(None, EvilEventPublisher);
    let evil = EvilEventPublisherClient::new(&env, &evil_address);

    // Initialize the evil token
    evil.initialize(&vulnerable_bank_address, &event_publisher_address);

    // Create the token contract
    let token_admin = Address::random(&env);
    let token_address = env.register_stellar_asset_contract(token_admin.clone());
    let token = TokenClient::new(&env, &token_address);
    

    // Initialize the bank
    vulnerable_bank_client.initialize(&token_address);

    // Mint some tokens to work with
    let user1 = Address::random(&env);
    let user2 = Address::random(&env);

    token.mint(&user1, &100);
    token.mint(&user2, &50);
    assert_eq!(token.balance(&user1), 100);
    assert_eq!(token.balance(&user2), 50);

    // Deposit directly to the bank
    vulnerable_bank_client.deposit(&user1, &100);
    vulnerable_bank_client.deposit(&user2, &50);
    assert_eq!(token.balance(&user1), 0);
    assert_eq!(token.balance(&user2), 0);
    assert_eq!(token.balance(&vulnerable_bank_address), 150);
    assert_eq!(vulnerable_bank_client.balance(&user1), 100);
    assert_eq!(vulnerable_bank_client.balance(&user2), 50);

    // Withdraw using the evil contract, not yet doing reentrancy:
    evil.publish_withdraw(&user1, &10);
    assert_eq!(token.balance(&user1), 10);
    assert_eq!(token.balance(&user2), 0);
    assert_eq!(token.balance(&vulnerable_bank_address), 140);
    assert_eq!(vulnerable_bank_client.balance(&user1), 90);
    assert_eq!(vulnerable_bank_client.balance(&user2), 50);


    // Withdraw directly from the bank, not yet doing reentrancy
    vulnerable_bank_client.withdraw(&user1, &40, &event_publisher_address);
    assert_eq!(token.balance(&user1), 50);
    assert_eq!(token.balance(&user2), 0);
    assert_eq!(token.balance(&vulnerable_bank_address), 100);
    assert_eq!(vulnerable_bank_client.balance(&user1), 50);
    assert_eq!(vulnerable_bank_client.balance(&user2), 50);


}

#[test]
#[should_panic(expected = "")]
fn test_reentrancy_should_panic() {
    let env = Env::default();
    env.mock_all_auths();

    // Create the event_publisher contract
    let event_publisher_address = env.register_contract_wasm(None, event_publisher::WASM);

    // Create the vulnerable_bank contract
    let vulnerable_bank_address = env.register_contract_wasm(None, vulnerable_bank::WASM);
    let vulnerable_bank_client = VulnerableBankClient::new(&env, &vulnerable_bank_address);

    // Create the evil_event_publisher contract
    let evil_address = env.register_contract(None, EvilEventPublisher);
    let evil = EvilEventPublisherClient::new(&env, &evil_address);

    // Initialize the evil token
    evil.initialize(&vulnerable_bank_address, &event_publisher_address);

    // Create the token contract
    let token_admin = Address::random(&env);
    let token_address = env.register_stellar_asset_contract(token_admin.clone());
    let token = TokenClient::new(&env, &token_address);
    

    // Initialize the bank
    vulnerable_bank_client.initialize(&token_address);

    // Mint some tokens to work with
    let user1 = Address::random(&env);
    let user2 = Address::random(&env);

    token.mint(&user1, &100);
    token.mint(&user2, &50);
    assert_eq!(token.balance(&user1), 100);
    assert_eq!(token.balance(&user2), 50);

    // Deposit directly to the bank
    vulnerable_bank_client.deposit(&user1, &100);
    vulnerable_bank_client.deposit(&user2, &50);
    assert_eq!(token.balance(&user1), 0);
    assert_eq!(token.balance(&user2), 0);
    assert_eq!(token.balance(&vulnerable_bank_address), 150);
    assert_eq!(vulnerable_bank_client.balance(&user1), 100);
    assert_eq!(vulnerable_bank_client.balance(&user2), 50);

    // Withdraw using the evil contract, not yet doing reentrancy:
    evil.publish_withdraw(&user1, &10);
    assert_eq!(token.balance(&user1), 10);
    assert_eq!(token.balance(&user2), 0);
    assert_eq!(token.balance(&vulnerable_bank_address), 140);
    assert_eq!(vulnerable_bank_client.balance(&user1), 90);
    assert_eq!(vulnerable_bank_client.balance(&user2), 50);


    // Withdraw directly from the bank, not yet doing reentrancy
    vulnerable_bank_client.withdraw(&user1, &40, &event_publisher_address);
    assert_eq!(token.balance(&user1), 50);
    assert_eq!(token.balance(&user2), 0);
    assert_eq!(token.balance(&vulnerable_bank_address), 100);
    assert_eq!(vulnerable_bank_client.balance(&user1), 50);
    assert_eq!(vulnerable_bank_client.balance(&user2), 50);

    //Do the reentrancy attack!
    vulnerable_bank_client.withdraw(&user1, &50, &evil_address);

}


#[test]
fn test_reentrancy_should_panic_wanna_see_the_error() {
    let env = Env::default();
    env.mock_all_auths();

    // Create the event_publisher contract
    let event_publisher_address = env.register_contract_wasm(None, event_publisher::WASM);

    // Create the vulnerable_bank contract
    let vulnerable_bank_address = env.register_contract_wasm(None, vulnerable_bank::WASM);
    let vulnerable_bank_client = VulnerableBankClient::new(&env, &vulnerable_bank_address);

    // Create the evil_event_publisher contract
    let evil_address = env.register_contract(None, EvilEventPublisher);
    let evil = EvilEventPublisherClient::new(&env, &evil_address);

    // Initialize the evil token
    evil.initialize(&vulnerable_bank_address, &event_publisher_address);

    // Create the token contract
    let token_admin = Address::random(&env);
    let token_address = env.register_stellar_asset_contract(token_admin.clone());
    let token = TokenClient::new(&env, &token_address);
    

    // Initialize the bank
    vulnerable_bank_client.initialize(&token_address);

    // Mint some tokens to work with
    let user1 = Address::random(&env);
    let user2 = Address::random(&env);

    token.mint(&user1, &100);
    token.mint(&user2, &50);
    assert_eq!(token.balance(&user1), 100);
    assert_eq!(token.balance(&user2), 50);

    // Deposit directly to the bank
    vulnerable_bank_client.deposit(&user1, &100);
    vulnerable_bank_client.deposit(&user2, &50);
    assert_eq!(token.balance(&user1), 0);
    assert_eq!(token.balance(&user2), 0);
    assert_eq!(token.balance(&vulnerable_bank_address), 150);
    assert_eq!(vulnerable_bank_client.balance(&user1), 100);
    assert_eq!(vulnerable_bank_client.balance(&user2), 50);

    // Withdraw using the evil contract, not yet doing reentrancy:
    evil.publish_withdraw(&user1, &10);
    assert_eq!(token.balance(&user1), 10);
    assert_eq!(token.balance(&user2), 0);
    assert_eq!(token.balance(&vulnerable_bank_address), 140);
    assert_eq!(vulnerable_bank_client.balance(&user1), 90);
    assert_eq!(vulnerable_bank_client.balance(&user2), 50);


    // Withdraw directly from the bank, not yet doing reentrancy
    vulnerable_bank_client.withdraw(&user1, &40, &event_publisher_address);
    assert_eq!(token.balance(&user1), 50);
    assert_eq!(token.balance(&user2), 0);
    assert_eq!(token.balance(&vulnerable_bank_address), 100);
    assert_eq!(vulnerable_bank_client.balance(&user1), 50);
    assert_eq!(vulnerable_bank_client.balance(&user2), 50);

    //Do the reentrancy attack!
    vulnerable_bank_client.withdraw(&user1, &50, &evil_address);

}