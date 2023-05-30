#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Events, vec, Env, IntoVal, testutils::Address as _, Address};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EventPublisher);
    let client = EventPublisherClient::new(&env, &contract_id);

    let user = Address::random(&env);
    client.publish_event(&user.clone(), &1_i128);

    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                contract_id.clone(),
                (Symbol::short("withdraw"), user.clone()).into_val(&env),
                1_i128.into_val(&env)
            )
        ]
    );
}