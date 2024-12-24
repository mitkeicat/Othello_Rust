// Bit functions for Othello


use crate::BOARD_SIZE;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}
#[allow(dead_code)]
pub fn get_function_name<F>(_: F) -> &'static str {
    std::any::type_name::<F>()
}
#[allow(dead_code)]
pub fn print_bits(bits: u64) {
    println!();
    println!("   A B C D E F G H");
    for i in 0..BOARD_SIZE {
        print!("{:2} ", i + 1);
        let b = (bits >> (56 - i * 8)) & 0xff;
        for j in 0..8 {
            if (b >> (7 - j)) & 0x01 != 0 {
                print!("O ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
    println!("bits= {:x}", bits);
}

pub fn print_leagl_moves(my_pieces: u64, en_pieces: u64) {
    let moves = make_legal_actions(my_pieces, en_pieces);
    print!("着手可能点:");
    for mv in moves {
        print!("{} ", make_index(mv));
    }
    println!();
}

pub fn make_legal_actions(my_pieces: u64, en_pieces: u64) -> Vec<u64> {
    let mut actions = Vec::new();
    let mut can_moves = can_locate(my_pieces, en_pieces);
    while can_moves != 0 {
        let put_posi = can_moves & (!can_moves + 1);
        actions.push(put_posi);
        can_moves ^= put_posi;
    }
    actions
}

pub fn make_index(mv: u64) -> String {
    if mv == 0 {
        return "Pa".to_string();
    }
    let mut n_out: usize = 0;
    let mut ab_out: usize = 0;
    for i in 0..8 {
        if (mv >> (56 - i * 8)) & 0xff != 0 {
            n_out = i;
        }
        if (mv >> i) & 0x0101010101010101 != 0 {
            ab_out = i;
        }
    }
    let n_out_char = "12345678".chars().nth(n_out).unwrap();
    let ab_out_char = "ABCDEFGH".chars().nth(7 - ab_out).unwrap();
    format!("{}{}", ab_out_char, n_out_char)
}

// Convert each bit position on the board using a hash function
// for De Bruijn sequence m-sequence;maximal length sequence
//#hashNum = 0x03F566ED27179461
#[allow(dead_code)]
fn make_hash() -> HashMap<u64, i32> {
    let mut table: HashMap<u64, i32> = HashMap::new();
    let mut hash_num: u64 = 0x03F566ED27179461;
    for i in (0..64).rev() {
        table.insert(hash_num >> 58, i);
        hash_num <<= 1;
    }
    table
}
// from action (only one bit) to index (int)
#[allow(dead_code)]
fn action_to_index(action: u64, table: &HashMap<u64, i32>) -> i32 {
    if action == 0 {
        return 65;
    }
    let i = (action * 0x03F566ED27179461) >> 58;
    match table.get(&i) {
        Some(&index) => index,
        None => 65,
    }
}

pub fn get_rev_pat(black: u64, white: u64, mv: u64) -> u64 {
    let mut rev = 0;
    if (black | white) & mv != 0 {
        return rev;
    }

    let mut rev0;
    let mut mask;

    // 各方向の処理
    // 右方向
    rev0 = 0;
    mask = (mv >> 1) & 0x7f7f7f7f7f7f7f7f;
    while mask != 0 && (mask & white) != 0 {
        rev0 |= mask;
        mask = (mask >> 1) & 0x7f7f7f7f7f7f7f7f;
    }
    if (mask & black) == 0 {
        rev0 = 0;
    }
    rev |= rev0;

    // 左方向
    rev0 = 0;
    mask = (mv << 1) & 0xfefefefefefefefe;
    while mask != 0 && (mask & white) != 0 {
        rev0 |= mask;
        mask = (mask << 1) & 0xfefefefefefefefe;
    }
    if (mask & black) == 0 {
        rev0 = 0;
    }
    rev |= rev0;

    // 下方向
    rev0 = 0;
    mask = (mv << 8) & 0xffffffffffffff00;
    while mask != 0 && (mask & white) != 0 {
        rev0 |= mask;
        mask = (mask << 8) & 0xffffffffffffff00;
    }
    if (mask & black) == 0 {
        rev0 = 0;
    }
    rev |= rev0;

    // 上方向
    rev0 = 0;
    mask = (mv >> 8) & 0x00ffffffffffffff;
    while mask != 0 && (mask & white) != 0 {
        rev0 |= mask;
        mask = (mask >> 8) & 0x00ffffffffffffff;
    }
    if (mask & black) == 0 {
        rev0 = 0;
    }
    rev |= rev0;

    // 右上方向
    rev0 = 0;
    mask = (mv << 7) & 0x7f7f7f7f7f7f7f7f;
    while mask != 0 && (mask & white) != 0 {
        rev0 |= mask;
        mask = (mask << 7) & 0x7f7f7f7f7f7f7f7f;
    }
    if (mask & black) == 0 {
        rev0 = 0;
    }
    rev |= rev0;

    // 左下方向
    rev0 = 0;
    mask = (mv >> 7) & 0xfefefefefefefefe;
    while mask != 0 && (mask & white) != 0 {
        rev0 |= mask;
        mask = (mask >> 7) & 0xfefefefefefefefe;
    }
    if (mask & black) == 0 {
        rev0 = 0;
    }
    rev |= rev0;

    // 左上方向
    rev0 = 0;
    mask = (mv << 9) & 0xfefefefefefefefe;
    while mask != 0 && (mask & white) != 0 {
        rev0 |= mask;
        mask = (mask << 9) & 0xfefefefefefefefe;
    }
    if (mask & black) == 0 {
        rev0 = 0;
    }
    rev |= rev0;

    // 右下方向
    rev0 = 0;
    mask = (mv >> 9) & 0x7f7f7f7f7f7f7f7f;
    while mask != 0 && (mask & white) != 0 {
        rev0 |= mask;
        mask = (mask >> 9) & 0x7f7f7f7f7f7f7f7f;
    }
    if (mask & black) == 0 {
        rev0 = 0;
    }
    rev |= rev0;

    rev
}

pub fn can_locate(black: u64, white: u64) -> u64 {
    let mut w: u64;
    let mut t: u64;
    let mut blank: u64;
    let mut mobility: u64;
    let mut shift: i32 = 7;

    // mask0
    w = white & 0x007e7e7e7e7e7e00;

    t = w & (black >> shift);
    t |= w & (t >> shift);
    t |= w & (t >> shift);
    t |= w & (t >> shift);
    t |= w & (t >> shift);
    t |= w & (t >> shift);
    blank = !(black | white);
    mobility = blank & (t >> shift);

    t = w & (black << shift);
    t |= w & (t << shift);
    t |= w & (t << shift);
    t |= w & (t << shift);
    t |= w & (t << shift);
    t |= w & (t << shift);
    mobility |= blank & (t << shift);

    // Check_Up(black, white, 9, MASK0)
    w = white & 0x007e7e7e7e7e7e00;
    shift = 9;

    t = w & (black >> shift);
    t |= w & (t >> shift);
    t |= w & (t >> shift);
    t |= w & (t >> shift);
    t |= w & (t >> shift);
    t |= w & (t >> shift);
    blank = !(black | white);
    mobility |= blank & (t >> shift);

    t = w & (black << shift);
    t |= w & (t << shift);
    t |= w & (t << shift);
    t |= w & (t << shift);
    t |= w & (t << shift);
    t |= w & (t << shift);
    mobility |= blank & (t << shift);

    // Check_Up(black, white, 8, MASK2)
    w = white & 0x00ffffffffffff00;
    shift = 8;

    t = w & (black >> shift);
    t |= w & (t >> shift);
    t |= w & (t >> shift);
    t |= w & (t >> shift);
    t |= w & (t >> shift);
    t |= w & (t >> shift);
    blank = !(black | white);
    mobility |= blank & (t >> shift);

    t = w & (black << shift);
    t |= w & (t << shift);
    t |= w & (t << shift);
    t |= w & (t << shift);
    t |= w & (t << shift);
    t |= w & (t << shift);
    mobility |= blank & (t << shift);

    // Check_Up(black, white, 1, MASK1)
    w = white & 0x7e7e7e7e7e7e7e7e;
    shift = 1;

    t = w & (black >> shift);
    t |= w & (t >> shift);
    t |= w & (t >> shift);
    t |= w & (t >> shift);
    t |= w & (t >> shift);
    t |= w & (t >> shift);
    blank = !(black | white);
    mobility |= blank & (t >> shift);

    t = w & (black << shift);
    t |= w & (t << shift);
    t |= w & (t << shift);
    t |= w & (t << shift);
    t |= w & (t << shift);
    t |= w & (t << shift);
    mobility |= blank & (t << shift);

    mobility
}

#[cfg(test)]
mod tests {
    use super::*;
    const BLACK_0: u64 = 0x0000_0008_1000_0000;
    const WHITE_0: u64 = 0x0000_0010_0800_0000;
    
    #[test]
    fn test_typename() {
        print_typename(print_bits);
        println!("{}", get_function_name(print_bits));        
        assert_eq!(print_typename(print_bits), ());
        assert_eq!(get_function_name(print_bits), "Othello::bit_lib::print_bits");
    }

    #[test]
    fn test_get_rev_pat() {
        get_rev_pat(BLACK_0, WHITE_0, 0);
        let actions = make_legal_actions(BLACK_0, WHITE_0);
        assert_eq!(get_rev_pat(BLACK_0, WHITE_0, actions[0]), 0x800_0000);
    }
    #[test]
    fn test_can_locate() {
        assert_eq!(can_locate(BLACK_0, WHITE_0), 0x1020_0408_0000);
    }

}