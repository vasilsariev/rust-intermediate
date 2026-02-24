use {crate::util::*, crate::wallet::*, actix_web::HttpResponse, actix_web::web::Json};

// List all wallets
#[get("/wallets")]
pub async fn list_wallets() -> HttpResponse {
    //TODO: Get all WalletDAO objects from DB and convert to Wallet objects

    let wallets: Vec<Wallet> = vec![];
    ResponseType::Ok(wallets).get_response()
}

//Get a wallet
#[get("/wallets/{id}")]
pub async fn get_wallet() -> HttpResponse {
    //TODO: Get the walletDAO from DB where id = request id and convert to Wallet obj
    let wallet: Option<Wallet> = None;
    match wallet {
        Some(wallet) => ResponseType::Ok(wallet).get_response(),
        None => ResponseType::NotFound(NotFoundMessage::new(String::from("Wallet not found")))
            .get_response(),
    }
}

#[post("/wallets")]
pub async fn create_wallet(_wallet_request: Option<Json<NewWalletRequest>>) -> HttpResponse {
    let wallet: Vec<Wallet> = vec![];
    ResponseType::Created(wallet).get_response()
}
