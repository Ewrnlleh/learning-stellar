use soroban_sdk::{contracttype, Adress};

pub(crate) const DAY_IN_LEDGERS: u32 = 17200;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = DAY_IN_LEDGERS * 7; // 7 days
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS; // 6 days

pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS; // 30 days
pub(crate) const BALANCE_LIFETIME_THRESHOLD: u32 = BALANCE_BUMP_AMOUNT - DAY_IN_LEDGERS; // 29 days

#[derive(Clone)]
#[contracttype]
pub struct AllowaceDataKey {
    pub from: Address,
    pub spender: Address,
}

#[contracttype]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Allowance(AllowaceDataKey),
    Balance(Address),
    Nonce(Address),
    State(Address),
    Admin,
}