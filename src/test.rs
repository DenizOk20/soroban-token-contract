#![cfg(test)]
extern crate std;
use crate::{contract::Token, TokenClient};
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, IntoVal, Symbol,String
};

fn create_token<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
    let token = TokenClient::new(e, &e.register_contract(None, Token {}));
    token.initialize(admin, &7, &"name".into_val(e), &"symbol".into_val(e));
    token
}

#[test]
fn test() {
    let env = Env::default();
    env.mock_all_auths();

    let admin1 = Address::generate(&env);
    let admin2 = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let user3 = Address::generate(&env);
    let token = create_token(&env, &admin1);

    token.mint(&user1, &1000);

    assert_eq!(
        env.auths(),
        std::vec![(
            admin1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("mint"),
                    (user1.clone(), 1000_i128).into_val(&env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.balance(&user1), 1000);

    token.approve(&user2,&user3,&500,&200);

    assert_eq!(
        env.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("approve"),
                    (&user2, &user3, 500_i128, 200_u32).into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(token.allowance(&user2, &user3), 500);

    token.transfer(&user1, &user2, &600);

    assert_eq!(
        env.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("transfer"),
                    (&user1, &user2, 600_i128).into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(token.balance(&user1), 400);
    assert_eq!(token.balance(&user2), 600);

    token.transfer_from(&user3, &user2, &user1, &400);

    assert_eq!(
        env.auths(),
        std::vec![(
            user3.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    Symbol::new(&env, "transfer_from"),
                    (&user3, &user2, &user1, 400_i128).into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(token.balance(&user1), 800);
    assert_eq!(token.balance(&user2), 200);

    token.transfer(&user1, &user3, &300);
    assert_eq!(token.balance(&user1), 500);
    assert_eq!(token.balance(&user3), 300);

    token.set_admin(&admin2);

    assert_eq!(
        env.auths(),
        std::vec![(
            admin1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("set_admin"),
                    (&admin2,).into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    token.approve(&user2, &user3, &500, &200);
    assert_eq!(token.allowance(&user2, &user3), 500);
    token.approve(&user2, &user3, &0, &200);

    assert_eq!(
        env.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("approve"),
                    (&user2, &user3, 0_i128, 200_u32).into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.allowance(&user2, &user3), 0);
}

#[test]
fn test_burn() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let token = create_token(&env, &admin);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.approve(&user1, &user2, &500, &200);
    assert_eq!(token.allowance(&user1, &user2), 500);

    token.burn_from(&user2, &user1, &500);
    assert_eq!(
        env.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("burn_from"),
                    (&user2, &user1, 500_i128).into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(token.allowance(&user1, &user2), 0);
    assert_eq!(token.balance(&user1), 500);
    assert_eq!(token.balance(&user2), 0);

    token.burn(&user1, &500);
    assert_eq!(
        env.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("burn"),
                    (&user1, 500_i128).into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(token.balance(&user1), 0);
    assert_eq!(token.balance(&user2), 0);
}

#[test]
#[should_panic(expected = "insufficient balance")]
fn transfer_insufficient_balance() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let token = create_token(&env, &admin);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.transfer(&user1, &user2, &1001);
}

#[test]
#[should_panic(expected = "insufficient allowance")]
fn transfer_from_insufficient_allowance() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let user3 = Address::generate(&env);
    let token = create_token(&env, &admin);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.approve(&user1, &user3, &100, &200);
    assert_eq!(token.allowance(&user1, &user3), 100);

    token.transfer_from(&user3, &user1, &user2, &101);
}

#[test]
#[should_panic(expected = "already initialized")]
fn initialize_already_initialized() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let token = create_token(&env, &admin);

    token.initialize(&admin, &10, &"name".into_val(&env), &"symbol".into_val(&env));
}

#[test]
#[should_panic(expected = "Decimal must fit in a u8")]
fn decimal_is_over_max() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let token = TokenClient::new(&env, &env.register_contract(None, Token {}));
    token.initialize(
        &admin,
        &(u32::from(u8::MAX) + 1),
        &"name".into_val(&env),
        &"symbol".into_val(&env),
    );
}

#[test]
fn test_total_supply() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let token = TokenClient::new(&env, &env.register_contract(None, Token {}));
    
    token.initialize(
        &admin, 
        &10u32, 
        &String::from_str(&env, "Token"), 
        &String::from_str(&env, "TKN")
    );
    
    assert_eq!(token.total_supply(), 0);
}

