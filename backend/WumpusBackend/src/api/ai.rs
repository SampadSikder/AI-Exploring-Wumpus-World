use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use serde::{Deserialize, Serialize};
use super::logic::get_next_move;
use super::logic::initialize_knowledge_base;
use super::logic::CellKnowledge;

#[derive(Deserialize, Serialize)]
struct Cell {
    x: i32,
    y: i32,
    piece:String,
    arrows: u32
}

static mut knowledge_base:Vec<Vec<CellKnowledge>> = Vec::new();


#[get("/")]
async fn hello() -> impl Responder {
    unsafe {
        initialize_knowledge_base(&mut knowledge_base);
        println!("{:?}", knowledge_base);
    }
    return HttpResponse::Ok().body("Knowledge Base Initialized");
}

#[post("/ai/explore")]
pub async fn start_explore(
    cell: web::Json<Cell>
) -> HttpResponse {
    let cell_data = cell.into_inner();
    println!("{:?}", cell_data.piece);
    HttpResponse::Ok().json(cell_data)
}
