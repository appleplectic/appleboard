use std::io;
use std::str::FromStr;
use chess::{Board, BoardStatus, ChessMove, MoveGen};

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    let board = Board::from_str(&input).unwrap();
    println!("{}", best_move(&board, 5).to_string())
}

fn get_heuristic(board: &Board) -> f64 {
    match board.status() {
        BoardStatus::Checkmate => {
            -10000f64
        }
        BoardStatus::Stalemate => 0f64,
        BoardStatus::Ongoing => 0f64
    }
}

fn best_move(board: &Board, depth: u8) -> ChessMove {
    let mut moves = MoveGen::new_legal(board);
    let mut best = (f64::NEG_INFINITY, ChessMove::default());

    for m in &mut moves {
        let new_board = board.make_move_new(m);
        let score = -negamax(&new_board, depth - 1, f64::NEG_INFINITY, f64::INFINITY,);

        if score > best.0 {
            best.0 = score;
            best.1 = m;
        }
    }

    best.1
}

fn negamax(node: &Board, depth: u8, mut alpha: f64, beta: f64) -> f64 {
    if depth == 0 || node.status() != BoardStatus::Ongoing {
        return get_heuristic(node);
    }

    let mut child_nodes = MoveGen::new_legal(node);
    let mut value = f64::NEG_INFINITY;

    for child in &mut child_nodes {
        let new_board = node.make_move_new(child);
        value = f64::max(value, -negamax(&new_board, depth-1, -beta, -alpha));
        alpha = f64::max(alpha, value);
        if alpha >= beta {
            break;
        }
    }
    value
}
