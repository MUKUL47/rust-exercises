
enum GameState{
    Won, 
    Draw,
    Running
}
#[allow(dead_code)]
pub struct TicTacToe{
    board: [[usize;3];3],
    player: usize,
    cells_covered: i32,
}
impl TicTacToe {
    pub fn new() -> Self{
        TicTacToe{
            board: [[0,0,0],[0,0,0],[0,0,0]],
            player: 1,
            cells_covered: 0
        }
    }
    #[allow(dead_code)]
    fn get_empty_coordinates(&self) -> Vec<usize> {
        return self.board.iter()
            .flat_map(|f| f.iter())
            .filter_map(|i| {
                if *i > 0 {
                    Some(*i)
                }else{
                    None
                }
            }).collect();
    }
    #[allow(dead_code)]
    fn ask_cell_coordinate(&self) -> String{
        let mut cell = String::from("");
        println!("Enter cell (1 - 9), Player - {0}", self.x_o(self.player));
        std::io::stdin().read_line(&mut cell).unwrap();
        cell = cell.trim_end().to_string();
        return cell
    }

    fn x_o(&self, n: usize) -> &str{
        match n {
            1 => "X",
            2 => "O",
            _ => " "
        }
    }

    fn print_board(&self) {
        println!("1|2|3      {0}|{1}|{2}",self.x_o(self.board[0][0]),self.x_o(self.board[0][1]),self.x_o(self.board[0][2]));
        println!("-----      -----");
        println!("4|5|6      {0}|{1}|{2}",self.x_o(self.board[1][0]),self.x_o(self.board[1][1]),self.x_o(self.board[1][2]));
        println!("-----      -----");
        println!("7|8|9      {0}|{1}|{2}",self.x_o(self.board[2][0]),self.x_o(self.board[2][1]),self.x_o(self.board[2][2]));
    }

    fn cell_idx_map(&self,n : usize) -> (usize, usize){
        match n {
            1 => (0,0),
            2 => (0,1),
            3 => (0,2),
            4 => (1,0),
            5 => (1,1),
            6 => (1,2),
            7 => (2,0),
            8 => (2,1),
            9 => (2,2),
            _ => (0,0)
        }
    }

    fn check_win_draw_conditions(&self) -> GameState{
        let b = self.board;
        if  b[0][0] == b[0][1] && b[0][1] == b[0][2] && b[0][0] != 0 ||
            b[1][0] == b[1][1] && b[1][1] == b[1][2] && b[1][0] != 0 ||
            b[2][0] == b[2][1] && b[2][1] == b[2][2] && b[2][0] != 0 ||
            //
            b[0][0] == b[1][0] && b[1][0] == b[2][0] && b[0][0] != 0 ||
            b[0][1] == b[1][1] && b[1][1] == b[2][1] && b[0][1] != 0 ||
            b[0][2] == b[1][2] && b[1][2] == b[2][2] && b[0][2] != 0 ||
            //
            b[0][2] == b[1][1] && b[1][1] == b[2][2] && b[0][2] != 0 ||
            b[0][2] == b[1][1] && b[1][1] == b[2][0] && b[0][2] != 0
         {
            return GameState::Won
        }
        if self.cells_covered < 9 {
            return GameState::Running;
        };
        return GameState::Draw
    }

    pub fn start_game(&mut self) {
        loop {
            self.print_board();
            let cell_coord = self.ask_cell_coordinate();    
            match cell_coord.parse::<usize>() {
                Ok(cell_c) => {
                    match cell_c{
                        1..= 9 => {
                            let (i,j) = self.cell_idx_map(cell_c);
                            if self.board[i][j] != 0 {
                                println!("This cell position is already consumed, pick another");
                                continue;
                            }
                            self.board[i][j] = self.player;
                            self.cells_covered += 1;
                            match self.check_win_draw_conditions(){
                                GameState::Draw => {
                                    println!("Nobody won!");
                                    self.print_board();
                                    break;
                                }
                                GameState::Won => {
                                    println!("Player {0} won!!",self.x_o(self.player));
                                    self.print_board();
                                    break;
                                }
                                _ => {}
                            }
                            match self.player{
                                1 => self.player = 2,
                                _ => self.player = 1
                            }
                        }
                        _ => {
                            println!("!Invalid range only 1 - 9!");
                        }
                    }
                }Err (_) => {
                    println!("!Enter valid number!");
                }
            }
        }
    }
}