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

const BREEZE = {
    "piece": "B",
    "piece_name": "Breeze"
};

const STENCH = {
    "piece": "S",
    "piece_name": "Stench"
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
    "image": "./resources/player_facing_to_down.png"
}

let wumpusWorld = [];
let exploredWorld = [];
let agent_position = {"x":0, "y":0};

function generate_world() {
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

    for (let i = 0; i < exploredWorld.length; i++)
        for (let j = 0; j < exploredWorld.length; j++)
            exploredWorld[i][j] = UNEXPLORED_CELL;

    wumpusWorld[0][0] = AGENT;
    exploredWorld[0][0] = AGENT;
    console.log(wumpusWorld);
}

function drawOriginalWorld() {
    const gridElement = document.getElementById("original-world");

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
            } else if (cellValue["piece_name"] == "Unexplored" || cellValue["piece_name"] == "Normal") {
                squareElement.setAttribute("src", NORMAL_CELL.image);
            } else if (cellValue["piece_name"] == "Agent") {
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

    for (let i = 0; i < exploredWorld.length; i++) {
        for (let j = 0; j < exploredWorld[i].length; j++) {
            const cellValue = exploredWorld[i][j];

            const squareElement = document.createElement("img");
            squareElement.id = `${i}-${j}`;
            squareElement.classList.add("square");

            squareElement.innerText = cellValue.piece_name;

            if (cellValue["piece_name"] == "Explored") {
                squareElement.setAttribute("src", EXPLORED_CELL.image);
                squareElement.classList.add("wumpus");
            } else if (cellValue["piece_name"] == "Unexplored" || cellValue["piece_name"] == "Normal") {
                squareElement.setAttribute("src", NORMAL_CELL.image);
            } else if (cellValue["piece_name"] == "Agent") {
                squareElement.setAttribute("src", AGENT.image);
            }

            gridElement.appendChild(squareElement);
        }
    }
}


function makeYourMoveAI() {
    console.log("Wait for move");
    console.log(agent_position);

    fetch(URL, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            x: agent_position.x,
            y: agent_position.y,
            piece: getPercepts(agent_position.x,agent_position.y),
            arrows: 1 
        }),
    })
        .then((response) => response.json())
        .then((reply) => {
            console.log("AI has replied!");
            console.log(reply);

            exploredWorld[agent_position.x][agent_position.y] = EXPLORED_CELL;
            agent_position.x = reply.x;
            agent_position.y = reply.y;
            exploredWorld[agent_position.x][agent_position.y] = AGENT;

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
    if(x<0 || y<0 || x>wumpusWorld.length || y>wumpusWorld.length) return null;

    if(wumpusWorld[x][y]==PIT) return PIT.effect;
    if(wumpusWorld[x][y]==WUMPUS) return WUMPUS.effect;
    if(wumpusWorld[x][y]==GOLD) return GOLD.effect;
    return NORMAL_CELL.effect;
}

function getPercepts(x,y){
    percepts = "";
    if(getPerceptAt(x-1,y)) percepts+=getPerceptAt(x-1,y);
    if(getPerceptAt(x+1,y)) percepts+=getPerceptAt(x+1,y);
    if(getPerceptAt(x,y-1)) percepts+=getPerceptAt(x,y-1);
    if(getPerceptAt(x,y+1)) percepts+=getPerceptAt(x,y+1);

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
