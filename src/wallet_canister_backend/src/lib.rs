//use ic_cdk::export::candid::{CandidType, Deserialize};
use candid::{CandidType, Deserialize};
//use ic_cdk_macros::{query, update};
use ic_cdk::query;
use ic_cdk::update;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
struct Wallet {
    balances: HashMap<String, u64>, // Store token balances per user
}

static mut WALLET: Option<Wallet> = None;
#[ic_cdk::init]
fn init_wallet() {
    let wallet = Wallet {
        balances: HashMap::new(),
    };
    unsafe { WALLET = Some(wallet) };
}

#[update]
fn send_tokens(from: String, to: String, amount: u64) -> String {
    unsafe {
        let wallet = WALLET.as_mut().unwrap();
        let sender_balance = wallet.balances.entry(from.clone()).or_insert(0);

        if *sender_balance < amount {
            return format!("Insufficient balance for {}", from);
        }

        *sender_balance -= amount;
        let receiver_balance = wallet.balances.entry(to.clone()).or_insert(0);
        *receiver_balance += amount;

        format!("Sent {} tokens from {} to {}", amount, from, to)
    }
}

#[update]
fn receive_tokens(user: String, amount: u64) -> String {
    unsafe {
        let wallet = WALLET.as_mut().unwrap();
        let balance = wallet.balances.entry(user.clone()).or_insert(0);
        *balance += amount;

        format!("{} received {} tokens", user, amount)
    }
}

#[query]
fn get_balance(user: String) -> u64 {
    unsafe {
        let wallet = WALLET.as_ref().unwrap();
        *wallet.balances.get(&user).unwrap_or(&0)
    }
}
