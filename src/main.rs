mod board;
use board::{Board, InsertionError, MAX_GAME_SCORE};
use std::io;
use std::io::Write;
use std::isize;
use std::{thread, time};

static mut COUNTER: usize = 0;

fn main() {
    let mut board = Board::new();
    clear_term();
    let MAX_DEPTH = get_user_input("Enter the depth for minimax with α/β pruning (10 recommended): ", 2, 25);
    clear_term();
    println!("{}", board);
    loop {
        // Uncomment for AI vs AI
        /*let (value, best) = negamax(
            &board,
            MAX_DEPTH,
            -MAX_GAME_SCORE,
            MAX_GAME_SCORE,
            board.current_turn.as_int(),
        );
        println!("{}, {}", value, best);
        io::stdout().flush().unwrap();*/

        // Comment this loop for AI vs AI
        'input: loop {
            let col = get_user_input("Enter column: ", 1, 7);
            match board.insert_at(col) {
                Err(InsertionError::FilledSlot) => {
                    println!("That slot is full! Try again!");
                }
                _ => break,
            };
        }

        if update(&board) {
            break;
        }

        unsafe {
        COUNTER = 0;
        }
        let (value, best) = negamax(
            &board,
            MAX_DEPTH,
            -MAX_GAME_SCORE,
            MAX_GAME_SCORE,
            board.current_turn.as_int(),
        );
        println!("{}, {}", value, best);
        unsafe {
        println!("COUNTER: {}", COUNTER);
        }
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(1500));
        board.insert_at(best).unwrap();

        if update(&board) {
            break;
        }
    }
}

fn negamax(node: &Board, depth: usize, alpha: isize, beta: isize, color: isize) -> (isize, usize) {
    unsafe {
        COUNTER += 1;
    }
    if depth == 0 || node.outcome.is_some() {
        let r = node.get_total_score();
        return (r*color, 1);
    }

    let mut value = -MAX_GAME_SCORE+1;
    let mut best_move = 1;
    let mut alpha = alpha;
    for action in node.get_valid_moves() {
        let mut child = node.clone();
        child.insert_at(action).unwrap();
        let (mut temp, _) = negamax(&child, depth - 1, -beta, -alpha, -color);
        temp = -temp;
        value = std::cmp::max(value, temp);
        /*if temp >= value {
            value = temp;
            best_move = action;
        }*/
        if value > alpha {
            alpha = value;
            best_move = action;
        }

        if alpha >= beta {
            break;
        }
    }

    (value, best_move)
}

fn get_user_input(message: &str, lower: usize, upper: usize) -> usize {
    loop {
        print!("{}", message);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<usize>() {
            Ok(num) => {
                if !(lower..=upper).contains(&num) {
                    println!("Out of range! Try again.");
                    continue;
                } else {
                    return num;
                }
            }
            Err(_) => {
                println!("Invalid input! Try again.");
                continue;
            }
        }
    }
}

fn update(board: &Board) -> bool {
    clear_term();
    println!("{}", board);
    println!("Turn: {}", board.current_turn);
    if let Some(outcome) = &board.outcome {
        match outcome {
            board::GameOutcome::Winner(piece) => {
                println!("Winner is: {}", piece);
            }
            board::GameOutcome::Draw => {
                println!("The game was a draw");
            }
        }
        return true;
    }
    false
}


fn clear_term() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

