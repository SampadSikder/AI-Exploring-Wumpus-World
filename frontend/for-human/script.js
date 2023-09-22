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

const wumpusWorld = [];
let exploredWorld = [];
let PLAYER_DEAD = false;

function generate_world(){
    let wumpusCount = 0, goldCount =0, pitCount=0, MAX=3;
    for (let i = 0; i < 10; i++) {
        let row = [];
        for (let j = 0; j < 10; j++) {
            const randomValue = Math.random();

            if (randomValue < 0.1 && goldCount<MAX) {
                row.push(GOLD);
                goldCount+=1;
            } else if (randomValue < 0.3 && pitCount<MAX) {
                row.push(PIT);
                pitCount+=1;
            } else if (randomValue < 0.4 && wumpusCount<MAX) {
                row.push(WUMPUS);
                wumpusCount+=1;
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
    exploredWorld[0][0] = AGENT;

    console.log(exploredWorld);
}


function drawOriginalWorld(world) {
    const gridElement = document.getElementById("grid");
    while (gridElement.firstChild) {
        gridElement.removeChild(gridElement.firstChild);
    }

    for (let i = 0; i < world.length; i++) {
        for (let j = 0; j < world[i].length; j++) {
            const cellValue = world[i][j];
	    
            const squareElement = document.createElement("div");
            squareElement.id = `${i}-${j}`;
            squareElement.classList.add("square");
            
            squareElement.onclick = function(event){makeMove(event,i,j);};


            if (cellValue["piece_name"] == "Wumpus") {
		        squareElement.style.backgroundImage =  `url(${WUMPUS.image})`;
                squareElement.classList.add("wumpus");
            } else if (cellValue["piece_name"] === "Gold") {
		        squareElement.style.backgroundImage = `url(${GOLD.image})`;
                squareElement.classList.add("gold");
            } else if (cellValue["piece_name"] === "Pit") {
		        squareElement.style.backgroundImage = `url(${PIT.image})`;
                squareElement.classList.add("pit");
            } else if(cellValue["piece_name"]=="Unexplored" || cellValue["piece_name"]=="Normal"){
		        squareElement.style.backgroundImage = `url(${NORMAL_CELL.image})`;
            } else if(cellValue["piece_name"]=="Agent"){
		        squareElement.style.backgroundImage = `url(${AGENT.image})`;
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

    drawOriginalWorld(exploredWorld);

    // Adding and Hiding stuffs for player
    for (let i = 0; i < exploredWorld.length; i++) {
        for (let j = 0; j < exploredWorld[i].length; j++) {
            const cellValue = exploredWorld[i][j];
            let cell = document.getElementById(`${i}-${j}`);

            handleNearbyCells(i, j, wumpusWorld[i-1][j]);
            handleNearbyCells(i, j, wumpusWorld[i+1][j]);
            handleNearbyCells(i, j, wumpusWorld[i][j-1]);
            handleNearbyCells(i, j, wumpusWorld[i][j+1]);

	                
            if(cellValue == EXPLORED_CELL) cell.style.backgroundImage =  `url(${EXPLORED_CELL.image})`;
            else if(cellValue == AGENT)cell.style.backgroundImage =  `url(${AGENT.image})`;
            else if(PLAYER_DEAD) {
                if(wumpusWorld[i][j]==WUMPUS) cell.style.backgroundImage = `url(${WUMPUS.image})`;

                else if(wumpusWorld[i][j]==PIT) cell.style.backgroundImage = `url(${PIT.image})`;

            }
            else cell.style.backgroundImage =`url(${UNEXPLORED_CELL.image})`;
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
    if(i>0 && j>0 && i<exploredWorld.length && j<exploredWorld.length && exploredWorld[i][j] == EXPLORED_CELL && (OBJECT==WUMPUS ||OBJECT==PIT)) {
        document.getElementById(`${i}-${j}`).innerText += OBJECT.effect_name;
    }
}

function findElement(matrix, target) {
    for (let row = 0; row < matrix.length; row++) {
      for (let col = 0; col < matrix[row].length; col++) {
        if (matrix[row][col] === target) {
          return { row: row, col: col }; 
        }
      }
    }
    return null; 
  }

function makeMove(event,moveX,moveY){
    updateLog(`x=${moveX}, y=${moveY}`, exploredWorld[moveX][moveY].piece_name);
    
    const cell = findElement(exploredWorld, AGENT);
    let agentPosX = cell.row, agentPosY = cell.col;
    console.log(`DEBUG: Agent=(${agentPosX}, ${agentPosY}). Move=(${moveX},${moveY}).`);


    if (Math.abs(agentPosX - moveX) + Math.abs(agentPosY - moveY) !== 1) {
        updateLog("ERROR", "Invalid Move!");
        drawExploredWorld();
        return;
    }


    if(wumpusWorld[moveX][moveY]==WUMPUS || wumpusWorld[moveX][moveY]==PIT) {
        updateLog("END","You have died!");
        PLAYER_DEAD = true;
        drawExploredWorld();
        return true;
    }
    else if(wumpusWorld[moveX][moveY]==GOLD ){
         updateLog("RICH","You have found Gold!");
    }

    exploredWorld[agentPosX][agentPosY] = EXPLORED_CELL;
    exploredWorld[moveX][moveY] = AGENT;
    drawExploredWorld();
    console.log(exploredWorld);
}
