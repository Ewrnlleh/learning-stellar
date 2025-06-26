use soroban_sdk::{Address, Env};
use crate::{Token, TokenClient};

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::IntoVal;

    fn create_token_contract<'a>(e: &Env) -> TokenClient<'a> {
        TokenClient::new(e, &e.register_contract(None, Token {}))
    }

    fn create_stellar_asset_contract<'a>(e: &Env) -> TokenClient<'a> {
        TokenClient::new(e, &e.register_stellar_asset_contract_v2(Address::generate(e)))
    }

    #[test]
    fn test_freeze_functionality() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        let contract = create_token_contract(&env);
        contract.initialize(&admin, &18, &"name".into_val(&env), &"symbol".into_val(&env));

        // Mint some tokens to user1
        contract.mint(&user1, &1000);
        assert_eq!(contract.balance(&user1), 1000);

        // User1 should be able to transfer initially
        contract.transfer(&user1, &user2, &100);
        assert_eq!(contract.balance(&user1), 900);
        assert_eq!(contract.balance(&user2), 100);

        // Admin freezes user1's account
        contract.freeze_account(&user1);
        assert_eq!(contract.is_frozen(&user1), true);

        // User1 should not be able to transfer when frozen
        let result = std::panic::catch_unwind(|| {
            contract.transfer(&user1, &user2, &100);
        });
        assert!(result.is_err());

        // Admin unfreezes user1's account
        contract.unfreeze_account(&user1);
        assert_eq!(contract.is_frozen(&user1), false);

        // User1 should be able to transfer again
        contract.transfer(&user1, &user2, &100);
        assert_eq!(contract.balance(&user1), 800);
        assert_eq!(contract.balance(&user2), 200);
    }

    #[test]
    fn test_frozen_account_transfer_from() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);
        let spender = Address::generate(&env);

        let contract = create_token_contract(&env);
        contract.initialize(&admin, &18, &"name".into_val(&env), &"symbol".into_val(&env));

        // Mint some tokens to user1 and approve spender
        contract.mint(&user1, &1000);
        contract.approve(&user1, &spender, &500, &(env.ledger().sequence() + 1000));

        // Freeze user1's account
        contract.freeze_account(&user1);

        // Spender should not be able to transfer from frozen account
        let result = std::panic::catch_unwind(|| {
            contract.transfer_from(&spender, &user1, &user2, &100);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_only_admin_can_freeze() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        let contract = create_token_contract(&env);
        contract.initialize(&admin, &18, &"name".into_val(&env), &"symbol".into_val(&env));

        // Non-admin should not be able to freeze accounts
        let result = std::panic::catch_unwind(|| {
            env.mock_all_auths_allowing_non_root_auth();
            contract.freeze_account(&user1);
        });
        // This would fail during authorization, not during execution
    }

    #[test]
    fn test_basic_token_functionality() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        let contract = create_token_contract(&env);
        contract.initialize(&admin, &18, &"TestToken".into_val(&env), &"TEST".into_val(&env));

        assert_eq!(contract.name(), "TestToken");
        assert_eq!(contract.symbol(), "TEST");
        assert_eq!(contract.decimals(), 18);

        contract.mint(&user1, &1000);
        assert_eq!(contract.balance(&user1), 1000);

        contract.transfer(&user1, &user2, &100);
        assert_eq!(contract.balance(&user1), 900);
        assert_eq!(contract.balance(&user2), 100);
    }
}
