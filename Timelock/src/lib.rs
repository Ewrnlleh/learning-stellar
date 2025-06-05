// ! This contract demonstrates 'timelock' concept and implements a
// ! greatly simplified Claimable Balance similar to
// ! https://developers.stellar.org/docs/qlossary/claimable—balance) .
// ! The contract allows to deposit some amount of token and allow another
// ! account(s) claim it before or after provided time point.
// ! For simplicity, the contract only supports invoker—based auth.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Vec};

#[derive(Clone)]
#[contracttype]

pub enum DataKey {
    Init,
    Balance,
}

#[derive(Clone)]
#[contracttype]

pub enum TimeBoundKind {
    Before,
    After,
}

#[derive(Clone)]
#[contracttype]
pub struct TimeBound {
    pub kind: TimeBoundKind,
    pub timeStamp: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct ClaimableBalance {
    pub token: Address,
    pub amount: i128,
    pub claimants: Vec<Address>,
    pub time_bound: TimeBound,
}

#[contract]
pub struct ClaimableBalanceContract;

// The 'timelock' part: check that provided timestamp is before/after
// the current ledger timestamp.
fn check_time_bound(e: &Env, time_bound: &TimeBound) -> bool {
    let ledger_timestamp: u64 = e.ledger().timestamp();

    match time_bound.kind {
        TimeBoundKind::Before => ledger_timestamp < time_bound.timeStamp,
        TimeBoundKind::After => ledger_timestamp > time_bound.timeStamp,
    }
}


#[contractimpl]
impl ClaimableBalanceContract {
    pub fn deposit(
        env: Env,
        from: Address,
        token: Address,
        amount: i128,
        claimants: Vec<Address>,
        time_bound: TimeBound,
        ) {
            if claimants.len() > 10 {
                panic!("Too many claimants, max 10 allowed");
            }

            if is_initialized(&env) {
                panic!("Contract already initialized");
            }
            // Make sure 'from' address authorized the deposit call with all the
            // arguments.
            from.require_auth();
            
            // Transfer token from *from' to this contract address.
            token::Client::new(&env, &token).transfer(&from, &env.current_contract_address(), &amount);
            // Store all the necessary info to allow one of the claimants to claim it.
            env.storage().instance().set(
                &DataKey::Balance,
                &ClaimableBalance {
                    token,
                    amount,
                    time_bound,
                    claimants,
                },
            );
            // Mark contract as initialized to prevent double—usage.
            // Note, that this is just one way to approach initialization - it may
            // be viable to allow one contract to manage several claimable balances.
            env.storage().instance().set(&DataKey::Init, &());
        }

    pub fn claim(env: Env, claimant: Address) {
        // Make sure claimant has authorized this call, which ensures their
        // identity.
        let claimable_balance: ClaimableBalance = env
            .storage()
            .instance()
            .get(&DataKey::Balance)
            .unwrap();

        if !check_time_bound(&env, &claimable_balance.time_bound) {
            panic!("Claim time bound not satisfied");
        }

        let claimants: &Vec<Address> = &claimable_balance.claimants;
        if !claimants.contains(&claimant) {
            panic!("Claimant not authorized to claim this balance");
        }
       
        // Transfer the stored amount of token to claimant after passing
        // all the checks.
        token::Client::new(&env, &claimable_balance.token).transfer(
            &env.current_contract_address(),
            &claimant,
            &claimable_balance.amount,
        );

        // Remove the balance entry to prevent any further claims.
        env.storage().instance().remove(&DataKey::Balance);
    }

  

}

  fn is_initialized(env: &Env) -> bool {
        env.storage().instance().has(&DataKey::Init)
    }

// ...existing code...



#[cfg(test)]
mod test {
    // Buraya test fonksiyonlarını yazabilirsin
}

// ...existing code...