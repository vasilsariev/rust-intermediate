use {crate::util::*, crate::wallet::*, actix_web::HttpResponse, actix_web::web::Json};

// List all wallets
#[get("/miners")]
pub async fn list_miners() -> HttpResponse {
    //TODO: Get all MinerDAO objects from DB and convert to Miner objects

    let miners: Vec<Wallet> = vec![];
    ResponseType::Ok(miners).get_response()
}

//Get a wallet
#[get("/wallets/{id}")]
pub async fn get_wallet() -> HttpResponse {
    //TODO: Get the walletDAO from DB where id = request id and convert to Wallet obj
    let wallet: Option<Wallet> = None;
    match wallet {
        Some(wallet) => ResponseType::Ok(wallet).get_response(),
        None => ResponseType::NotFound(NotFoundMessage::new(String::from("Miner not found")))
            .get_response(),
    }
}

#[post("/wallets")]
pub async fn create_miner(miner_request: Json<NewWalletRequest>) -> HttpResponse {
    let wallet: Vec<Wallet> = vec![];
    ResponseType::Created(wallet).get_response()
}
