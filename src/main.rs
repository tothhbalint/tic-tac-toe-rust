use std::io;

pub fn read_input() -> String {
        //read input from stdin
        let stdin = io::stdin();
        let mut input_string = String::new();
        let _ = stdin.read_line(&mut input_string);

        //we have to remove the newline from the end
        String::from(input_string.trim_end())
}

enum CellState{
    Occupied,
    None
}

struct Cell{
    state : CellState
}

enum Player{
    Cross,
    Circle
}


struct Board{
    cells: Vec<(Cell,u32)>,
    dimensions : u32,
}

impl Board{
    fn new(dimensions : u32) -> Board{
        Board {cells :
            {
                let mut cells : Vec<(Cell,u32)> = vec![];
                for x in 0..dimensions{
                    for y in 0..dimensions{
                        cells.push((Cell {state : CellState::None},x+y*3));
                    }
                }
                cells
            }, dimensions : dimensions}
    }

    fn print(self){
        for x in 1..self.cells.len()+1{
            print!("[{}]", x);
            if x%3 == 0 && x!=0{
                println!("");
            }
        }
    }
}

struct Game{
    board : Board,
    player : Player
}

impl Game{
    fn new(dimension : u32) -> Game{
        Game {board: Board::new(dimension), player : Player::Cross}
    }


    fn turn(self){
        Board::print(self.board);

        //prompt user for input
        println!("Choose a cell to place your symbol:");

        let input : u32 = read_input().parse().unwrap();
    }

    fn run_game(self){
        self.turn();
    }
}


fn main() {
    let game = Game::new(3);
    game.run_game();
}
