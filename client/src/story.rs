//! Dialogue tree logic: opening nodes, applying effects, choosing options,
//! and deciding which node an interactable opens with.

use crate::dialogue::{Node, NODES};
use crate::state::{
    has, l, wrap_text, CUR_NODE, DIALOG_ANNIM, EFF_ENDING, EFF_NONE, F_ACCEPTED_LYRA, F_ASSAULT,
    F_HAS_RELIC, F_KNOWS_RELIC, F_RODERICK_LINE, F_TALKED_ANSELM, F_TALKED_ERIK, F_TALKED_KONRAD,
    F_TALKED_LYRA, F_TALKED_MATHILDE, F_TALKED_MILDRED, F_TALKED_RODERICK, F_TALKED_SORA,
    F_TOLD_LYRA, ST_DIALOG,
};

pub fn node_count() -> usize {
    NODES.len()
}

pub fn node(id: i32) -> &'static Node {
    &NODES[id as usize]
}

pub fn nodes() -> &'static [Node] {
    NODES
}

pub fn cur_node() -> i32 {
    unsafe { CUR_NODE }
}

pub fn open_node(id: i32) {
    unsafe {
        CUR_NODE = id;
        crate::state::STATE = ST_DIALOG;
        DIALOG_ANNIM = 0.0;
    }
    rewrap_current();
}

pub fn rewrap_current() {
    let id = cur_node();
    if id >= 0 && (id as usize) < NODES.len() {
        let max_w = (crate::state::CANVAS_W - 80) as f64;
        let text = l(NODES[id as usize].text_de, NODES[id as usize].text_en);
        wrap_text(text, max_w, 17.0);
    }
}

pub fn close_dialog() {
    unsafe {
        CUR_NODE = -1;
        crate::state::STATE = crate::state::ST_ROAM;
    }
}

pub fn apply_effect(effect: i32, val: i32) {
    use crate::state::{EFF_CLEAR_FLAG, EFF_SET_FLAG, EFF_SET_PROGRESS, ENDING_ID, FLAGS,
        PROGRESS, STATE, ST_ENDING};
    match effect {
        EFF_SET_PROGRESS => unsafe { PROGRESS = PROGRESS.max(val) },
        crate::state::EFF_SET_FLAG => unsafe { FLAGS |= val as u32 },
        EFF_CLEAR_FLAG => unsafe { FLAGS &= !(val as u32) },
        EFF_ENDING => unsafe {
            ENDING_ID = val;
            STATE = ST_ENDING;
        },
        _ => {}
    }
}

pub fn select_choice(idx: i32) {
    let id = cur_node();
    if id < 0 || (id as usize) >= NODES.len() {
        return;
    }
    let n = &NODES[id as usize];
    if idx < 0 || (idx as usize) >= n.choices.len() {
        return;
    }
    let ch = &n.choices[idx as usize];
    let eff = ch.effect;
    let val = ch.val;
    let next = ch.next;
    apply_effect(eff, val);
    bump_progress();
    if eff == EFF_ENDING {
        return;
    }
    if next < 0 {
        close_dialog();
    } else {
        open_node(next);
    }
}

fn bump_progress() {
    use crate::state::PROGRESS;
    let mut p = 0;
    if has(F_TALKED_MATHILDE) {
        p = p.max(1);
    }
    if has(F_KNOWS_RELIC) {
        p = p.max(2);
    }
    if has(F_HAS_RELIC) {
        p = p.max(3);
    }
    if has(F_RODERICK_LINE) || has(F_ASSAULT) || has(F_ACCEPTED_LYRA) {
        p = p.max(4);
    }
    unsafe { PROGRESS = PROGRESS.max(p) };
}

pub fn start_node_for(kind: i32) -> i32 {
    match kind {
        0 => {
            if !has(F_TALKED_MATHILDE) {
                0
            } else {
                2
            }
        }
        1 => {
            if has(F_TALKED_ANSELM) && has(F_KNOWS_RELIC) {
                6
            } else if has(F_TALKED_ANSELM) {
                5
            } else {
                3
            }
        }
        2 => {
            if has(F_HAS_RELIC) && !has(F_RODERICK_LINE) {
                13
            } else if has(F_RODERICK_LINE) {
                -1
            } else if has(F_TALKED_RODERICK) {
                11
            } else {
                9
            }
        }
        3 => {
            if has(F_TALKED_LYRA) {
                17
            } else {
                14
            }
        }
        4 => gate_node(),
        5 => {
            if has(F_HAS_RELIC) {
                -1
            } else if has(F_KNOWS_RELIC) {
                22
            } else {
                21
            }
        }
        6 => 31,  // Konrad
        7 => 35,  // Mildred
        8 => 39,  // Erik
        9 => 42,  // Sora
        _ => -1,
    }
}

fn gate_node() -> i32 {
    if !has(F_HAS_RELIC) && !has(F_ASSAULT) {
        return 23;
    }
    if has(F_TOLD_LYRA) && has(F_ACCEPTED_LYRA) {
        return 26;
    }
    if has(F_HAS_RELIC) {
        if has(F_ACCEPTED_LYRA) && !has(F_TOLD_LYRA) {
            return 25;
        }
        return 24;
    }
    27
}

// keep unused imports referenced
#[allow(dead_code)]
fn _unused() {
    let _ = (EFF_NONE, F_TALKED_KONRAD, F_TALKED_MILDRED, F_TALKED_ERIK, F_TALKED_SORA);
}
