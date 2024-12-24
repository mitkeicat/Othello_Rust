// Othello game main

use std::fmt;
use std::io;
use std::io::Write;
use std::time::{Duration, Instant};

mod bit_lib; // file name
use crate::bit_lib::{can_locate, get_rev_pat}; // bit functions
use crate::bit_lib::{make_index, print_leagl_moves};
mod alpha_beta; // file name
use crate::alpha_beta::alphabeta;
mod board; // file name
use crate::board::Board;
mod monte_carlo;
use crate::monte_carlo::{mcts, primitive_montecarlo};

const BOARD_SIZE: usize = 8;
const BOARDSIZE_SQ: usize = BOARD_SIZE * BOARD_SIZE;
const BLACK_0: u64 = 0x0000_0008_1000_0000;
const WHITE_0: u64 = 0x0000_0010_0800_0000;

//#これで黒6石勝ち --> black 10 win
const BLACK_1: u64 = 0x6000_1008_1012_0500;
const WHITE_1: u64 = 0x8EFC_EFF7_6F6D_3A3F;


#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum TurnPlayer {
    Black,
    White,
}
impl TurnPlayer {
    fn flip(&self) -> Self {
        match self {
            TurnPlayer::Black => TurnPlayer::White,
            TurnPlayer::White => TurnPlayer::Black,
        }
    }
}
impl fmt::Display for TurnPlayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Black => write!(f, "Black"),
            Self::White => write!(f, "White"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Pmove {
    Pvmove(u64),
    Passmove,
    Endnode,
    Nonenode,
    Quit,
}

#[derive(Clone, Debug, PartialEq)]
enum PlayMode {
    Black,
    White,
    Pcpc,
    HumanVsHuman,
    PcpcMult,
    Test,
}
struct Play {
    play_mode: PlayMode,
    board: Board,
    com_black: fn(u64, u64, usize) -> Pmove,
    com_white: fn(u64, u64, usize) -> Pmove,
    win1f: u32,
    lose1f: u32,
    draw1f: u32,
}
impl Play {
    fn new() -> Self {
        let pc1: String = "(BLACK:".to_string();
        let pc2: String = "(White:".to_string();
        let msg = "対戦モードを選択してください:
  1:黒(先手) 2:白（後手）3:PC vs PC 4:Human vs Human 5:Mult 6:Test";
        let input = Play::select_input(msg, 6, false);
        let (play_mode, pc1, pc2) = match input {
            1 => (PlayMode::Black, pc1 + "Human", pc2),
            2 => (PlayMode::White, pc1, pc2 + "Human"),
            3 => (PlayMode::Pcpc, pc1, pc2),
            4 => (PlayMode::HumanVsHuman, pc1 + "Human", pc2 + "Human"),
            5 => (PlayMode::PcpcMult, pc1, pc2),
            _ => (PlayMode::Test, pc1, pc2),
        };
        let black_name:&str;
        let white_name:&str;
        let meg =
            "X(Black) のアルゴリズムの選択 (1: alpha_beta, 2: MCTS, 3: primitive_montecarlo) :";
        let input = Play::select_input(
            meg, 4, play_mode == PlayMode::Black || play_mode == PlayMode::HumanVsHuman);
        let com_black = match input {
            1 => {
                black_name = "alphabeta)";
                alphabeta
            }
            2 => {
                black_name = "MCTS)";
                mcts
            }
            3 => {
                black_name = "primitive_montecarlo)";
                primitive_montecarlo
            }
            _ => {
                black_name = ")";
                alphabeta
            }
        };
        let meg =
            "O(White) のアルゴリズムの選択 (1: alpha_beta, 2: MCTS, 3: primitive_montecarlo) :";
        let input = Play::select_input(
            meg, 4, play_mode == PlayMode::White || play_mode == PlayMode::HumanVsHuman);
        let com_white = match input {
            1 => {
                white_name = "alphabeta)";
                alphabeta
            }
            2 => {
                white_name = "MCTS)";
                mcts
            }
            3 => {
                white_name = "primitive_montecarlo)";
                primitive_montecarlo
            }
            _ => {
                white_name = ")";
                alphabeta
            }
        };
        let _com_a = alphabeta;
        let _com_b = mcts;
        let _com_c = primitive_montecarlo;

        let f_player = pc1 + black_name ;
        let s_player = pc2 + white_name ;
        println!(" set  com_black  {}", f_player);
        println!(" set  com_white  {}", s_player);
        let mut board = Board::new(
            (TurnPlayer::Black, f_player.clone(), s_player.clone()),
            BLACK_0,
            WHITE_0,
        );
        if play_mode == PlayMode::Test {
            board = Board::new((TurnPlayer::Black, f_player, s_player), BLACK_1, WHITE_1);
        }
        Play {
            play_mode,
            board,
            com_black,
            com_white,
            win1f: 0,
            lose1f: 0,
            draw1f: 0,
        }
    }

    fn end_game(&mut self) -> bool {
        if self.board.is_can_play() {
            return false;
        }
        let black_disk = self.board.black.count_ones();
        let white_disk = self.board.white.count_ones();
        let final_score = black_disk as i32 - white_disk as i32;
        let winner = match final_score {
            x if x > 0 => {
                self.win1f += 1;
                format!("先手{}", self.board.first_player)
            },
            x if x < 0 => {
                self.lose1f += 1;
                format!("後手{}", self.board.second_player)
            },
            _ => {
                self.draw1f += 1;
                "DRAW".to_string()
            }
        };
        if final_score != 0 {
            println!("{}の {} 石勝ちです", winner, final_score.abs());
        } else {
            println!("{} :引き分けです", winner);
        }
        true
    }

    fn human_input(&self) -> Pmove {
        fn make_pmove(x: usize, y: usize) -> u64 {
            0x8000000000000000 >> (x + y * 8)
        }
        let player_str = match self.board.turn_player {
            TurnPlayer::Black => "X:BLACK",
            TurnPlayer::White => "O:WHITE",
        };
        println!(
            "あなたの番です({}), 次の手を入力してください (q:quit p:pass)",
            player_str
        );
        print_leagl_moves(self.board.first, self.board.second);
        loop {
            let mut buffer = String::new();
            io::stdin()
                .read_line(&mut buffer)
                .expect("Failed to read line");
            let buffer = buffer.trim();
            match buffer {
                "q" | "quit" => {
                    println!("ゲームを終了します");
                    return Pmove::Quit;
                }
                "p" | "pass" => {
                    if can_locate(self.board.first, self.board.second) == 0 {
                        println!("パスです");
                        return Pmove::Passmove;
                    } else {
                        println!("パスはできません");
                    }
                }
                _ if buffer.len() == 2 => {
                    let x = buffer.chars().nth(0).unwrap().to_ascii_lowercase();
                    let y = buffer.chars().nth(1).unwrap();
                    if let (Some(x), Some(y)) = ("abcdefgh".find(x), "12345678".find(y)) {
                        let mv = make_pmove(x, y);
                        if get_rev_pat(self.board.first, self.board.second, mv) == 0 {
                            println!("そこには置けません");
                        } else {
                            return Pmove::Pvmove(mv);
                        }
                    } else {
                        println!("無効なコマンドです");
                    }
                }
                _ => println!("無効なコマンドです"),
            }
        }
    }

    fn com_think(&mut self) -> Pmove {
        if self.play_mode != PlayMode::PcpcMult {
            println!("{} コンピュータ思考中...", self.board.turn_player);
        };
        let pm = if self.board.turn_player == TurnPlayer::Black {
            (self.com_black)(self.board.first, self.board.second, self.board.left)
        } else {
            (self.com_white)(self.board.first, self.board.second, self.board.left)
        };
        if self.play_mode != PlayMode::PcpcMult {
            match pm {
                Pmove::Pvmove(mv) => println!("PC の着手： {}", make_index(mv)),
                Pmove::Passmove => println!("パスします"),
                _ => {}
            }
        }
        pm
    }

    fn loop_game(&mut self) {
        if self.play_mode != PlayMode::PcpcMult {
            loop {
                self.board.print_board();
                if self.end_game() {
                    break;
                }
                if self.play_mode == PlayMode::Test {
                    print!("q is quit: ");
                    io::stdout().flush().unwrap();
                    let mut buffer = String::new();
                    io::stdin()
                        .read_line(&mut buffer)
                        .expect("Failed to read line");
                    if buffer.trim() == "q" {
                        return;
                    }
                }
                let pmove = if (self.board.turn_player == TurnPlayer::Black
                    && self.play_mode == PlayMode::Black)
                    || (self.board.turn_player == TurnPlayer::White
                        && self.play_mode == PlayMode::White)
                    || self.play_mode == PlayMode::HumanVsHuman
                {
                    self.human_input()
                } else {
                    self.com_think()
                };
                if let Pmove::Quit = pmove {
                    return;
                }
                self.board.update_board(pmove);
            }
        } else {
            for game_count in 0..50 {
                let start = Instant::now();

                self.board = Board::new(
                    (
                        TurnPlayer::Black,
                        self.board.first_player.clone(),
                        self.board.second_player.clone(),
                    ),
                    BLACK_0,
                    WHITE_0,
                );
                loop {
                    if self.end_game() {
                        break;
                    }
                    let pmove = self.com_think();
                    self.board.update_board(pmove);
                }
                println!(
                    " 先手： {} 勝  後手： {} 勝  引き分け： {}  試合数： {}",
                    self.win1f,
                    self.lose1f,
                    self.draw1f,
                    game_count + 1
                );
                let duration: Duration = start.elapsed();
                println!("Time elapsed in alphabet is: {:?}", duration);
            }
        }
    }
    fn select_input(msg: &str, max_number: usize, human: bool) -> usize {
        if human { return 0;}
        loop {
            println!("{msg}");
            let mut buffer = String::new();
            io::stdin()
                .read_line(&mut buffer)
                .expect("Failed to read line");
            let input = match buffer.trim().parse::<usize>() {
                Ok(n) if 0 < n && n <= max_number => n,
                Ok(_) => {
                    println!(
                        "無効な入力です。1から{}までの整数を入力してください。",
                        max_number
                    );
                    continue;
                }
                Err(_) => {
                    println!("数値を入力してください。");
                    continue;
                }
            };

            return input;
        }
    }
}

fn main() {
    println!("Othello Reversi program ");
    let mut play = Play::new();
    play.loop_game();
}
