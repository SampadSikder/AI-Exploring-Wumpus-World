use actix_web::{
    post, 
    web,
    HttpResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct World {
    arrays: Vec<Vec<i32>>,
}

#[post("/ai/explore")]
pub async fn start_explore(
    exploredWorld: web::Json<World>,
    wumpusWorld: web::Json<World>,
) -> HttpResponse {
    let world_data = exploredWorld.into_inner();
    
    for row in &world_data.arrays {
        for val in row {
            println!("Value: {}", val);
        }
    }
    HttpResponse::Ok().json(world_data)
}
