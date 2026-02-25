use {
    crate::db::WalletDb,
    crate::util::*,
    crate::wallet::*,
    actix_web::{
        HttpResponse,
        web::{Data, Json},
    },
    sqlx::PgPool,
};

// List all wallets
#[actix_web::get("/wallets")]
pub async fn list_wallets(pool: Data<Option<PgPool>>) -> HttpResponse {
    let Some(pool) = pool.get_ref().as_ref() else {
        return ResponseType::Ok(Vec::<Wallet>::new()).get_response();
    };

    let wallets = sqlx::query_as::<_, WalletDAO>("SELECT address, club_name FROM wallets")
        .fetch_all(pool)
        .await;

    match wallets {
        Ok(rows) => {
            let payload = rows
                .into_iter()
                .map(|row| Wallet {
                    address: row.address,
                    club_name: row.club_name,
                    total_hash_rate: 0,
                    total_shares_mined: 0,
                    total_workers_online: 0,
                    workers_online: vec![],
                })
                .collect::<Vec<Wallet>>();

            ResponseType::Ok(payload).get_response()
        }
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "message": format!("failed to load wallets: {err}")
        })),
    }
}

//Get a wallet
#[actix_web::get("/wallets/{id}")]
pub async fn get_wallet() -> HttpResponse {
    //TODO: Get the walletDAO from DB where id = request id and convert to Wallet obj
    let wallet: Option<Wallet> = None;
    match wallet {
        Some(wallet) => ResponseType::Ok(wallet).get_response(),
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
    db.insert(new_id, _wallet_request.into_inner());
    ResponseType::Created(NewWalletResponse {
        id: new_id,
        club_name: club_name,
    })
    .get_response()
}
