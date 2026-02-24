use {
    crate::miner::*, crate::util::*, actix_web::HttpResponse, actix_web::web, actix_web::web::Json,
};

// List all miners
#[actix_web::get("/miners")]
pub async fn list_miners() -> HttpResponse {
    //TODO: Get all MinerDAO objects from DB and convert to Miner objects

    let miners: Vec<Miner> = vec![];
    ResponseType::Ok(miners).get_response()
}

#[actix_web::get("/miners/{id}")]
pub async fn get_miner(user_id: web::Path<u32>) -> HttpResponse {
    //TODO: Get the minerDAO from DB WHERE id = request id and convert to Miner obj
    let miner: Option<Miner> = None;
    match miner {
        Some(miner) => ResponseType::Ok(miner).get_response(),
        None => ResponseType::NotFound(NotFoundMessage::new(String::from("Miner not found")))
            .get_response(),
    }
}

#[actix_web::post("/wallets/{id}/miners/")]
pub async fn create_miner(_miner_request: Option<Json<NewMinerRequest>>) -> HttpResponse {
    let miner: Vec<Miner> = vec![];
    ResponseType::Created(miner).get_response()
}
