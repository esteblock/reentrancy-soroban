#![cfg(test)]

use crate::{VulnerableBankClient, VulnerableBank};
use super::token::Client as TokenClient;

mod event_publisher {
    soroban_sdk::contractimport!(
        file = "../event_publisher/target/wasm32-unknown-unknown/release/event_publisher.wasm"
    );
}


use soroban_sdk::{testutils::Events, 
                vec, 
                Env, 
                IntoVal, 
                testutils::Address as _,
                Address,
                Symbol};


fn create_vulnerable_bank_contract<'a>(
    e: &Env,
    token: &Address,
) -> VulnerableBankClient<'a> {
    let vulnerable_bank = VulnerableBankClient::new(e, &e.register_contract(None, VulnerableBank {}));
    vulnerable_bank.initialize(token);
    vulnerable_bank
}

struct VulnerableBankTest<'a> {
    e: Env,
    user1: Address,
    user2: Address,
    token: TokenClient<'a>,
    vulnerable_bank: VulnerableBankClient<'a>,
    event_publisher_address: Address
}


impl<'a> VulnerableBankTest<'a> {
    fn setup() -> Self {
        let e: Env = soroban_sdk::Env::default();
        e.mock_all_auths();

        let user1 = Address::random(&e);
        let user2 = Address::random(&e);

        // Create the token contract
        let token_admin = Address::random(&e);
        let contract_token = e.register_stellar_asset_contract(token_admin.clone());
        let token = TokenClient::new(&e, &contract_token);
        
        // Mint some tokens to work with
        token.mint(&user1, &100);
        token.mint(&user2, &50);

        // Create the event_publisher contract
        let event_publisher_address = e.register_contract_wasm(None, event_publisher::WASM);


        let vulnerable_bank = create_vulnerable_bank_contract(
            &e,
            &token.address,
        );
        
        VulnerableBankTest {
            e,
            user1,
            user2,
            token,
            vulnerable_bank,
            event_publisher_address
        }

    }
}

#[test]
fn test_success() {
    //let env = Env::default();
    let setup = VulnerableBankTest::setup(); 

    // Test original balances
    assert_eq!(setup.token.balance(&setup.user1), 100);
    assert_eq!(setup.token.balance(&setup.user2), 50);
    assert_eq!(setup.token.balance(&setup.vulnerable_bank.address), 0);
    assert_eq!(setup.vulnerable_bank.balance(&setup.user1), 0);
    assert_eq!(setup.vulnerable_bank.balance(&setup.user2), 0);


    //Every user deposit all its tokens
    setup.vulnerable_bank.deposit(&setup.user1, &100);
    setup.vulnerable_bank.deposit(&setup.user2, &50);

    assert_eq!(setup.token.balance(&setup.user1), 0);
    assert_eq!(setup.token.balance(&setup.user2), 0);
    assert_eq!(setup.token.balance(&setup.vulnerable_bank.address), 150);
    assert_eq!(setup.vulnerable_bank.balance(&setup.user1), 100);
    assert_eq!(setup.vulnerable_bank.balance(&setup.user2), 50);

    // Test withdraw using the event_publisher:

    setup.vulnerable_bank.withdraw(&setup.user1, &80, &setup.event_publisher_address);
    
    assert_eq!(setup.token.balance(&setup.user1), 80);
    assert_eq!(setup.token.balance(&setup.user2), 0);
    assert_eq!(setup.token.balance(&setup.vulnerable_bank.address), 70);
    assert_eq!(setup.vulnerable_bank.balance(&setup.user1), 20);
    assert_eq!(setup.vulnerable_bank.balance(&setup.user2), 50);

    let events = setup.e.events().all();
    let second_to_last_event_vec = vec![&setup.e, events.get(events.len() - 2).unwrap().unwrap()];
    assert_eq!(
        second_to_last_event_vec,
        vec![
            &setup.e,
            (
                    setup.event_publisher_address.clone(),
                    (Symbol::short("withdraw"), &setup.user1.clone()).into_val(&setup.e),
                    80_i128.into_val(&setup.e)
            )
            
        ]
    );
}

// 