use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use serde::{Deserialize, Serialize};
use super::logic::get_next_move;
use super::logic::initialize_knowledge_base;
use super::logic::CellKnowledge;
use super::save_kb::save_knowledge_base;
use super::save_kb::load_knowledge_base;

#[derive(Deserialize, Serialize)]
pub struct Cell {
    x: i32,
    y: i32,
    piece: String,
    arrows: u32
}

#[get("/")]
async fn initialize() -> impl Responder {
    let mut knowledge_base:Vec<Vec<CellKnowledge>> = Vec::new();
    initialize_knowledge_base(&mut knowledge_base);
    println!("Done initializing KB. Time to save it.");

    save_knowledge_base(&String::from("kb.txt"),&knowledge_base);
    let result = load_knowledge_base(&String::from("kb.txt"));
    match result {
        Ok(data) => {
            print!("Loaded the kb successfully");
        } 
        Err(error)=>{
            print!("Got error {}",error);
        }  
    }

    return HttpResponse::Ok().body("Knowledge Base Initialized & Saved.");
}

#[post("/ai/explore")]
pub async fn start_explore(
    cell: web::Json<Cell>
) -> HttpResponse {
    let mut cell_data = cell.into_inner();
    println!("{:?}", cell_data.piece);

    let mut perceived : Vec<u8> = Vec::new();
    for character in cell_data.piece.as_bytes() {
        perceived.push(*character);
    }

    let result = load_knowledge_base(&String::from("kb.txt"));
    match result {
        Ok(mut knowledge_base) => {
            let _move = get_next_move(cell_data.x, cell_data.y, &perceived, &mut knowledge_base, &mut cell_data.arrows);
            let _ = save_knowledge_base(&String::from("kb.txt"),&knowledge_base);

            HttpResponse::Ok().json(Cell{
                x: _move.0 as i32,
                y: _move.1 as i32,
                piece: cell_data.piece,
                arrows: cell_data.arrows
            })
        } 
        Err(error)=>{
            print!("Got error {}",error);

            HttpResponse::Ok().json("Encountered error while loading knowledge base.")
        }  
    }
}
