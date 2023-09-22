const GOLD = {
    "piece": "G",
    "piece_name":"Gold",
    "image": "./resources/wumpus/gold.gif",
    "effect_name": "Shine"
};

const PIT = {
    "piece": "P",
    "piece_name":"Pit",
    "image": "./resources/wumpus/pitc.gif",
    "effect_name": "Breeze"
};

const WUMPUS = {
    "piece": "W",
    "piece_name":"Wumpus",
    "image": "./resources/wumpus/wumpusc.gif",
    "effect_name": "Stench"
};

const BREEZE = {
    "piece": "B",
    "piece_name":"Breeze"
};

const STENCH = {
    "piece": "S",
    "piece_name":"Stench"
};

const EXPLORED_CELL = {
    "piece": "C",
    "piece_name":"Explored",
    "image": "./resources/floor.png"
}
const UNEXPLORED_CELL = {
    "piece": "C",
    "piece_name":"Unexplored",
    "image": "./resources/floor_1.png"
}
const NORMAL_CELL = {
    "piece": "C",
    "piece_name":"Normal",
    "image": "./resources/floor_1.png"
};

const AGENT = {
    "piece": "A",
    "piece_name":"Agent",
    "image": "./resources/player_facing_to_down.png"
}

let wumpusWorld = [];
let exploredWorld = [];

function generate_world(){
    for (let i = 0; i < 10; i++) {
        let row = [];
        for (let j = 0; j < 10; j++) {
            const randomValue = Math.random();

            if (randomValue < 0.1) {
                row.push(GOLD);
            } else if (randomValue < 0.3) {
                row.push(PIT);
            } else if (randomValue < 0.4) {
                row.push(WUMPUS);
            } else {
                row.push(NORMAL_CELL);
            }
        }
        wumpusWorld.push(row);
    }

    exploredWorld = JSON.parse(JSON.stringify(wumpusWorld));
    
    for(let i=0; i<exploredWorld.length; i++)
        for(let j=0; j<exploredWorld.length; j++)
            exploredWorld[i][j] = UNEXPLORED_CELL;

    wumpusWorld[0][0] = AGENT;
    exploredWorld[0][0] = EXPLORED_CELL;

    console.log(wumpusWorld);
}


function drawOriginalWorld() {
    const gridElement = document.getElementById("grid");
    
    for (let i = 0; i < wumpusWorld.length; i++) {
        for (let j = 0; j < wumpusWorld[i].length; j++) {
            const cellValue = wumpusWorld[i][j];
	    
            const squareElement = document.createElement("img");
            squareElement.id = `${i}-${j}`;
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
            } else if(cellValue["piece_name"]=="Unexplored" || cellValue["piece_name"]=="Normal"){
                squareElement.setAttribute("src", NORMAL_CELL.image);
            } else if(cellValue["piece_name"]=="Agent"){
                squareElement.setAttribute("src", AGENT.image);
            } 
	    
            gridElement.appendChild(squareElement);
        }
    }
}


function drawExploredWorld() {
    const gridElement = document.getElementById("grid");
    while (gridElement.firstChild) {
        gridElement.removeChild(gridElement.firstChild);
    }

    drawOriginalWorld();

    // Adding and Hiding stuffs for player
    for (let i = 0; i < wumpusWorld.length; i++) {
        for (let j = 0; j < wumpusWorld[i].length; j++) {
            const cellValue = wumpusWorld[i][j];
            let cell = document.getElementById(`${i}-${j}`);
	                
            if(cellValue["piece_name"]=="Explored"){
                cell.setAttribute("src", EXPLORED_CELL.image);
            } else if(cellValue["piece_name"]!="Agent"){
                cell.setAttribute("src", UNEXPLORED_CELL.image);
            }
        }
    }
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

// Miscellaneous

function handleNearbyCells(i,j, OBJECT){
    if(i>0 && j>0 && i<exploredWorld.length && j<exploredWorld.length && exploredWorld[i][j]!=AGENT) {
        let cell = document.getElementById(`${i}-${j}`);
        cell.removeAttribute("src");
        cell.innerText = OBJECT.effect_name;
        return true;
    }
    return false;
}
