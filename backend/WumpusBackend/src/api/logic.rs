use serde::{Serialize, Deserialize};

const BREEZE : u8 = b'b';
const STENCH : u8 = b's';
const GLITTER : u8 = b'g';

const PIT : u8 = b'p';
const WUMPUS : u8 = b'w';
const GOLD : u8 = b'$';

const WUMPUS_WORLD_SIZE : i32 = 4;

#[derive(Debug, Serialize, Deserialize)]
pub struct CellKnowledge {
    x: usize,
    y: usize,

    pit: bool,
    wumpus: bool, 
    gold: bool ,

    countBreezeSensedNearby: u64,
    countStenchSensedNearby: u64,
    countGlitterSensedNearby: u64,

    visited: bool
}

impl CellKnowledge {
    pub fn new(x: usize,y: usize) -> CellKnowledge{
        CellKnowledge{
            x,
            y,
            pit: true,
            wumpus: true, 
            gold: true ,
        
            countBreezeSensedNearby: 0,
            countStenchSensedNearby: 0,
            countGlitterSensedNearby: 0, 

            visited : false
        }
    } 
}


pub fn initialize_knowledge_base(knowledge_base: &mut Vec<Vec<CellKnowledge>>){
    for i in 0..WUMPUS_WORLD_SIZE {
        let mut row: Vec<CellKnowledge>  = Vec::new();
        for j in 0..WUMPUS_WORLD_SIZE {
            row.push(CellKnowledge::new(i as usize,j as usize));
        }
        knowledge_base.push(row);
    }
}

fn update_knowledge_base(x: i32, y: i32, perceived_arr: &Vec<u8>, knowledge_base: &mut Vec<Vec<CellKnowledge>>) -> () {    
    if x<0 || y<0 || x>=WUMPUS_WORLD_SIZE || y>=WUMPUS_WORLD_SIZE {return;}

    let x:usize = x as usize;
    let y:usize = x as usize;

    for perceived  in perceived_arr {
        knowledge_base[x][y].pit &= *perceived == BREEZE;
        knowledge_base[x][y].wumpus &= *perceived == STENCH;
        knowledge_base[x][y].gold &= *perceived == GLITTER;

        if *perceived == BREEZE { knowledge_base[x][y].countBreezeSensedNearby+=1 }
        else if *perceived == STENCH { knowledge_base[x][y].countStenchSensedNearby+=1 }
        else if *perceived == GLITTER { knowledge_base[x][y].countGlitterSensedNearby+=1 }
    }
}


pub fn get_next_move(x: i32, y: i32, perceived: &Vec<u8>, knowledge_base: &mut Vec<Vec<CellKnowledge>>, num_of_arrows: &mut u32)->(i32, i32){
    knowledge_base[x as usize][y as usize].visited = true;

    return (1,1);
}