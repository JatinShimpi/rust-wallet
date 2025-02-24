use candid::{Principal, CandidType};
use ic_cdk::{init, post_upgrade, query, update};
use serde::Deserialize;
use std::cell::RefCell;
use std::collections::HashMap;

mod security;
use security::is_authorized;

thread_local! {
    static BALANCES: RefCell<HashMap<Principal, u64>> = RefCell::new(HashMap::new());
}

#[init]
fn init() {
    // Initialize with admin balance (example: 1_000_000 tokens)
    let admin = ic_cdk::caller();
    BALANCES.with(|b| b.borrow_mut().insert(admin, 1_000_000));
}

#[update]
fn transfer(to: Principal, amount: u64) -> Result<(), String> {
    let caller = ic_cdk::caller();
    
    // Security check
    if !is_authorized(&caller) {
        return Err("Unauthorized".to_string());
    }

    BALANCES.with(|b| {
        let mut balances = b.borrow_mut();
        let from_balance = balances.get(&caller).copied().unwrap_or(0);
        
        if from_balance < amount {
            return Err("Insufficient balance".to_string());
        }

        balances.insert(caller, from_balance - amount);
        let to_balance = balances.get(&to).copied().unwrap_or(0);
        balances.insert(to, to_balance + amount);
    });

    Ok(())
}

#[query]
fn get_balance(user: Principal) -> u64 {
    BALANCES.with(|b| b.borrow().get(&user).copied().unwrap_or(0))
}

// Candid interface for deployment
ic_cdk::export_candid!();