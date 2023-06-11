use std::io;

pub fn read_input() -> String {
    //read input from stdin
    let stdin = io::stdin();
    let mut input_string = String::new();
    let _ = stdin.read_line(&mut input_string);

    //we have to remove the newline from the end
    String::from(input_string.trim_end())
}

struct Cell {
    occupant: Option<Player>,
}

impl Cell {
    fn place(&mut self, player: Player) -> bool {
        if self.occupant.is_some() {
            return false;
        } else {
            self.occupant = Some(player);
            return true;
        }
    }
}
#[derive(Copy, Clone)]
enum Player {
    Cross,
    Circle,
}

struct Board {
    cells: Vec<Cell>,
    dimensions: u32,
}

impl Board {
    fn new(dimensions: u32) -> Board {
        Board {
            cells: {
                let mut cells: Vec<Cell> = vec![];
                for x in 0..dimensions {
                    for y in 0..dimensions {
                        cells.push(Cell { occupant: None });
                    }
                }
                cells
            },
            dimensions: dimensions,
        }
    }

    fn print(&self) {
        for x in 1..self.cells.len() + 1 {
            match self.cells[x - 1] {
                Cell {
                    occupant: Some(Player::Cross),
                } => print!("[X]"),
                Cell {
                    occupant: Some(Player::Circle),
                } => print!("[O]"),
                Cell { occupant: None } => print!("[{}]", x),
            }
            if x as u32 % self.dimensions == 0 && x != 0 {
                println!("");
            }
        }
    }

    fn place(&mut self, id: u32, player: Player) -> bool {
        let got = &mut self.cells[id as usize];
        if !got.place(player) {
            println!("That position is already occupied");
            return false;
        }
        true
    }
}

struct Game {
    board: Board,
    player: Player,
}

impl Game {
    fn new(dimension: u32) -> Game {
        Game {
            board: Board::new(dimension),
            player: Player::Cross,
        }
    }

    fn turn(&mut self) {
        self.board.print();

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

    fn over(&self) -> bool {
        false
    }

    fn run_game(mut self) {
        loop {
            self.turn();
            if self.over() {
                break;
            }
        }
    }
}

fn main() {
    let game = Game::new(3);
    game.run_game();
}
