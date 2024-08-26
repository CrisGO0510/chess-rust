use crate::chessboard::{
    chessboard::Chessboard,
    utilities::{calculate_score, get_all_pieces, is_valid_move},
};

use super::piece_actions::PieceActions;

fn minimax(
    chessboard: &Chessboard,
    depth: i32,
    mut alpha: f32,
    mut beta: f32,
    maximizing_player: bool,
) -> (f32, Option<([usize; 2], [usize; 2])>) {
    if depth == 0 {
        return (calculate_score(chessboard) as f32, None);
    }

    let all_pieces = get_all_pieces(chessboard, chessboard.player_turn);

    if maximizing_player {
        let mut max_eval = f32::NEG_INFINITY;
        let mut best_move = None;

        for piece in all_pieces {
            let allowed_moves = piece.allowed_moves();

            for move_position in allowed_moves {
                let mut temp_chessboard = chessboard.clone();
                let move_result_message =
                    temp_chessboard.move_piece(piece.position, move_position, true);

                if !is_valid_move(&move_result_message) {
                    continue;
                }

                let (eval, _) = minimax(&temp_chessboard, depth - 1, alpha, beta, false);

                if eval > max_eval {
                    max_eval = eval;
                    best_move = Some((piece.position, move_position));
                    alpha = alpha.max(max_eval);
                    if beta <= alpha {
                        break;
                    }
                }
            }
        }
        (max_eval, best_move)
    } else {
        let mut min_eval = f32::INFINITY;
        let mut best_move = None;

        for piece in all_pieces {
            let allowed_moves = piece.allowed_moves();

            for move_position in allowed_moves {
                let mut temp_chessboard = chessboard.clone();
                let move_result_message =
                    temp_chessboard.move_piece(piece.position, move_position, true);

                if !is_valid_move(&move_result_message) {
                    continue;
                }

                let (eval, _) = minimax(&temp_chessboard, depth - 1, alpha, beta, true);
                println!(
                    "Podando rama2, profundidad {}, alpha {}, beta {}, score {}, best_move {:?}, {:?}, piece {}, message {:?}",
                    depth, alpha, beta, eval, piece.position, move_position, piece.to_char(), move_result_message
                );
                if eval < min_eval {
                    min_eval = eval;
                    best_move = Some((piece.position, move_position));
                    beta = beta.min(min_eval);

                    if beta <= alpha {
                        break;
                    }
                }
            }
        }
        (min_eval, best_move)
    }
}

pub fn get_best_move(chessboard: &Chessboard, depth: i32) -> ([usize; 2], [usize; 2]) {
    let (_, best_move) = minimax(chessboard, depth, f32::NEG_INFINITY, f32::INFINITY, false);

    best_move.expect("No valid move found")
}
