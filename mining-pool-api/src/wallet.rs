use serde::{Deserialize, Serialize};

// ------------------------- JSON Payload (REST)

#[derive(Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub id: u32,
    pub club_name: String,
}

// ------------------ POST Request Body for new Wallet

#[derive(Debug, Deserialize, Serialize)]
pub struct NewWalletRequest {
    pub club_name: String,
}

// ------------------ DAO Object (DB Table Records)

#[derive(Debug, Deserialize, Serialize)]
pub struct WalletDAO {
    pub id: u32,
    pub club_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewWalletResponse {
    pub id: u32,
    pub club_name: String,
}
