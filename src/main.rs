use std::io;

//read input from stdin and format it for use later
pub fn read_input() -> String {
    let stdin = io::stdin();
    let mut input_string = String::new();
    let _ = stdin.read_line(&mut input_string);

    String::from(input_string.trim_end())
}

//define a cell struct
struct Cell {
    occupant: Option<Player>,
    // a value for the cell, so the board can be a magic square
    value: i32,
}

//implement the cell struct
impl Cell {
    fn place(&mut self, player: Player, value: u32) -> bool {
        if self.occupant.is_some() {
            return false;
        } else {
            self.occupant = Some(player);
            match player {
                Player::Cross => self.value = -1 * value as i32,
                Player::Circle => self.value = value as i32,
            }
            return true;
        }
    }
}

//define the two players
#[derive(Copy, Clone)]
enum Player {
    Cross,
    Circle,
}

enum Winner {
    Cross,
    Circle,
    Draw,
    None,
}

//define the board
struct Board {
    cells: Vec<Cell>,
    dimensions: u32,
    magic_sum: u32,
    magic_square: Vec<u32>,
}

fn calc_odd_magic_square(start_num : u32,dimensions : u32, square : &mut Vec<u32>){
        let mut x: u32 = start_num + 1;
        //the position to place the number
        let mut y: u32 = (dimensions + 1) / 2;
        //offset in the matrix in x direction
        let mut i: u32 = y % dimensions;

        while x <= dimensions * dimensions + start_num {
            square[(y - 1) as usize] = x;
            x = x + 1;
            let prev_y = y;
            if y <= dimensions {
                y = y + (dimensions - 1) * dimensions;
            } else {
                y = y - dimensions;
            }
            i = y % dimensions;
            if i == 0 {
                y = y - (dimensions - 1);
            } else {
                y = y + 1;
            }
            if square[(y - 1) as usize] != 0 {
                y = prev_y + dimensions;
            }
        }
}

//function to generate the numbers for the magic square
// a vector represents the square, it should be interpreted as a 2d array
fn create_magic_square(dimensions: u32, magic_square: &mut Vec<u32>) {
    magic_square.resize((dimensions * dimensions) as usize, 0);
    //case for a odd magic squares
    if dimensions % 2 != 0 {
        calc_odd_magic_square(0,dimensions, magic_square);
    }
    //case for a doubly even magic square
    else if dimensions % 4 == 0 {
        // TODO implement
    }
    //case for a singly even magic square
    else if dimensions % 3 == 0 {
        // first we divide into 4 quadrants then call calc_odd_magic_square on each quadrant
        let mut squares : Vec<Vec<u32>> = vec![vec![]; 4];
        let dim = dimensions / 2;
        // resize the squares inner vectors to the correct length
        for i in 0..4 {
            squares[i].resize((dim * dim) as usize, 0);
        }
        // fill the squares
        for x in 0..dimensions{
            for y in 0..dimensions{
                //top left
                if x < dim && y < dim{
                    squares[0][(x * dim + y) as usize] = magic_square[(x * dimensions + y) as usize];
                }
                //bottom left
                else if x < dim && y >= dim{
                    squares[3][(x * dim + (y - dim)) as usize] = magic_square[(x * dimensions + y) as usize];
                }
                //top right
                else if x >= dim && y < dim{
                    squares[2][((x - dim) * dim + y) as usize] = magic_square[(x * dimensions + y) as usize];
                }
                //bottom right
                else if x >= dim && y >= dim{
                    squares[1][((x - dim) * dim + (y - dim)) as usize] = magic_square[(x * dimensions + y) as usize];
                }
            }
        }

        // calculate the magic squares for each quadrant
        for i in 0..4{
            calc_odd_magic_square(i as u32 * (dim * dim), dim, &mut squares[i]);
        }

        // fill the magic square
        for x in 0..dimensions{
            for y in 0..dimensions{
                //top left
                if x < dim && y < dim{
                    magic_square[(x * dimensions + y) as usize] = squares[0][(x * dim + y) as usize];
                }
                //bottom left
                else if x < dim && y >= dim{
                    magic_square[(x * dimensions + y) as usize] = squares[2][(x * dim + (y - dim)) as usize];
                }
                //top right
                else if x >= dim && y < dim{
                    magic_square[(x * dimensions + y) as usize] = squares[3][((x - dim) * dim + y) as usize];
                }
                //bottom right
                else if x >= dim && y >= dim{
                    magic_square[(x * dimensions + y) as usize] = squares[1][((x - dim) * dim + (y - dim)) as usize];
                }
            }
        }
    }
}

//implement all functions related to the board
impl Board {
    //create a new board based on thedimensions
    fn new(dimensions: u32) -> Board {
        Board {
            cells: {
                let mut cells: Vec<Cell> = vec![];
                for _ in 0..dimensions * dimensions {
                    cells.push(Cell {
                        occupant: None,
                        value: 0,
                    });
                }
                cells
            },
            dimensions,
            magic_sum: dimensions * (dimensions * dimensions + 1) / 2,
            magic_square: {
                let mut magic_square: Vec<u32> = vec![];
                create_magic_square(dimensions, &mut magic_square);
                magic_square
            },
        }
    }

    //print out the current state of the board
    fn print(&self) {
        print!("\x1B[2J\x1B[1;1H");
        for x in 1..self.cells.len() + 1 {
            match self.cells[x - 1] {
                Cell {
                    occupant: Some(Player::Cross),
                    value: _,
                } => print!("[X]"),
                Cell {
                    occupant: Some(Player::Circle),
                    value: _,
                } => print!("[O]"),
                Cell {
                    occupant: None,
                    value: _,
                } => print!("[{}]", x),
            }
            if x as u32 % self.dimensions == 0 && x != 0 {
                println!("");
            }
        }
    }

    //place a player's symbol on the board
    fn place(&mut self, id: u32, player: Player) -> bool {
        let got = &mut self.cells[id as usize];
        if !got.place(player, self.magic_square[id as usize]) {
            println!("That position is already occupied");
            return false;
        }
        true
    }

    fn check_sum(&self, sum: i32) -> Option<Winner> {
        if sum.abs() == self.magic_sum as i32 {
            if sum < 0 {
                return Some(Winner::Cross);
            } else {
                return Some(Winner::Circle);
            }
        } else {
            return None;
        }
    }

    //check if the game is over based on the magic square, and if so, who won
    fn over(&self) -> Option<Winner> {
        let mut no_empty: bool = true;
        let mut rows: Vec<Vec<i32>> = vec![vec![]; self.dimensions as usize];
        let mut columns: Vec<Vec<i32>> = vec![vec![]; self.dimensions as usize];
        let mut diagonals: Vec<Vec<i32>> = vec![vec![]; 2];


        let mut winner: Option<Winner> = None;

        for x in 0..self.dimensions {
            for y in 0..self.dimensions {
                if self.cells[(x * self.dimensions + y) as usize]
                    .occupant
                    .is_some()
                {
                    //add diags to check
                    if x == y {
                        diagonals[0].push(self.cells[(x * self.dimensions + y) as usize].value);
                    }
                    if x + y == self.dimensions - 1 {
                        diagonals[1].push(self.cells[(x * self.dimensions + y) as usize].value);
                    }

                    //add rows to check
                    rows[x as usize].push(self.cells[(x * self.dimensions + y) as usize].value);

                    //add columns to check
                    columns[y as usize].push(self.cells[(x * self.dimensions + y) as usize].value);
                } else {
                    no_empty = false;
                }
            }
        }

        let to_check: Vec<i32> = {
            let mut to_check: Vec<i32> = vec![];
            for row in rows {
                to_check.push(row.iter().sum());
            }
            for column in columns {
                to_check.push(column.iter().sum());
            }
            for diagonal in diagonals {
                to_check.push(diagonal.iter().sum());
            }
            to_check
        };

        for val in to_check {
                winner = self.check_sum(val);
                if winner.is_some() {
                    break;
                }
        }

        if no_empty && !winner.is_some() {
            return Some(Winner::Draw);
        }
        winner
    }
}

//define the game struct that controls the game
struct Game {
    board: Board,
    player: Player,
}

//implement the game controller
impl Game {
    //create a new game
    fn new(dimension: u32) -> Game {
        Game {
            board: Board::new(dimension),
            player: Player::Cross,
        }
    }

    //run a turn
    fn turn(&mut self) {
        //prompt user for input
        println!("Choose a cell to place your symbol:");

        let input: u32 = read_input().parse().unwrap();

        if self.board.place(input - 1, self.player) {
            self.player = match self.player {
                Player::Cross => Player::Circle,
                Player::Circle => Player::Cross,
            }
        }
    }

    //loop the turns while the game is not over
    fn run_game(mut self) {
        self.board.print();
        loop {
            self.turn();
            self.board.print();
            match self.board.over() {
                Some(Winner::Cross) => {
                    println!("Cross wins!");
                    break;
                }
                Some(Winner::Circle) => {
                    println!("Circle wins!");
                    break;
                }
                Some(Winner::Draw) => {
                    println!("It's a draw!");
                    break;
                }
                Some(Winner::None) => {}
                None => {}
            }
        }
    }
}

// TODO: Add a menu to choose the dimensions of the board

fn main() {
    let game = Game::new(6);
    game.run_game();
}
