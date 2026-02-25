use {
    crate::db::WalletDb,
    crate::util::*,
    crate::wallet::*,
    actix_web::{
        HttpResponse,
        web::{Data, Json},
    },
};

// List all wallets
#[actix_web::get("/wallets")]
pub async fn list_wallets() -> HttpResponse {
    ResponseType::NotFound(NotFoundMessage::new(String::from("Wallet not found"))).get_response()
}

//Get a wallet
#[actix_web::get("/wallets/{id}")]
pub async fn get_wallet(wallet_id: actix_web::web::Path<u32>, db: Data<WalletDb>) -> HttpResponse {
    let wallet_id = wallet_id.into_inner();
    let db = db.lock().unwrap();
    match db.get(&wallet_id) {
        Some(wallet_data) => ResponseType::Ok(wallet_data).get_response(),
        None => ResponseType::NotFound(NotFoundMessage::new(String::from("Wallet not found")))
            .get_response(),
    }
}

#[actix_web::post("/wallets")]
pub async fn create_wallet(
    _wallet_request: Json<NewWalletRequest>,
    db: Data<WalletDb>,
) -> HttpResponse {
    let mut db = db.lock().unwrap();
    let new_id = db.keys().max().unwrap_or(&0) + 1;
    let club_name = _wallet_request.club_name.clone();
    let wallet_dao = WalletDAO {
        id: new_id,
        club_name: club_name.clone(),
    };
    db.insert(new_id, wallet_dao);
    ResponseType::Created(NewWalletResponse {
        id: new_id,
        club_name: club_name,
    })
    .get_response()
}
