// board lib for Othello

use crate::bit_lib::{can_locate, get_rev_pat};
use crate::{Pmove, TurnPlayer, BOARDSIZE_SQ}; // from main // from bit_lib

#[derive(Clone, Debug)]
pub struct Board {
    pub turn_player: TurnPlayer,
    pub black: u64,
    pub white: u64,
    pub first: u64,
    pub second: u64,
    pub first_player: String,
    pub second_player: String,
    step: u32,
    pub left: usize,
}

impl Board {
    pub fn new(t_player: (TurnPlayer, String, String), black: u64, white: u64) -> Self {
        // turn player black or white, player 1 name, player 2 name
        Board {
            turn_player: t_player.0, // turn player black or white
            black,
            white,
            first: black,
            second: white,
            first_player: t_player.1,  // player 1 name
            second_player: t_player.2, // player 2 name
            step: 1,
            left: BOARDSIZE_SQ - (black | white).count_ones() as usize,
        }
    }

    pub fn is_can_play(&self) -> bool {
        ! (can_locate(self.first, self.second) == 0 
            && can_locate(self.second, self.first) == 0)
    }

    pub fn update_board(&mut self, pmove: Pmove) {
        if let Pmove::Pvmove(mv) = pmove {
            let rever_pat = get_rev_pat(self.first, self.second, mv);
            self.put_stone(mv, rever_pat);
        }
        self.turn_player = match self.turn_player {
            TurnPlayer::Black => {
                self.first = self.white;
                self.second = self.black;
                TurnPlayer::White
            }
            TurnPlayer::White => {
                self.first = self.black;
                self.second = self.white;
                TurnPlayer::Black
            }
        };
        self.step += 1;
        self.left = BOARDSIZE_SQ - (self.black | self.white).count_ones() as usize;
    }

    pub fn print_board(&self) {
        println!();
        println!("----------------------------");
        println!("   A B C D E F G H");
        for i in 0..8 {
            print!("{:2} ", i + 1);
            let b = (self.black >> (56 - i * 8)) & 0xff;
            let w = (self.white >> (56 - i * 8)) & 0xff;
            let bw = b | w;
            for j in (0..8).rev() {
                if (bw >> j) & 0x01 != 0 {
                    if (b >> j) & 0x01 != 0 {
                        print!("X ");
                    } else {
                        print!("O ");
                    }
                } else {
                    print!(". ");
                }
            }
            println!();
        }
        println!(
            "X{:5}:{:2} vs O{:5}:{:2}",
            self.first_player,
            self.black.count_ones(),
            self.second_player,
            self.white.count_ones()
        );
    }

    fn put_stone(&mut self, put_posi: u64, rev_pat: u64) {
        if self.turn_player == TurnPlayer::Black {
            self.black ^= put_posi | rev_pat;
            self.white ^= rev_pat;
        } else {
            self.white ^= put_posi | rev_pat;
            self.black ^= rev_pat;
        }
    }
}
