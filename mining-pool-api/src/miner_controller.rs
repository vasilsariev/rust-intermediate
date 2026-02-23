use {crate::miner::*, crate::util::*, actix_web::HttpResponse, actix_web::web::Json};

// List all miners
#[get("/miners")]
pub async fn list_miners() -> HttpResponse {
    //TODO: Get all MinerDAO objects from DB and convert to Miner objects

    let miners: Vec<Miner> = vec![];
    ResponseType::Ok(miners).get_response()
}

#[get("/miners/{id}")]
pub async fn get_miner() -> HttpResponse {
    //TODO: Get the minerDAO from DB WHERE id = request id and convert to Miner obj
    let miner: Option<Miner> = None;
    match miner {
        Some(miner) => ResponseType::Ok(miner).get_response(),
        None => ResponseType::NotFound(NotFoundMessage::new(String::from("Miner not found")))
            .get_response(),
    }
}

#[post("/miners")]
pub async fn create_miner(miner_request: Json<NewMinerRequest>) -> HttpResponse {
    let miner: Vec<Miner> = vec![];
    ResponseType::Created(miner).get_response()
}
