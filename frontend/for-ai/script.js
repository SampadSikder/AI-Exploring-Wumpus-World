const URL = "http://localhost:8080/ai/explore"
const GOLD = {
    "piece": "G",
    "piece_name": "Gold",
    "image": "./resources/wumpus/gold.gif",
    "effect_name": "Glitter",
    "effect": "g"
};

const PIT = {
    "piece": "P",
    "piece_name": "Pit",
    "image": "./resources/wumpus/pitc.gif",
    "effect_name": "Breeze",
    "effect": "b"
};

const WUMPUS = {
    "piece": "W",
    "piece_name": "Wumpus",
    "image": "./resources/wumpus/wumpusc.gif",
    "effect_name": "Stench",
    "effect": "s"
};

const EXPLORED_CELL = {
    "piece": "C",
    "piece_name": "Explored",
    "image": "./resources/floor.png"
}
const UNEXPLORED_CELL = {
    "piece": "C",
    "piece_name": "Unexplored",
    "image": "./resources/floor_1.png"
}
const NORMAL_CELL = {
    "piece": "C",
    "piece_name": "Normal",
    "image": "./resources/floor_1.png",
    "effect": "n"
};

const AGENT = {
    "piece": "A",
    "piece_name": "Agent",
    "image": "./resources/player_facing_to_down.png",
    "effect_name": ""
}

let wumpusWorld = [];
let exploredWorld = [];
let agent_position = {"x":0, "y":0};
const WUMPUS_WORLD_SIZE = 10;
let arrows = 1;


const ENTITY_COUNT = {
    "gold": 3,
    "pit": 10,
    "wumpus": 3
}

function generate_world() {


    let entity_count = {
        "gold": 0,
        "pit": 0,
        "wumpus": 0
    }

    // Initializing
    for (let i = 0; i < WUMPUS_WORLD_SIZE; i++) {
        const row = [];
        for (let j = 0; j < WUMPUS_WORLD_SIZE; j++) {
          row.push(NORMAL_CELL); 
        }
        wumpusWorld.push(row);
    }

    // Generating World
    for (let i = WUMPUS_WORLD_SIZE-1; i > 0; i--) {
        for (let j = WUMPUS_WORLD_SIZE-1; j > 0; j--) {
            const randomValue = Math.random();
            if (randomValue < 0.1 && entity_count.gold <= ENTITY_COUNT.gold) {
                wumpusWorld[i][j] = GOLD ;
                entity_count.gold+=1;
            } else if (randomValue < 0.3 && entity_count.pit <= ENTITY_COUNT.pit) {
                wumpusWorld[i][j] = PIT ;
                entity_count.pit +=1 ;
            } else if (randomValue < 0.4 && entity_count.wumpus <= ENTITY_COUNT.wumpus) {
                wumpusWorld[i][j] = WUMPUS ;
                entity_count.wumpus += 1;
            } else {
                wumpusWorld[i][j] = NORMAL_CELL ;
            }
        }
    } 

    /*const STATIC_WORLD = [
        [AGENT, NORMAL_CELL, PIT, NORMAL_CELL],
        [NORMAL_CELL, NORMAL_CELL,NORMAL_CELL, NORMAL_CELL],
        [WUMPUS, GOLD, PIT, NORMAL_CELL],
        [NORMAL_CELL, NORMAL_CELL, NORMAL_CELL, PIT]
    ];
    wumpusWorld = STATIC_WORLD;*/

    exploredWorld = JSON.parse(JSON.stringify(wumpusWorld));

    for (let i = 0; i < exploredWorld.length; i++)
        for (let j = 0; j < exploredWorld.length; j++)
            exploredWorld[i][j] = UNEXPLORED_CELL;

    wumpusWorld[0][0] = AGENT;
    exploredWorld[0][0] = AGENT;
    console.log(wumpusWorld);
}


function drawExploredWorld() {
    const gridElement = document.getElementById("grid");
    while (gridElement.firstChild) {
        gridElement.removeChild(gridElement.firstChild);
    }

    for (let i = 0; i < exploredWorld.length; i++) {
        for (let j = 0; j < exploredWorld[i].length; j++) {
            const cellValue = exploredWorld[i][j];

            const squareElement = document.createElement("div");
            squareElement.id = `${i}-${j}`;
            squareElement.classList.add("square");

            if (exploredWorld[i][j]==EXPLORED_CELL) {
                let percepts = "";
                if(i-1>0 && wumpusWorld[i-1][j]!=NORMAL_CELL && wumpusWorld[i-1][j]!=GOLD) percepts+=wumpusWorld[i-1][j].effect_name+"\n";
                if(i+1<WUMPUS_WORLD_SIZE && wumpusWorld[i+1][j]!=NORMAL_CELL && wumpusWorld[i+1][j]!=GOLD) percepts+=wumpusWorld[i+1][j].effect_name+"\n";
                if(j-1>0 && wumpusWorld[i][j-1]!=NORMAL_CELL && wumpusWorld[i][j-1]!=GOLD) percepts+=wumpusWorld[i][j-1].effect_name+"\n";
                if(j+1<WUMPUS_WORLD_SIZE && wumpusWorld[i][j+1]!=NORMAL_CELL && wumpusWorld[i][j+1]!=GOLD) percepts+=wumpusWorld[i][j+1].effect_name+"\n";

                squareElement.innerText = percepts;
           }

            if (cellValue["piece_name"] == "Explored") {
                squareElement.style.backgroundImage = `url(${EXPLORED_CELL.image})`;
                squareElement.classList.add("wumpus");
            } else if (cellValue["piece_name"] == "Unexplored" || cellValue["piece_name"] == "Normal") {
                squareElement.style.backgroundImage = `url(${NORMAL_CELL.image})`;
            } else if (cellValue["piece_name"] == "Agent") {
                squareElement.style.backgroundImage = `url(${AGENT.image})`;
            }
            
            if(exploredWorld[i][j]==EXPLORED_CELL){
                if (wumpusWorld[i][j]==WUMPUS) {
                    squareElement.style.backgroundImage = `url(${WUMPUS.image})`;
                } else if (wumpusWorld[i][j]==GOLD) {
                    squareElement.style.backgroundImage = `url(${GOLD.image})`;
                } else if (wumpusWorld[i][j]==PIT) {
                    squareElement.style.backgroundImage = `url(${PIT.image})`;
                }
            }

            gridElement.appendChild(squareElement);
        }
    }
    console.log(exploredWorld);
}


function makeYourMoveAI() {
    console.log("Wait for move");
    console.log({
        x: agent_position.x,
        y: agent_position.y,
        piece: getPercepts(agent_position.x,agent_position.y),
        arrows: arrows
    });

    fetch(URL, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            x: agent_position.x,
            y: agent_position.y,
            piece: getPercepts(agent_position.x,agent_position.y),
            arrows: arrows,
            path: []
        }),
    })
        .then((response) => response.json())
        .then((reply) => {
            console.log("AI has replied!");
            console.log(reply);


            exploredWorld[agent_position.x][agent_position.y] = EXPLORED_CELL;
            if(reply.path.length!=0){
                // Simulate move
                agent_position.x = reply.path[reply.path.length-1][0];
                agent_position.y = reply.path[reply.path.length-1][1];
                console.log(`PATH SIMULATIONN:`);
                console.log(agent_position);
            }
            else {
                agent_position.x = reply.x;
                agent_position.y = reply.y;
            }
            exploredWorld[agent_position.x][agent_position.y] = AGENT;

            let message = "";
            if(wumpusWorld[agent_position.x][agent_position.y]==GOLD) {
                message = "You have gained gold!";
                wumpusWorld[agent_position.x][agent_position.y] = NORMAL_CELL;
            }else if(wumpusWorld[agent_position.x][agent_position.y]==WUMPUS && reply.arrows < arrows) {
                message = "You have shot an arrow to wumpus and killed it!";
                wumpusWorld[agent_position.x][agent_position.y] = NORMAL_CELL;
            } else if(wumpusWorld[agent_position.x][agent_position.y]==WUMPUS) {
                message = "You have been killed by wumpus! GAME OVER.";
                //wumpusWorld[agent_position.x][agent_position.y] = NORMAL_CELL;
            } else if(wumpusWorld[agent_position.x][agent_position.y]==PIT) {
                message = "You have dropped into PIT! GAME OVER.";
                //wumpusWorld[agent_position.x][agent_position.y] = NORMAL_CELL;
            }

            document.getElementById("game-message").innerText = "Message: "+message;

            drawExploredWorld();
        })
        .catch((error) => {
            console.error("An error occurred:", error);
        });
}

// Miscellaneous

function handleNearbyCells(i, j, OBJECT) {
    if (i > 0 && j > 0 && i < exploredWorld.length && j < exploredWorld.length && exploredWorld[i][j] != AGENT) {
        let cell = document.getElementById(`${i}-${j}`);
        cell.removeAttribute("src");
        cell.innerText = OBJECT.effect_name;
        return true;
    }
    return false;
}

function getPerceptAt(x,y){
    if(x<0 || y<0 || x>=wumpusWorld.length || y>=wumpusWorld.length) return null;

    if(wumpusWorld[x][y]==PIT) return PIT.effect;
    if(wumpusWorld[x][y]==WUMPUS) return WUMPUS.effect;
    //if(wumpusWorld[x][y]==GOLD) return GOLD.effect;
    return null;
}

function getPercepts(x,y){
    percepts = "";
    if(getPerceptAt(x-1,y)) percepts+=getPerceptAt(x-1,y);
    if(getPerceptAt(x+1,y)) percepts+=getPerceptAt(x+1,y);
    if(getPerceptAt(x,y-1)) percepts+=getPerceptAt(x,y-1);
    if(getPerceptAt(x,y+1)) percepts+=getPerceptAt(x,y+1);

    if(percepts.length == 0) percepts += NORMAL_CELL.effect;

    return percepts;
}


function updateLog(moveNumber, content) {
    const tableBody = document.querySelector("#moveTable tbody");

    const newRow = document.createElement("tr");

    const moveCell = document.createElement("td");
    moveCell.textContent = moveNumber;
    newRow.appendChild(moveCell);

    const playerCell = document.createElement("td");
    playerCell.textContent = content;
    newRow.appendChild(playerCell);

    tableBody.appendChild(newRow);
}

function drawOriginalWorld() {
    const gridElement = document.getElementById("original-world");
    while (gridElement.firstChild) {
        gridElement.removeChild(gridElement.firstChild);
    }

    for (let i = 0; i < wumpusWorld.length; i++) {
        for (let j = 0; j < wumpusWorld[i].length; j++) {
            const cellValue = wumpusWorld[i][j];

            const squareElement = document.createElement("img");
            squareElement.id = `origin-${i}-${j}`;
            squareElement.classList.add("square");

            squareElement.innerText = cellValue.piece_name;

            if (cellValue["piece_name"] == "Wumpus") {
                squareElement.setAttribute("src", WUMPUS.image);
                squareElement.classList.add("wumpus");
            } else if (cellValue["piece_name"] === "Gold") {
                squareElement.setAttribute("src", GOLD.image);
                squareElement.classList.add("gold");
            } else if (cellValue["piece_name"] === "Pit") {
                squareElement.setAttribute("src", PIT.image);
                squareElement.classList.add("pit");
            } else if (cellValue["piece_name"] == "Unexplored" || cellValue["piece_name"] == "Normal") {
                squareElement.setAttribute("src", NORMAL_CELL.image);
            } else if (cellValue["piece_name"] == "Agent") {
                squareElement.setAttribute("src", AGENT.image);
            }

            gridElement.appendChild(squareElement);
        }
    }
}
