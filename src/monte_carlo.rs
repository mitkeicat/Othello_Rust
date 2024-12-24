// monte carlo library AI for Othello

use std::fmt;
use rand::seq::SliceRandom;
use rand::thread_rng;

// from  bit_lib
use crate::bit_lib::{can_locate, get_rev_pat, make_index}; // bit functions
use crate::{Pmove, TurnPlayer}; // from main

const THRESHOLD: u32 = 1;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum WinLose {
    Win,
    Lose,
    Draw,
    Undecided,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum NodeState {
    Pass,
    None,
    End,
    BelowTh,
    AboveTh,
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Node {
    turn_player: TurnPlayer,
    my_piece: u64,
    en_piece: u64,
    n_sa: u32, // visit count
    w_sa: u32, // total win value
    ucb1: f32,
    value: i32,
    step: u32,
    winlose: WinLose,        // win lose draw , undecided
    state: NodeState,        // pass none, end, below threshold, above threshold
    action: u64,             // move to this node
    legal_actions: Vec<u64>, // can move points
    child_nodes: Vec<Node>,
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Node {{turn_player: {:?},  my_piece: {},  en_piece: {},  step: {}, 
    n_sa: {},  w_sa: {},  ucb1: {:.4},  value: {},  winlose: {:?},
    state: {:?},  action: {},  legal_actions: {:?},  child_nodes_len: {}}}",
            self.turn_player,
            self.my_piece,
            self.en_piece,
            self.step,
            self.n_sa,
            self.w_sa,
            self.ucb1,
            self.value,
            self.winlose,
            self.state,
            make_index(self.action),
            self.legal_actions
                .iter()
                .map(|&x| make_index(x))
                .collect::<Vec<String>>(),
            self.child_nodes.len(),
        )
    }
}

impl Node {
    fn new(turn_player: TurnPlayer, my_piece: u64, en_piece: u64, action: u64, step1: u32) -> Self {
        Node {
            turn_player,
            my_piece,
            en_piece,
            n_sa: 0,
            w_sa: 0,
            ucb1: 0.0,
            step: step1 + 1,
            value: 0, // one time win lose draw 
            winlose: WinLose::Undecided,
            action,
            state: NodeState::None,
            legal_actions: make_legal_actions(my_piece, en_piece),
            child_nodes: Vec::new(),
        }
    }
    fn pass_node(&self) -> Self {
        Node::new(
            self.turn_player.flip(),
            self.en_piece,
            self.my_piece,
            0,
            self.step,
        )
    }
    fn next_node(&self, action: &u64) -> Node {
        let rev = get_rev_pat(self.my_piece, self.en_piece, *action);
        Node::new(
            self.turn_player.flip().clone(),
            self.en_piece ^ rev,
            self.my_piece ^ (action | rev),
            *action,
            self.step,
        )
    }
    fn ucb1_update(&mut self, all_n: u32) {
        self.ucb1 = self.w_sa as f32 / self.n_sa as f32
            + 2.0 * ((all_n as f32).log10() / self.n_sa as f32).sqrt();
    }
}

fn make_legal_actions(my_piece: u64, en_piece: u64) -> Vec<u64> {
    let mut put_posi: u64;
    let mut can_moves = can_locate(my_piece, en_piece);
    let mut legal_actions: Vec<u64> = Vec::new();
    while can_moves != 0 {
        put_posi = can_moves & (!can_moves + 1);
        legal_actions.push(put_posi);
        can_moves ^= put_posi
    }
    legal_actions
}

// MCTS
pub fn mcts_s(node: &mut Node) -> Pmove {
    fn evaluate(node: &mut Node) -> i32 {
        node.n_sa += 1;
        if node.n_sa == 1 {
            if node.legal_actions.is_empty() {
                // first visit
                if can_locate(node.en_piece, node.my_piece) == 0 {
                    node.state = NodeState::End; // game end
                    let score =
                        node.my_piece.count_ones() as i32 - node.en_piece.count_ones() as i32;
                    node.value = match score {
                        //# score plus 2 -> 2, draw 0 -> 1 , minus -2 -> 0 
                        0 => {
                            node.winlose = WinLose::Draw;
                            0
                        }
                        _ if score > 0 => {
                            node.winlose = if node.turn_player == TurnPlayer::Black {
                                WinLose::Win
                            } else {
                                WinLose::Lose
                            };
                            -2
                        }
                        _ => {
                            node.winlose = if node.turn_player == TurnPlayer::Black {
                                WinLose::Lose
                            } else {
                                WinLose::Win
                            };
                            2
                        }
                    };
                    node.w_sa += ((node.value + 2) / 2) as u32;
                    node.ucb1_update(node.n_sa);
                    return node.value;
                } else {
                    node.state = NodeState::Pass; // pass
                    node.child_nodes.push(node.pass_node());
                    let value = -evaluate(&mut node.child_nodes[0]);
                    node.w_sa += ((value + 2) / 2) as u32;
                    return value;
                }
            } else {
                node.state = NodeState::BelowTh;
            }
        }
        if node.n_sa > THRESHOLD  && node.child_nodes.is_empty() && node.state == NodeState::BelowTh {
            node.state = NodeState::AboveTh;
            for &action in &node.legal_actions {
                let next = node.next_node(&action); // make child nodes
                node.child_nodes.push(next);
            }
        }
        match node.state {
            NodeState::End => {
                node.w_sa += ((node.value + 2) / 2) as u32;
                node.value
            }
            NodeState::Pass => {
                node.value = match node.child_nodes.first() {
                    Some(first_child)
                        if first_child.winlose != WinLose::Undecided && node.step > 1 =>
                    {
                        node.winlose = first_child.winlose.clone();
                        -first_child.value
                    }
                    Some(_) => -evaluate(&mut node.child_nodes[0]),
                    None => panic!("Child node is empty"),
                };
                node.w_sa += ((node.value + 2) / 2) as u32;
                node.value
            }
            NodeState::BelowTh => {
                node.value = -2 * roolout(node);
                node.w_sa += ((node.value + 2) / 2) as u32;
                node.value
            }
            NodeState::AboveTh => {
                let mut max_ucb: f32 = f32::MIN;
                let mut selected_node = None;
                let mut is_bias_all: bool = true;
                for node_ch in node.child_nodes.iter_mut() {
                    let is_win = match node.turn_player {
                        TurnPlayer::Black => node_ch.winlose == WinLose::Win,
                        _ => node_ch.winlose == WinLose::Lose,
                    };
                    if is_win && node.step > 1 {
                        node.value = -node_ch.value;
                        node.w_sa += ((node.value + 2) / 2) as u32;
                        node.winlose = node_ch.winlose.clone();
                        node.ucb1_update(node.n_sa);
                        return node.value;
                    }
                    let is_not_bias = if node.turn_player == TurnPlayer::Black {
                        node_ch.winlose != WinLose::Lose
                    } else {
                        node_ch.winlose != WinLose::Win
                    };
                    if is_not_bias {
                        is_bias_all = false;
                    }
                    if node_ch.n_sa == 0 {
                        selected_node = Some(node_ch);
                        break;
                    }
                    node_ch.ucb1_update(node.n_sa);
                    if max_ucb < node_ch.ucb1 {
                        max_ucb = node_ch.ucb1;
                        selected_node = Some(node_ch);
                    }
                }
                if is_bias_all {
                    node.winlose = node.child_nodes[0].winlose.clone();
                    node.value = -node.child_nodes[0].value;
                    node.w_sa += ((node.value + 2) / 2) as u32;
                    node.ucb1_update(node.n_sa);
                    return node.value;
                }
                let selected_node = selected_node.expect("No valid child node found");
                node.value = -evaluate(selected_node);
                node.w_sa += ((node.value + 2) / 2) as u32;
                node.value
            }
            _ => node.value,
        }
    }

    if node.legal_actions.is_empty() {
        return Pmove::Passmove; // Pass
    }
    if node.legal_actions.len() == 1 {
        // only one move
        return Pmove::Pvmove(*node.legal_actions.first().unwrap());
    };
    // for evaluatte 5000 times
    for _ in 0..5000 {
        evaluate(node);
    }
    //println!("{}", node);
    //println!("---------");
    // choose best move
    let mut n_sa_max: u32 = 0;
    let mut max_action: u64 = 0;
    for ch_node in node.child_nodes.iter() {
        if n_sa_max < ch_node.n_sa {
            n_sa_max = ch_node.n_sa;
            max_action = ch_node.action;
        }
    }
    Pmove::Pvmove(max_action)
}

pub fn mcts(black: u64, white: u64, _left: usize) -> Pmove {
    let mut node = Node::new(TurnPlayer::Black, black, white, 0, 0);
    mcts_s(&mut node)
}

//# simple monte carlo
fn primitive_montecarlo_s(node: &Node) -> Pmove {
    if node.legal_actions.is_empty() {
        return Pmove::Passmove; //# pass check
    }
    if node.legal_actions.len() == 1 {
        return Pmove::Pvmove(node.legal_actions[0]);
    }
    let mut values: Vec<i32> = Vec::new();
    for &action in node.legal_actions.iter() {
        // win +1 lose -1 draw 0
        let mut total_value: i32 = 0;
        let next_node = node.next_node(&action);
        for _ in 0..200 {
            total_value -= roolout(&next_node);
        }
        values.push(total_value);
    }
    let max_index = values
        .iter()
        .enumerate()
        .max_by_key(|&(_, value)| value)
        .map(|(index, _)| index)
        .unwrap();
    Pmove::Pvmove(node.legal_actions[max_index])
}
pub fn primitive_montecarlo(black: u64, white: u64, _left: usize) -> Pmove {
    let node = Node::new(TurnPlayer::Black, black, white, 0, 0);
    primitive_montecarlo_s(&node)
}

fn roolout(node: &Node) -> i32 {
    if node.legal_actions.is_empty() {
        //# pass check
        if can_locate(node.en_piece, node.my_piece) == 0 {
            let score = node.my_piece.count_ones() as i32 - node.en_piece.count_ones() as i32;
            let value = match score {
                //# score plus -> 1, zero -> 0 , minus  -> - 1
                0 => 0,
                _ if score > 0 => 1,
                _ => -1,
            };
            return value; // game end
        } else {
            let node2 = node.pass_node(); // pass
            return - roolout(&node2);
        }
    }
    let mut rng = thread_rng();
    let next_action = node.legal_actions.choose(&mut rng).unwrap();
    let next_node = node.next_node(next_action);
        - roolout(&next_node)
}
