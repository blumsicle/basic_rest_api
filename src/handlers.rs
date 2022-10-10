use actix_web::{delete, get, post, put, web, HttpResponse};

use crate::{errors::ErrorNoId, models::Ticket, AppState};

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[post("/tickets")]
async fn post_ticket(req: web::Json<Ticket>, data: web::Data<AppState>) -> HttpResponse {
    let new_ticket = Ticket {
        id: req.id,
        author: req.author.clone(),
    };

    let mut tickets = data.tickets.lock().unwrap();
    tickets.push(new_ticket.clone());

    HttpResponse::Created().json(new_ticket)
}

#[get("/tickets")]
async fn get_tickets(data: web::Data<AppState>) -> HttpResponse {
    let tickets = data.tickets.lock().unwrap();
    HttpResponse::Ok().json(&*tickets)
}

#[get("/tickets/{id}")]
async fn get_ticket(id: web::Path<u32>, data: web::Data<AppState>) -> Result<Ticket, ErrorNoId> {
    let ticket_id = *id;
    let tickets = data.tickets.lock().unwrap();
    let ticket = tickets.iter().filter(|x| x.id == ticket_id).next();

    match ticket {
        Some(t) => Ok(t.to_owned()),
        None => Err(ErrorNoId {
            id: ticket_id,
            err: "ticket not found".to_string(),
        }),
    }
}

#[put("/tickets/{id}")]
async fn update_ticket(
    id: web::Path<u32>,
    req: web::Json<Ticket>,
    data: web::Data<AppState>,
) -> Result<Ticket, ErrorNoId> {
    let ticket_id = *id;
    let mut tickets = data.tickets.lock().unwrap();

    let ticket = tickets.iter_mut().find(|x| x.id == ticket_id);

    match ticket {
        Some(t) => {
            t.id = req.id;
            t.author = req.author.clone();
            Ok(t.to_owned())
        }

        None => Err(ErrorNoId {
            id: ticket_id,
            err: "ticket not found".to_string(),
        }),
    }
}

#[delete("/tickets/{id}")]
async fn delete_ticket(id: web::Path<u32>, data: web::Data<AppState>) -> Result<Ticket, ErrorNoId> {
    let ticket_id = *id;
    let mut tickets = data.tickets.lock().unwrap();
    let index = tickets.iter().position(|x| x.id == ticket_id);

    match index {
        Some(i) => Ok(tickets.remove(i)),
        None => Err(ErrorNoId {
            id: ticket_id,
            err: "ticket not found".to_string(),
        }),
    }
}
