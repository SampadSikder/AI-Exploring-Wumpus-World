// Create a grid of squares
const grid = [[null, null, null],
              [null, null, null],
              [null, null, null]];

    // Add the Wumpus and the treasure to the grid
grid[1][1] = "Wumpus";
grid[2][2] = "Gold";

// Create a function to draw the grid
function drawGrid() {
    const gridElement = document.getElementById("grid");
    
    for (let i = 0; i < grid.length; i++) {
        for (let j = 0; j < grid[i].length; j++) {
            const square = grid[i][j];
	    
            const squareElement = document.createElement("div");
            squareElement.classList.add("square");
	    
            if (square === "Wumpus") {
		squareElement.classList.add("wumpus");
            } else if (square === "Gold") {
		squareElement.classList.add("gold");
            }
	    
            gridElement.appendChild(squareElement);
        }
    }
}



