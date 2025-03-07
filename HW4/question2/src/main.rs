struct Board {
    board: [[bool; 24]; 24], // true for there and false for not
}

impl Board {

    fn new(board: [[bool; 24]; 24]) -> Self {
        Board { board }
    }

    fn get_neighbors(&self, coord: (i8, i8)) -> i8 {
        // Returns the number of alive neighbors

        let mut neighbors: i8 = 0;

        let offsets: [(i8, i8); 8] = [
            (1, 0), (1, 1), (1, -1), 
            (0, 1), (0, -1), 
            (-1, 1), (-1, 0), (-1, -1)
        ];

        // Iterate over all neighbors
        for (i, j) in offsets {
            
            // Wrap around
            let find_row: i8 = if (coord.0 + i) >= 0 && 24 > (coord.0 + i)
                    {coord.0 + i} else { 23 };
            let find_col: i8 = if (coord.1 + j) >= 0 && 24 > (coord.1 + j)
                    {coord.1 + j} else { 23 };

            if self.board[find_row as usize][find_col as usize] {
                neighbors += 1;
            }
        }

        return neighbors;

    }

    fn run_generation(&self) -> Self {

        let mut new_board: [[bool; 24]; 24] = [[false; 24]; 24];

        for i in 0..24 {
            for j in 0..24 {
                new_board[i][j] = self.evaluate((i as i8, j as i8));
            }
        }

        return Board::new(new_board);
    }

    fn print_board(&self) {
        for i in 0..24 {
            for j in 0..24 {
                print!("{}|", if self.board[i][j] { "1" } else { "0" });
            }
            println!("")
        }
    }

    fn evaluate(&self, coord: (i8, i8)) -> bool {
        let pos: bool = self.board[coord.0 as usize][coord.1 as usize];
        let num_neighbors = self.get_neighbors(coord);

        return match num_neighbors {
            3 => true, // to alive
            2 => pos, // no change
            _ => false, // default to false
        }
    }
}

fn main() {
    // Run sim
    let initial_board: [[bool; 24]; 24] = [[false; 24]; 24];
    let mut board = Board::new(initial_board);
    
    // Setup intial condition
    let initial_points = vec![(0,1), (1,2), (2,0), (2,1), (2,2)];

    for p in initial_points {
        board.board[p.0][p.1] = true;
    }

    // Run simulation
    let mut current_board = board;
    for gen in 1..=24 {
        current_board = current_board.run_generation();
        println!("Generation: {}", gen);
        current_board.print_board();
    }

    // Test that bad senario dies
    let t1_initial_board: [[bool; 24]; 24] = [[false; 24]; 24];
    let mut t1_board = Board::new(t1_initial_board);
    
    // Setup intial condition
    let t1_initial_points = vec![(0,0), (0,2), (2,0), (2,2)]; // spaced so all should die

    for p in t1_initial_points {
        t1_board.board[p.0][p.1] = true;
    }

    assert_eq!(t1_board.run_generation().board, [[false; 24]; 24]);
    // Test that good senario is correct
    let t2_initial_board: [[bool; 24]; 24] = [[false; 24]; 24];
    let mut t2_board = Board::new(t2_initial_board);
    
    // Setup intial condition
    let t2_initial_points = vec![(0,0), (0,1), (1,0)]; // should form box

    for p in t2_initial_points {
        t2_board.board[p.0][p.1] = true;
    }

    let mut answer_board = [[false; 24]; 24];
    answer_board[0][0] = true;
    answer_board[1][0] = true;
    answer_board[0][1] = true;
    answer_board[1][1] = true;

    assert_eq!(t2_board.run_generation().board, answer_board);
    // Test evaluate function
    let t3_initial_board: [[bool; 24]; 24] = [[false; 24]; 24];
    let mut t3_board = Board::new(t3_initial_board);
    
    // Test (0, 0) survives
    let t3_initial_points = vec![(0,0), (0,1), (1,0)]; // should form box

    for p in t3_initial_points {
        t3_board.board[p.0][p.1] = true;
    }

    assert_eq!(t3_board.evaluate((0, 0)), true);
    // Wrap around kills
    let t4_initial_board: [[bool; 24]; 24] = [[false; 24]; 24];
    let mut t4_board = Board::new(t4_initial_board);
    
    // Test 4 dies
    let t4_initial_points = vec![(0,0), (0,1), (1,0), (23, 0), (0, 23)]; // should kill (0,0)

    for p in t4_initial_points {
        t4_board.board[p.0][p.1] = true;
    }
    
    assert_eq!(t4_board.evaluate((0, 0)), false);
}
