use candid::Principal;
use std::collections::HashSet;

thread_local! {
    static AUTHORIZED_USERS: RefCell<HashSet<Principal>> = RefCell::new(HashSet::new());
}

pub fn initialize_authorized_users(users: Vec<Principal>) {
    AUTHORIZED_USERS.with(|a| {
        let mut auth_users = a.borrow_mut();
        for user in users {
            auth_users.insert(user);
        }
    });
}

pub fn is_authorized(user: &Principal) -> bool {
    AUTHORIZED_USERS.with(|a| a.borrow().contains(user))
}