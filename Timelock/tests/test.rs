#[cfg(test)]
mod tests {
    use soroban_sdk::{Env, Address};
    use soroban_sdk::testutils::{Address as _, Ledger as _};
    use crate::{ClaimableBalanceContract, TimeBound, TimeBoundKind};

    #[test]
    fn test_deposit_and_claim() {
        let env = Env::default();

        let from = Address::generate(&env);
        let token = Address::generate(&env);
        let claimant = Address::generate(&env);

        let time_bound = TimeBound {
            kind: TimeBoundKind::After,
            timestamp: env.ledger().timestamp() - 1,
        };

        ClaimableBalanceContract::deposit(
            env.clone(),
            from.clone(),
            token.clone(),
            100,
            vec![claimant.clone()],
            time_bound,
        );

        ClaimableBalanceContract::claim(env, claimant);
    }
}