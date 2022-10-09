use actix_web::{get, web::Json};

use crate::models::Status;

#[get("/status")]
async fn status() -> Json<Status> {
    Json(Status {
        status: "UP".to_owned(),
    })
}
