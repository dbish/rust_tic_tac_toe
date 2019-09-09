use std::io;
use math::round;

fn print_board(board: &[[u8;3];3]){
    let mut piece = String::new();
    println!("---------");
    for i in 0..3{
        for j in 0..3{
            match board[i][j]{
                1 => piece = String::from("X"),
                2 => piece = String::from("O"),
                _ => piece = {
                    let loc = i*3+j+1;
                    loc.to_string() 
                }
            }
            print!("|{}|",piece); 
        }
        println!("\n---------");
    }
}

fn check_for_win(board: &[[u8;3];3]) -> u8{
    //check for column wins
    for i in 0..3{
        if board[i][0] > 0{
            if (board[i][0] == board[i][1]) && (board[i][1] == board[i][2]){
                return board[i][0];
            }
        }
    }

    //check for row wins
    for j in 0..3{
        if board[0][j] > 0{
            if (board[0][j] == board[1][j]) && (board[1][j] == board[2][j]){
                return board[0][j];
            }
        }
    }

    //check for diagonal wins
    if board[1][1] > 0{
        if (board[0][0] == board[1][1]) && (board[1][1] == board[2][2]){
            return board[1][1];
        }
        if (board[0][2] == board[1][1]) && (board[1][1] == board[2][0]){
            return board[1][1];
        }

    }

    return 0; 
}

fn potential_win(board: &[[u8;3];3], player: u8) -> Option<(usize, usize)>{
    let mut board_prime = board.clone();
    for i in 0..3{
        for j in 0..3{
            if board[i][j] == 0{
                board_prime[i][j] = player;
                if check_for_win(&board_prime) == player{
                    return Some((i, j));
                } else {
                    board_prime[i][j] = 0;
                }
            }
        }
    }
    None
}

fn ai_move(board: &mut [[u8;3];3]){
    //check for win option
    let winning_move = potential_win(&board, 2);
    if winning_move != None{
        let moves = winning_move.unwrap();
        board[moves.0][moves.1] = 2; 
    }else{
        //check for block option
        let blocking_move = potential_win(&board, 1);
        if blocking_move != None{
            let moves = blocking_move.unwrap();
            board[moves.0][moves.1] = 2;
        }else{
            //open center
            if board[1][1] == 0{
                board[1][1] = 2;
            }else{
                //open corner
                if board[0][0] == 0{
                    board[0][0] = 2;
                }else if board[0][2] == 0{
                    board[0][2] = 2;
                }else if board[2][0] == 0{
                    board[2][0] = 2;
                }else if board[2][2] == 0{
                    board[2][2] = 2;
                }else{
                    //any open spot
                    for i in 0..3{
                        for j in 0..3{
                            if board[i][j] == 0{
                                board[i][j] = 2;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let mut board = [[0u8; 3]; 3]; 
    print_board(&board);
    let mut winning_player = 0;

    loop {
        println!("please enter your move");
        let mut next_move = String::new();
        io::stdin().read_line(&mut next_move).expect("failed to read line");
        let next_move: f64 = match next_move.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        let i = ((round::ceil(next_move/3.0, 0)) as usize) - 1;
        let j = ((next_move as usize)-1)%3;
        println!("[{},{}]", i, j);
        board[i][j] = 1;
        print_board(&board);
        //check for win
        let winning_player = check_for_win(&board);
        if winning_player > 0{
            if winning_player == 1{
                println!("you win!");
            }else if winning_player == 2{
                println!("you lose :(");
            }else{
                println!("that's a tie");
            }
            break;
        }

        //ai move
        ai_move(&mut board);
        print_board(&board);

        //check for win
        let winning_player = check_for_win(&board);
        if winning_player > 0{
            if winning_player == 1{
                println!("you win!");
            }else if winning_player == 2{
                println!("you lose :(");
            }else{
                println!("that's a tie");
            }
            break;
        }

    }
}


