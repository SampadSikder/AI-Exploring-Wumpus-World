use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use serde::{Deserialize, Serialize};
use super::logic::get_next_move;
use super::logic::initialize_knowledge_base;
use super::logic::CellKnowledge;

#[derive(Deserialize, Serialize)]
struct Cell {
    x: i32,
    y: i32,
    perceived_environment:Vec<u8>,
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
    let perceived_environment = &cell_data.perceived_environment;

    unsafe{
        get_next_move(cell_data.x as usize, cell_data.y as usize, perceived_environment, &mut knowledge_base, cell_data.arrows);
    }

    
    for item in perceived_environment {
        println!("perceived_environment item: {}", item);
    }
    HttpResponse::Ok().json(cell_data)
}
