use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Ticket {
    pub id: u32,
    pub author: String,
}

impl Responder for Ticket {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

