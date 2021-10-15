use chess::{Board, ChessMove};

use crate::bm::bm_eval::{eval::Evaluation, evaluator::StdEvaluator};

#[derive(Debug, Clone)]
pub struct Position {
    board: Vec<Board>,
    evaluator: StdEvaluator,
}

//TODO: Counting Bloom Filter for threefold repetition detection
impl Position {
    pub fn new(board: Board) -> Self {
        let evaluator = StdEvaluator::new();
        Self {
            board: vec![board],
            evaluator,
        }
    }

    #[inline]
    pub fn three_fold_repetition(&self) -> bool {
        let hash = self.hash();
        self.board
            .iter()
            .rev()
            .skip(1)
            .filter(|board| board.get_hash() == hash)
            .count()
            >= 2
    }

    #[inline]
    pub fn board(&self) -> &Board {
        &self.board[self.board.len() - 1]
    }

    #[inline]
    pub fn null_move(&mut self) -> bool {
        if let Some(new_board) = self.board().null_move() {
            self.board.push(new_board);
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn make_move(&mut self, make_move: ChessMove) {
        let old_board = *self.board();
        let board = old_board.make_move_new(make_move);
        self.board.push(board);
    }

    #[inline]
    pub fn unmake_move(&mut self) {
        self.board.pop();
    }

    #[inline]
    pub fn hash(&self) -> u64 {
        self.board().get_hash()
    }

    pub fn get_eval(&mut self) -> Evaluation {
        let board = *self.board();
        self.evaluator.evaluate(&board)
    }
}
