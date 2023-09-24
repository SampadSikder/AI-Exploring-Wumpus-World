use serde::{Serialize, Deserialize};

const BREEZE : u8 = b'b';
const STENCH : u8 = b's';
const GLITTER : u8 = b'g';

const PIT : u8 = b'p';
const WUMPUS : u8 = b'w';
const GOLD : u8 = b'$';

const WUMPUS_WORLD_SIZE : i32 = 10;

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
    if x<0 || y<0 || x>WUMPUS_WORLD_SIZE || y>WUMPUS_WORLD_SIZE {return;}

    let x:usize = x as usize;
    let y:usize = x as usize;

    for perceived  in perceived_arr {
        knowledge_base[x][y].pit ^= *perceived == BREEZE;
        knowledge_base[x][y].wumpus ^= *perceived == STENCH;
        knowledge_base[x][y].gold ^= *perceived == GLITTER;

        if *perceived == BREEZE { knowledge_base[x][y].countBreezeSensedNearby+=1 }
        else if *perceived == STENCH { knowledge_base[x][y].countStenchSensedNearby+=1 }
        else if *perceived == GLITTER { knowledge_base[x][y].countGlitterSensedNearby+=1 }
    }
}

fn predicate_glittery_and_safe_path(x: i32, y: i32, knowledge_base: &mut Vec<Vec<CellKnowledge>>)->bool{
    if x<0 || y<0 || x>WUMPUS_WORLD_SIZE || y>WUMPUS_WORLD_SIZE {return false;}
    
    let x:usize = x as usize;
    let y:usize = x as usize;

    if knowledge_base[x][y].countGlitterSensedNearby >=1 && knowledge_base[x][y].pit==false && knowledge_base[x][y].wumpus==false {return true;}

    return false;
}

fn predicate_safe_unvisited_path(x: i32, y: i32, knowledge_base: &mut Vec<Vec<CellKnowledge>>)->bool{
    if x<0 || y<0 || x>WUMPUS_WORLD_SIZE || y>WUMPUS_WORLD_SIZE {return false;}
    
    let x:usize = x as usize;
    let y:usize = x as usize;

    if knowledge_base[x][y].visited==false && knowledge_base[x][y].pit==false && knowledge_base[x][y].wumpus==false {return true;}

    return false;
}

fn predicate_throw_arrow(x: i32, y: i32, knowledge_base: &mut Vec<Vec<CellKnowledge>>, num_of_arrows: &mut u32)->bool{
    if x<0 || y<0 || x>WUMPUS_WORLD_SIZE || y>WUMPUS_WORLD_SIZE {return false;}

    let x:usize = x as usize;
    let y:usize = x as usize;

    if knowledge_base[x][y].countStenchSensedNearby>=2 && knowledge_base[x][y].countBreezeSensedNearby<2 && *num_of_arrows>0 {
        *num_of_arrows -= 1;
        return true;
    }
    return false;
}

fn exclude_death_paths(x: i32, y: i32, knowledge_base: &mut Vec<Vec<CellKnowledge>>) -> Vec<(usize, usize)>{
    let mut less_dangerous_paths : Vec<(usize, usize)> = Vec::new();

    let x:usize = x as usize;
    let y:usize = x as usize;

    // ERROR , CHECK x-1 >0

    if knowledge_base[x-1][y].countBreezeSensedNearby < 2 && knowledge_base[x-1][y].countStenchSensedNearby < 2 {less_dangerous_paths.push((x-1,y));}
    else if knowledge_base[x+1][y].countBreezeSensedNearby < 2 && knowledge_base[x+1][y].countStenchSensedNearby < 2 {less_dangerous_paths.push((x+1,y));}
    else if knowledge_base[x][y-1].countBreezeSensedNearby < 2 && knowledge_base[x][y-1].countStenchSensedNearby < 2 {less_dangerous_paths.push((x,y-1));}
    else if knowledge_base[x][y+1].countBreezeSensedNearby < 2 && knowledge_base[x][y+1].countStenchSensedNearby < 2 {less_dangerous_paths.push((x,y+1));}

    return less_dangerous_paths;
}
pub fn backtrack(x: i32,y: i32, knowledge_base: &mut Vec<Vec<CellKnowledge>>) -> bool {
    if x<0 || y<0 || x>WUMPUS_WORLD_SIZE || y>WUMPUS_WORLD_SIZE {return false;}

    let x:usize = x as usize;
    let y:usize = x as usize;

    return knowledge_base[x][y].visited;
}

pub fn get_next_move(x: i32, y: i32, perceived: &Vec<u8>, knowledge_base: &mut Vec<Vec<CellKnowledge>>, num_of_arrows: &mut u32)->(i32, i32){
    knowledge_base[x as usize][y as usize].visited = true;

    update_knowledge_base(x-1, y, &perceived, knowledge_base);
    update_knowledge_base(x+1, y, &perceived, knowledge_base);
    update_knowledge_base(x, y-1, &perceived, knowledge_base);
    update_knowledge_base(x, y+1, &perceived, knowledge_base);

    if predicate_glittery_and_safe_path(x+1, y, knowledge_base) {return (x+1,y)} 
    else if predicate_glittery_and_safe_path(x-1, y, knowledge_base) {return (x-1,y)} 
    else if predicate_glittery_and_safe_path(x, y+1, knowledge_base) {return (x,y+1)} 
    else if predicate_glittery_and_safe_path(x, y-1, knowledge_base) {return (x,y-1)} ;

    if predicate_throw_arrow(x-1,y,knowledge_base,num_of_arrows) {return (x-1,y);}
    else if predicate_throw_arrow(x+1,y,knowledge_base,num_of_arrows) {return (x+1,y);}
    else if predicate_throw_arrow(x,y-1,knowledge_base,num_of_arrows) {return (x,y-1);}
    else if predicate_throw_arrow(x,y+1,knowledge_base,num_of_arrows) {return (x,y+1);}

    if predicate_safe_unvisited_path(x+1, y, knowledge_base) {return (x+1,y)} 
    else if predicate_safe_unvisited_path(x-1, y, knowledge_base) {return (x-1,y)} 
    else if predicate_safe_unvisited_path(x, y+1, knowledge_base) {return (x,y+1)} 
    else if predicate_safe_unvisited_path(x, y-1, knowledge_base) {return (x,y-1)} ;

    // Backtrack since no visited safe path found 
    if backtrack(x+1,y,knowledge_base) {return (x+1,y);}
    else if backtrack(x-1,y,knowledge_base) {return (x-1,y);}
    else if backtrack(x,y+1,knowledge_base) {return (x,y+1);}
    else if backtrack(x,y-1,knowledge_base) {return (x,y-1);}

    // We are back to cell (1,1). We have no other choice but to make a dangerous move So we will list "probably dangerous" paths and pick one at random.
    let probably_dangerous_paths = exclude_death_paths(x,y, knowledge_base);
    return (probably_dangerous_paths[0].0 as i32, probably_dangerous_paths[0].1 as i32);

}