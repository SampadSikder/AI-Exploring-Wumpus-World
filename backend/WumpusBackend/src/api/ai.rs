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
    perceived_environment:Vec<String>,
    arrows: i32
}

#[post("/ai/explore")]
pub async fn start_explore(
    cell: web::Json<Cell>
) -> HttpResponse {
    let cell_data = cell.into_inner();
    let perceived_environment = &cell_data.perceived_environment;

    
    for item in perceived_environment {
        println!("perceived_environment item: {}", item);
    }
    HttpResponse::Ok().json(cell_data)
}
