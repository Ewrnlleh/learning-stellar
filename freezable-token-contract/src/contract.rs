use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, symbol_short};

#[derive(Clone)]
#[contracttype]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}

#[derive(Clone)]
#[contracttype]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Allowance(AllowanceDataKey),
    Balance(Address),
    Nonce(Address),
    State(Address),
    Admin,
    Frozen(Address), // New key for tracking frozen accounts
}

#[derive(Clone)]
#[contracttype]
pub struct TokenMetadata {
    pub decimal: u32,
    pub name: String,
    pub symbol: String,
}

fn check_nonnegative_amount(amount: i128) {
    if amount < 0 {
        panic!("negative amount is not allowed: {}", amount)
    }
}

#[contract]
pub struct Token;

#[contractimpl]
impl Token {
    pub fn initialize(
        env: Env,
        admin: Address,
        decimal: u32,
        name: String,
        symbol: String,
    ) {
        if has_administrator(&env) {
            panic!("already initialized")
        }
        write_administrator(&env, &admin);
        if decimal > 18 {
            panic!("decimal must not be greater than 18");
        }
        write_metadata(&env, TokenMetadata { decimal, name, symbol })
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        check_nonnegative_amount(amount);
        let admin = read_administrator(&env);
        admin.require_auth();

        receive_balance(&env, to.clone(), amount);
        TokenUtils::new(&env).events().mint(admin, to, amount);
    }

    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        read_allowance(&env, from, spender).amount
    }

    pub fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        check_nonnegative_amount(amount);

        write_allowance(&env, from.clone(), spender.clone(), amount, expiration_ledger);
        TokenUtils::new(&env)
            .events()
            .approve(from, spender, amount, expiration_ledger);
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        read_balance(&env, id)
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        check_nonnegative_amount(amount);
        
        // Check if the from account is frozen
        if is_account_frozen(&env, &from) {
            panic!("account is frozen and cannot transfer tokens");
        }

        spend_balance(&env, from.clone(), amount);
        receive_balance(&env, to.clone(), amount);
        TokenUtils::new(&env).events().transfer(from, to, amount);
    }

    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);
        
        // Check if the from account is frozen
        if is_account_frozen(&env, &from) {
            panic!("account is frozen and cannot transfer tokens");
        }

        spend_allowance(&env, from.clone(), spender, amount);
        spend_balance(&env, from.clone(), amount);
        receive_balance(&env, to.clone(), amount);
        TokenUtils::new(&env).events().transfer(from, to, amount);
    }

    pub fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();

        check_nonnegative_amount(amount);
        spend_balance(&env, from.clone(), amount);
        TokenUtils::new(&env).events().burn(from, amount);
    }

    pub fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);
        spend_allowance(&env, from.clone(), spender, amount);
        spend_balance(&env, from.clone(), amount);
        TokenUtils::new(&env).events().burn(from, amount);
    }

    pub fn decimals(env: Env) -> u32 {
        read_metadata(&env).decimal
    }

    pub fn name(env: Env) -> String {
        read_metadata(&env).name
    }

    pub fn symbol(env: Env) -> String {
        read_metadata(&env).symbol
    }

    // New freeze functionality
    pub fn freeze_account(env: Env, account: Address) {
        let admin = read_administrator(&env);
        admin.require_auth();

        write_frozen_status(&env, &account, true);
        env.events().publish((symbol_short!("freeze"),), (account.clone(),));
    }

    pub fn unfreeze_account(env: Env, account: Address) {
        let admin = read_administrator(&env);
        admin.require_auth();

        write_frozen_status(&env, &account, false);
        env.events().publish((symbol_short!("unfreeze"),), (account.clone(),));
    }

    pub fn is_frozen(env: Env, account: Address) -> bool {
        is_account_frozen(&env, &account)
    }

    pub fn admin(env: Env) -> Address {
        read_administrator(&env)
    }

    pub fn set_admin(env: Env, new_admin: Address) {
        let admin = read_administrator(&env);
        admin.require_auth();
        write_administrator(&env, &new_admin);
        TokenUtils::new(&env).events().set_admin(admin, new_admin);
    }
}

pub struct TokenUtils(Env);

impl TokenUtils {
    fn new(env: &Env) -> TokenUtils {
        TokenUtils(env.clone())
    }

    fn events(&self) -> TokenEvents {
        TokenEvents::new(&self.0)
    }
}

pub struct TokenEvents(Env);

impl TokenEvents {
    pub fn new(env: &Env) -> TokenEvents {
        TokenEvents(env.clone())
    }

    pub fn mint(&self, admin: Address, to: Address, amount: i128) {
        let topics = (symbol_short!("mint"), admin, to);
        self.0.events().publish(topics, amount);
    }

    pub fn approve(&self, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        let topics = (symbol_short!("approve"), from, spender);
        self.0.events().publish(topics, (amount, expiration_ledger));
    }

    pub fn transfer(&self, from: Address, to: Address, amount: i128) {
        let topics = (symbol_short!("transfer"), from, to);
        self.0.events().publish(topics, amount);
    }

    pub fn burn(&self, from: Address, amount: i128) {
        let topics = (symbol_short!("burn"), from);
        self.0.events().publish(topics, amount);
    }

    pub fn set_admin(&self, admin: Address, new_admin: Address) {
        let topics = (symbol_short!("set_admin"), admin);
        self.0.events().publish(topics, new_admin);
    }
}

fn read_metadata(env: &Env) -> TokenMetadata {
    env.storage()
        .instance()
        .get(&DataKey::State(env.current_contract_address()))
        .unwrap()
}

fn write_metadata(env: &Env, metadata: TokenMetadata) {
    env.storage()
        .instance()
        .set(&DataKey::State(env.current_contract_address()), &metadata);
}

fn read_administrator(env: &Env) -> Address {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .unwrap()
}

fn has_administrator(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

fn write_administrator(env: &Env, id: &Address) {
    env.storage().instance().set(&DataKey::Admin, id);
}

fn read_allowance(env: &Env, from: Address, spender: Address) -> AllowanceValue {
    let key = DataKey::Allowance(AllowanceDataKey { from, spender });
    if let Some(allowance) = env.storage().temporary().get::<DataKey, AllowanceValue>(&key) {
        if allowance.expiration_ledger < env.ledger().sequence() {
            AllowanceValue {
                amount: 0,
                expiration_ledger: allowance.expiration_ledger,
            }
        } else {
            allowance
        }
    } else {
        AllowanceValue {
            amount: 0,
            expiration_ledger: 0,
        }
    }
}

fn write_allowance(
    env: &Env,
    from: Address,
    spender: Address,
    amount: i128,
    expiration_ledger: u32,
) {
    let allowance = AllowanceValue {
        amount,
        expiration_ledger,
    };

    if amount > 0 && expiration_ledger < env.ledger().sequence() {
        panic!("expiration_ledger is less than ledger seq when amount > 0")
    }

    let key = DataKey::Allowance(AllowanceDataKey { from, spender });
    env.storage()
        .temporary()
        .set(&key, &allowance);

    if amount > 0 {
        let live_for = expiration_ledger
            .checked_sub(env.ledger().sequence())
            .unwrap();

        env.storage().temporary().extend_ttl(&key, live_for, live_for)
    }
}

fn spend_allowance(env: &Env, from: Address, spender: Address, amount: i128) {
    let allowance = read_allowance(env, from.clone(), spender.clone());

    if allowance.amount < amount {
        panic!("insufficient allowance")
    }
    write_allowance(
        env,
        from,
        spender,
        allowance.amount - amount,
        allowance.expiration_ledger,
    );
}

fn read_balance(env: &Env, addr: Address) -> i128 {
    let key = DataKey::Balance(addr);
    if let Some(balance) = env.storage().persistent().get::<DataKey, i128>(&key) {
        balance
    } else {
        0
    }
}

fn receive_balance(env: &Env, addr: Address, amount: i128) {
    let balance = read_balance(env, addr.clone());
    let key = DataKey::Balance(addr);
    env.storage()
        .persistent()
        .set(&key, &(balance + amount));
}

fn spend_balance(env: &Env, addr: Address, amount: i128) {
    let balance = read_balance(env, addr.clone());
    if balance < amount {
        panic!("insufficient balance");
    }
    let key = DataKey::Balance(addr);
    env.storage()
        .persistent()
        .set(&key, &(balance - amount));
}

// New frozen account functionality
fn is_account_frozen(env: &Env, addr: &Address) -> bool {
    let key = DataKey::Frozen(addr.clone());
    env.storage().persistent().get::<DataKey, bool>(&key).unwrap_or(false)
}

fn write_frozen_status(env: &Env, addr: &Address, frozen: bool) {
    let key = DataKey::Frozen(addr.clone());
    if frozen {
        env.storage().persistent().set(&key, &frozen);
    } else {
        env.storage().persistent().remove(&key);
    }
}
