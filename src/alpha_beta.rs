// library AI for Othello

// from  bit_lib
use crate::bit_lib::{can_locate, get_rev_pat}; // bit functions
use crate::{Pmove, BOARDSIZE_SQ}; // from main
const READING_LEVEL: i32 = 8;
const COMPLETE_READING_LEVEL: usize = 14;
// 評価テーブル
// table 6 - Self made
#[rustfmt::skip]
const TABLE6: [i32; BOARDSIZE_SQ] = [
     50,  -24,   8,   6,   6,   8,  -24,  50,
    -24,  -13,   1,   1,   1,   1,  -13, -24,
      8,    1,   0,  -1,  -1,   0,    1,   8,
      6,    1,  -1,  -1,  -1,  -1,    1,   6,
      6,    1,  -1,  -1,  -1,  -1,    1,   6,
      8,    1,   0,  -1,  -1,   0,    1,   8,
    -24,  -13,   1,   1,   1,   1,  -13, -24,
     50,  -24,   8,   6,   6,   8,  -24,  50,
];
/*
// 評価テーブル
// table 1 - 隅を重視する
//#P. Novig: "Paradigms of Artificial Intelligence Programming,"
//#Case Studies in Common Lisp, Morgan Kaufmann, 1992
const TABLE1: [i32; BOARDSIZE_SQ] = [
    120,  -20,  20,   5,   5,  20,  -20, 120,
    -20,  -40,  -5,  -5,  -5,  -5,  -40, -20,
     20,   -5,  15,   3,   3,  15,   -5,  20,
      5,   -5,   3,   3,   3,   3,   -5,   5,
      5,   -5,   3,   3,   3,   3,   -5,   5,
     20,   -5,  15,   3,   3,  15,   -5,  20,
    -20,  -40,  -5,  -5,  -5,  -5,  -40, -20,
    120,  -20,  20,   5,   5,  20,  -20, 120,
];

// table 2 - 相手に囲ませる 石を多く取らない
const TABLE2: [i32; BOARDSIZE_SQ] = [
     30,  -12,   0,  -1,  -1,   0,  -12,  30,
    -12,  -15,  -3,  -3,  -3,  -3,  -15, -12,
      0,   -3,   0,  -1,  -1,   0,   -3,   0,
     -1,   -3,  -1,  -1,  -1,  -1,   -3,  -1,
     -1,   -3,  -1,  -1,  -1,  -1,   -3,  -1,
      0,   -3,   0,  -1,  -1,   0,   -3,   0,
    -12,  -15,  -3,  -3,  -3,  -3,  -15, -12,
     30,  -12,   0,  -1,  -1,   0,  -12,  30,
];

// table 3 - Stage 1
const TABLE3: [i32; BOARDSIZE_SQ] = [
    500,  -86,  96,  26,  26,  96,  -86, 500,
    -86, -1219,  -6,   0,   0,  -6, -1219, -86,
     96,   -6,  52,  15,  15,  52,   -6,  96,
     26,    0,  15, -17, -17,  15,    0,  26,
     26,    0,  15, -17, -17,  15,    0,  26,
     96,   -6,  52,  15,  15,  52,   -6,  96,
    -86, -1219,  -6,   0,   0,  -6, -1219, -86,
    500,  -86,  96,  26,  26,  96,  -86, 500,
];

// table 4 - Stage 2
const TABLE4: [i32; BOARDSIZE_SQ] = [
    500, -240,  85,  69,  69,  85, -240, 500,
   -240, -130,  49,  23,  23,  49, -130, -240,
     85,   49,   1,   9,   9,   1,   49,  85,
     69,   23,   9,  32,  32,   9,   23,  69,
     69,   23,   9,  32,  32,   9,   23,  69,
     85,   49,   1,   9,   9,   1,   49,  85,
   -240, -130,  49,  23,  23,  49, -130, -240,
    500, -240,  85,  69,  69,  85, -240, 500,
];

// table 6 - Self made
const TABLE6: [i32; BOARDSIZE_SQ] = [
     50,  -24,   8,   6,   6,   8,  -24,  50,
    -24,  -13,   1,   1,   1,   1,  -13, -24,
      8,    1,   0,  -1,  -1,   0,    1,   8,
      6,    1,  -1,  -1,  -1,  -1,    1,   6,
      6,    1,  -1,  -1,  -1,  -1,    1,   6,
      8,    1,   0,  -1,  -1,   0,    1,   8,
    -24,  -13,   1,   1,   1,   1,  -13, -24,
     50,  -24,   8,   6,   6,   8,  -24,  50,
];
*/
#[rustfmt::skip]
const EDGE_DATA1: [u64; 52] = [
    0x0000000000000007,  0x000000000000000f,  0x000000000000001f,
    0x000000000000003f,  0x000000000000007f,  0x00000000000000e0,
    0x00000000000000f0,  0x00000000000000f8,  0x00000000000000fc,
    0x00000000000000fe,  0x00000000000000ff,  0x0000000000010101,
    0x0000000001010101,  0x0000000101010101,  0x0000010101010101,
    0x0001010101010101,  0x0101010101010101,  0x0101010000000000,
    0x0101010100000000,  0x0101010101000000,  0x0101010101010000,
    0x0101010101010100,  0x0700000000000000,  0x0f00000000000000,
    0x1f00000000000000,  0x3f00000000000000,  0x7f00000000000000,
    0xff00000000000000,  0xe000000000000000,  0xf000000000000000,
    0xf800000000000000,  0xfc00000000000000,  0xfe00000000000000,
    0x0000000000808080,  0x0000000080808080,  0x0000008080808080,
    0x0000808080808080,  0x0080808080808080,  0x8080808080808080,
    0x8080800000000000,  0x8080808000000000,  0x8080808080000000,
    0x8080808080800000,  0x8080808080808000,  0x0000000000000303,
    0x000000000000c0c0,  0x0303000000000000,  0xc0c0000000000000,
    0x0102040000000000,  0x0000000000040201,  0x0000000000204080,
    0x8040200000000000,
];

const MASK3: u64 = 0xffff_ffff_ffff_ffff;
const MASKS: [u64; 8] = [
    0x7f7f_7f7f_7f7f_7f7f,
    0xfefe_fefe_fefe_fefe,
    0x00ff_ffff_ffff_ffff,
    0xffff_ffff_ffff_ff00,
    0x00fe_fefe_fefe_fefe,
    0x7f7f_7f7f_7f7f_7f00,
    0x007f_7f7f_7f7f_7f7f,
    0xfefe_fefe_fefe_fe00,
];

// 評価関数の統合 Integration of evaluation functions
fn eval_all(black: u64, white: u64, left: usize) -> i32 {
    if left < 20 {
        2 * eval_stones(black, white)
    } else {
        2 * eval_stones(black, white)
            + 2 * eval_table(black, white, &TABLE6)
            + 2 * eval_move(black, white)
            + 2 * eval_edge(black, white)
            + 2 * eval_open(black, white)
    }
}

fn alphabeta_s(
    black: u64,
    white: u64,
    depth: i32,
    mut alpha: i32,
    beta: i32,
    left: usize,
) -> (i32, Pmove) {
    let mut action: Pmove = Pmove::Nonenode;
    let mut score: i32;
    if depth <= 0 {
        return (eval_all(black, white, left), Pmove::Endnode);
    }
    let mut can_move_point = can_locate(black, white);
    if can_move_point == 0 {
        if (black).count_ones() == 0 {
            return (i32::MIN + 10, Pmove::Endnode); // black is zero, game end
        }
        if can_locate(white, black) == 0 {
            return (0x1000 * eval_all(black, white, left), Pmove::Endnode); //
        } else {
            // pass
            (score, _) = alphabeta_s(white, black, depth, -beta, -alpha, left);
        } //# 深さは手数が進まないのだから変わらない
        return (-score, action);
    } else {
        while can_move_point != 0 {
            let put_posi = can_move_point & (!can_move_point + 1); //  一番右のビットのみ取り出す

            let rever_pat = get_rev_pat(black, white, put_posi); // 反転するパターン、着手マス
            (score, _) = alphabeta_s(
                white ^ rever_pat,
                black ^ (put_posi | rever_pat),
                depth - 1,
                -beta,
                -alpha,
                left - 1,
            );
            // when - i32::MIN overflow
            let (_, error_f) = score.overflowing_neg();
            score = if error_f { i32::MAX - 1 } else { -score }; //score = - score;
            can_move_point ^= put_posi; //# 一番右のビットをOFFにする
            if score > alpha {
                //αカット
                alpha = score;
                action = Pmove::Pvmove(put_posi);
            }
            if alpha >= beta {
                return (alpha, action);
            }
        }
    }
    (alpha, action)
}

pub fn alphabeta(black: u64, white: u64, left: usize) -> Pmove {
    let depth: i32 = if left < COMPLETE_READING_LEVEL {
        COMPLETE_READING_LEVEL as i32
    } else {
        READING_LEVEL
    };
    alphabeta_s(black, white, depth, i32::MIN + 10, i32::MAX - 10, left).1
}

// 石の枚数の差による評価　Evaluation based on the number of Othello stones
fn eval_stones(black: u64, white: u64) -> i32 {
    black.count_ones() as i32 - white.count_ones() as i32
}

// テーブル方式の評価関数 Table-based evaluation function
fn eval_table(black1: u64, white1: u64, &table: &[i32; 64]) -> i32 {
    table
        .iter()
        .enumerate()
        .map(|(i, &value)| {
            if (black1 >> i) & 0x01 != 0 {
                value
            } else if (white1 >> i) & 0x01 != 0 {
                -value
            } else {
                0
            }
        })
        .sum()
    //let mut black_score = 0;
    //let mut white_score = 0;
    //for i in 0..BOARDSIZE_SQ {
    //    if (black1 >> i) & 0x01 != 0 {
    //        black_score += table[i];
    //    }
    //    if (white1 >> i) & 0x01 != 0 {
    //        white_score += table[i];
    //    }
    //}
    //black_score - white_score
}

//静的評価関数 着手可能数
fn eval_move(black: u64, white: u64) -> i32 {
    can_locate(black, white).count_ones() as i32 - can_locate(white, black).count_ones() as i32
}

// エッジパターン評価関数
fn eval_edge(black: u64, white: u64) -> i32 {
    let mut black_sum = 0;
    let mut white_sum = 0;
    for &mask in EDGE_DATA1.iter() {
        if (black & mask) == mask {
            black_sum += 1;
        }
        if (white & mask) == mask {
            white_sum += 1;
        }
    }
    black_sum - white_sum
}

// #開放度 を計算する関数  Function to calculate openness
fn eval_open(black: u64, white: u64) -> i32 {
    let bw = (black | white) ^ MASK3; // ビット反転、開いているマス目
    let count_sub = |pieces: u64| -> i32 {
        let mut sum1 = 0; // 黒石の隣は空いているか？
        sum1 += (pieces >> 1 & bw & MASKS[0]).count_ones() as i32; // 1bit
        sum1 += (pieces << 1 & bw & MASKS[1]).count_ones() as i32;
        sum1 += (pieces >> 8 & bw & MASKS[2]).count_ones() as i32; // 8bit
        sum1 += (pieces << 8 & bw & MASKS[3]).count_ones() as i32;
        sum1 += (pieces >> 7 & bw & MASKS[4]).count_ones() as i32; // 7bit
        sum1 += (pieces << 7 & bw & MASKS[5]).count_ones() as i32;
        sum1 += (pieces >> 9 & bw & MASKS[6]).count_ones() as i32; // 9bit
        sum1 += (pieces << 9 & bw & MASKS[7]).count_ones() as i32;
        sum1
    };
    // 開放度は少ない方が良い Less openness is better
    count_sub(white) - count_sub(black)
}
