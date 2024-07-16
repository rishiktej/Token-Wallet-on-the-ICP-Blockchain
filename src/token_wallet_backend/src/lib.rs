use candid::{CandidType, Deserialize};
use ic_cdk_macros::{init, query, update};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
struct Wallet {
    balance: u64,
}

static mut BALANCES: Option<HashMap<String, Wallet>> = None;

#[init]
fn init() {
    unsafe {
        BALANCES = Some(HashMap::new());
    }
}

#[update]
fn send_tokens(to: String, amount: u64) -> bool {
    let from = ic_cdk::api::caller().to_string();
    unsafe {
        let balances = BALANCES.as_mut().unwrap();

        if let Some(sender_wallet) = balances.get_mut(&from) {
            if sender_wallet.balance >= amount {
                sender_wallet.balance -= amount;

                let receiver_wallet = balances.entry(to.clone()).or_insert(Wallet { balance: 0 });
                receiver_wallet.balance += amount;

                return true;
            }
        }
    }
    false
}

#[update]
fn receive_tokens(amount: u64) {
    let receiver = ic_cdk::api::caller().to_string();
    unsafe {
        let balances = BALANCES.as_mut().unwrap();
        let receiver_wallet = balances.entry(receiver).or_insert(Wallet { balance: 0 });
        receiver_wallet.balance += amount;
    }
}

#[query]
fn get_balance() -> u64 {
    let caller = ic_cdk::api::caller().to_string();
    unsafe {
        let balances = BALANCES.as_mut().unwrap();
        if let Some(wallet) = balances.get(&caller) {
            return wallet.balance;
        }
    }
    0
}
