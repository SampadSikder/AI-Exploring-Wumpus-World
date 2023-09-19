const GOLD = {
    "piece": "G",
    "piece_name":"Gold",
    "image": "./resources/wumpus/gold.gif"
};

const PIT = {
    "piece": "P",
    "piece_name":"Pit",
    "image": "./resources/wumpus/pitc.gif"
};

const WUMPUS = {
    "piece": "W",
    "piece_name":"Wumpus",
    "image": "./resources/wumpus/wumpusc.gif"
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
    "image": "./resources/wumpus/original_wall.gif"
}
const NORMAL_CELL = {
    "piece": "C",
    "piece_name":"Normal",
    "image": "./resources/wumpus/original_wall.gif"
};

const AGENT = {
    "piece": "A",
    "piece_name":"Agent"
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
            squareElement.classList.add("square");
            
            squareElement.innerText = cellValue.piece_name;

            if (cellValue["piece_name"] == "Wumpus") {
		        squareElement.setAttribute("src", WUMPUS.image);
            } else if (cellValue["piece_name"] === "Gold") {
                squareElement.setAttribute("src", GOLD.image);
            } else if (cellValue["piece_name"] === "Pit") {
		        squareElement.setAttribute("src", PIT.image);
            } else if(cellValue["piece_name"]=="Normal"){
                squareElement.setAttribute("src", NORMAL_CELL.image);
            }
	    
            gridElement.appendChild(squareElement);
        }
    }
}

function drawExploredWorld() {
    const gridElement = document.getElementById("grid");
    
    for (let i = 0; i < wumpusWorld.length; i++) {
        for (let j = 0; j < wumpusWorld[i].length; j++) {
            const cellValue = wumpusWorld[i][j];
	    
            const squareElement = document.createElement("img");
            squareElement.classList.add("square");
            
            squareElement.innerText = cellValue.piece_name;

            if (cellValue["piece_name"] == "Wumpus") {
		        squareElement.setAttribute("src", WUMPUS.image);
            } else if (cellValue["piece_name"] === "Gold") {
                squareElement.setAttribute("src", GOLD.image);
            } else if (cellValue["piece_name"] === "Pit") {
		        squareElement.setAttribute("src", PIT.image);
            } else if(cellValue["piece_name"]=="Explored"){
                squareElement.setAttribute("src", EXPLORED_CELL.image);
            } else if(cellValue["piece_name"]=="Unexplored"){
                squareElement.setAttribute("src", UNEXPLORED_CELL.image);
            }
	    
            gridElement.appendChild(squareElement);
        }
    }
}



