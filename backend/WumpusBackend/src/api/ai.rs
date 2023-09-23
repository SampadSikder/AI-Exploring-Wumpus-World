use actix_web::{
    post, 
    web,
    HttpResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Cell {
    x: i32,
    y: i32,
    perceived_environment:String,
    arrows: i32
}

#[post("/ai/explore")]
pub async fn start_explore(
    cell: web::Json<Cell>
) -> HttpResponse {
    let cell_data = cell.into_inner();
    println!("perceived_environment: {}", cell_data.perceived_environment);
    HttpResponse::Ok().json(cell_data)
}
