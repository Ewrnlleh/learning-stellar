// ! This contract implements trading of one token pair between one seller and
// ! multiple buyers.
// ! It demonstrates one of the ways of how trading might be implemented.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

// Represents the storage keys for the contract.
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Offer,
}

// Represents an offer managed by the SingleOffer contract.
// If a seller wants to sell XLM for USDC, they would set
// 'sell_token' as XLM, 'buy_token' as USDC, and set the price accordingly.
#[derive(Clone)]
#[contracttype]
pub struct Offer {
    // Owner of this offer. Sells sell_token to get buy_token.
    pub seller: Address,
    pub sell_token: Address,
    pub buy_token: Address,
    // Seller-defined price of the sell token in arbitrary units.
    pub sell_price: u32,
    // Seller-defined price of the buy token in arbitrary units.
    pub buy_price: u32,
}



#[contract]

pub struct SingleOffer;

//! How this contract should be used:
//! 1. Call `create` once to create the offer and register its seller.
//! 2. Seller may transfer arbitrary amounts of the `sell_token` for sale to the
//!    contract address for trading. They may also update the offer price.
//! 3. Buyers may call `trade` to trade with the offer. The contract will
//!    immediately perform the trade and send the respective amounts
//!    of `buy_token` and `sell_token` to the seller and buyer respectively.
//! 4. Seller may call `withdraw` to claim any remaining `sell_token` balance.

#[contractimpl]
impl SingleOffer {
    /// Creates the offer for seller for the given token pair and initial price.
    /// See comment above the `Offer` struct for information on pricing.
    pub fn create(
        e: Env,
        seller: Address,
        sell_token: Address,
        buy_token: Address,
        sell_price: u32,
        buy_price: u32,
    ) {
        // Check if offer already exists
        if e.storage().instance().has(&DataKey::Offer) {
            panic!("offer is already created");
        }
        // Check for zero price
        if buy_price == 0 || sell_price == 0 {
            panic!("zero price is not allowed");
        }
        // Authorize the `create` call by seller to verify their identity.
        seller.require_auth();
        write_offer(
            &e,
            Offer {
                seller,
                sell_token,
                buy_token,
                sell_price,
                buy_price,
            },
        );

       
    }

    /// Trades `buy_token_amount` of buy_token from buyer for sell_token amount defined by the price.
    /// `min_sell_token_amount` defines a lower bound on the price that the buyer would accept.
    /// Buyer needs to authorize the `trade` call and internal `transfer` call to the contract address.
    pub fn trade(
        e: Env,
        buyer: Address,
        buy_token_amount: i128,
        min_sell_token_amount: i128,
    ) {
        // Buyer needs to authorize the trade.
        buyer.require_auth();

        // Load the offer and prepare the token clients to do the trade.
        let offer: Offer = read_offer(&e);
        let sell_token_client = token::Client::new(&e, &offer.sell_token);
        let buy_token_client = token::Client::new(&e, &offer.buy_token);

        // Compute the amount of sell_token that buyer needs to receive.
        // sell_token_amount = buy_token_amount * sell_price / buy_price
        let sell_token_amount: i128 = buy_token_amount i128
            .checked_mul(offer.sell_price as i128)
            .unwrap_optimized()
            / offer.buy_price as i128;

        // Check if the computed amount is above the minimum acceptable by the buyer.
        if sell_token_amount < min_sell_token_amount {
            panic!("price is too low");
        }

        let contract: Address = e.current_contract_address();

        // Perform the trade in 3 'transfer' steps.
        // Note: We don't need to verify any balances — the contract will
        // just trap and roll back if any of the transfers fails for any reason,
        // including insufficient balance.

        // 1. Transfer the 'buy_token' from buyer to this contract.
        // This transfer call should be authorized by buyer.
        // This could as well be a direct transfer to the seller, but sending to
        // the contract address allows building more transparent signature
        // payload where the buyer doesn't need to worry about sending token to
        // some 'unknown' third party.
        buy_token_client.transfer(&buyer, &contract, &buy_token_amount);

        // 2. Transfer the 'sell_token' from contract to buyer.
        sell_token_client.transfer(&contract, &buyer, &sell_token_amount);

        // 3. Transfer the 'buy_token' to the seller immediately.
        buy_token_client.transfer(&contract, &offer.seller, &buy_token_amount);

    }

    /// Sends amount of token from this contract to the seller.
    /// Must be authorized by seller.
    /// This is intentionally flexible so that the seller can withdraw any
    /// outstanding balance of the contract (in case they mistakenly transferred wrong token to it).
    pub fn withdraw(e: Env, token: Address, amount: i128) {
        let offer: Offer = read_offer(&e);
        offer.seller.require_auth();

        let token_client = token::Client::new(&e, &token);
        token_client.transfer(
            &e.current_contract_address(),
            &offer.seller,
            &amount,
        );
    }


    /// Updates the price.
    /// Must be authorized by seller.
    pub fn update_price(e: Env, sell_price: u32, buy_price: u32) {
        if buy_price == 0 || sell_price == 0 {
            panic!("zero price is not allowed");
        }
        let mut offer: Offer = read_offer(&e);
        offer.seller.require_auth();
        offer.sell_price = sell_price;
        offer.buy_price = buy_price;
        write_offer(&e, offer);
    }

    /// Returns the current state of the offer.
    pub fn get_offer(e: Env) -> Offer {
        read_offer(&e)
    }


    fn read_offer(e: &Env) -> Offer {
        e.storage().instance().get(&DataKey::Offer).unwrap()
    }

    fn write_offer(e: &Env, offer: Offer) {
        e.storage().instance().set(&DataKey::Offer, &offer);
    }





      


}

