use crate::wallet::*;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub type WalletDb = Arc<Mutex<HashMap<u32, NewWalletRequest>>>;

pub fn create_wallet_db() -> WalletDb {
    Arc::new(Mutex::new(HashMap::<u32, NewWalletRequest>::new()))
}
