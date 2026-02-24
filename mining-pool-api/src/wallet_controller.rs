use {
    crate::util::*,
    crate::wallet::*,
    actix_web::{
        web::{Data, Json},
        HttpResponse,
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
pub async fn create_wallet(_wallet_request: Option<Json<NewWalletRequest>>) -> HttpResponse {
    let wallet: Vec<Wallet> = vec![];
    ResponseType::Created(wallet).get_response()
}
