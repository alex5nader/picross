//! Console version of picross.

#![deny(missing_docs)]

use crate::cell::SimpleCell;
use itertools::Itertools;
use pancurses::{
    curs_set, endwin, init_pair, initscr, noecho, start_color, Input, COLOR_BLACK, COLOR_GREEN, COLOR_PAIR, COLOR_WHITE,
};
use picore::{constraints, Cell, Picross, Puzzle};
use std::collections::HashMap;

pub mod cell;

fn demo_puzzle() -> Puzzle<SimpleCell> {
    //  # #
    //  # #
    //
    // #   #
    //  ###
    Puzzle::new(
        constraints![
            [1, SimpleCell; 1, SimpleCell]
            [1, SimpleCell; 1, SimpleCell]
            []
            [1, SimpleCell; 1, SimpleCell]
            [3, SimpleCell]
        ],
        constraints! {
            [1, SimpleCell]
            [2, SimpleCell; 1, SimpleCell]
            [1, SimpleCell]
            [2, SimpleCell; 1, SimpleCell]
            [1, SimpleCell]
        },
    )
}

const COLOR_SOLVED: i16 = 1;
const COLOR_SELECTION: i16 = 2;
const COLOR_SELECTION_SOLVED: i16 = 3;

fn main() {
    let mut picross = Picross::new(demo_puzzle());

    let window = initscr();
    curs_set(0);
    noecho();

    start_color();
    init_pair(COLOR_SOLVED, COLOR_GREEN, COLOR_BLACK);
    init_pair(COLOR_SELECTION, COLOR_BLACK, COLOR_WHITE);
    init_pair(COLOR_SELECTION_SOLVED, COLOR_BLACK, COLOR_GREEN);

    window.keypad(true);

    let mut row_sizes = vec![0; picross.row_constraints().iter().map(|c| c.len()).max().unwrap_or(0)];
    let mut col_sizes = vec![0; picross.column_constraints().len()];

    for rc in picross.row_constraints() {
        for (i, entry) in rc.iter().enumerate() {
            let text_size = ((entry.size as f32).log10() as usize) + 1;
            if text_size > row_sizes[i] {
                row_sizes[i] = text_size;
            }
        }
    }

    for (i, cc) in picross.column_constraints().iter().enumerate() {
        for entry in cc {
            let text_size = ((entry.size as f32).log10() as usize) + 1;
            if text_size > col_sizes[i] {
                col_sizes[i] = text_size;
            }
        }
    }

    let row_label_len = (row_sizes.iter().sum::<usize>() + row_sizes.len()) as i32;
    let col_label_len = picross.column_constraints().iter().map(|c| c.len()).max().unwrap_or(0) as i32;

    let board_base = (col_label_len + 1, row_label_len + 1);

    let mut board_pos = HashMap::new();

    for r in 0..picross.height() {
        let mut col_offset = 0;
        for c in 0..picross.width() {
            board_pos.insert((r, c), (r as i32, col_offset as i32));
            col_offset += col_sizes[c] + 1;
        }
    }

    let mut pos = (0, 0);

    let mut solved = false;

    loop {
        window.clear();
        window.mvprintw(1, window.get_max_x() - 20, format!("{:?}", pos));
        {
            let (row_status, column_status) = picross.status();
            window.mvprintw(
                2,
                window.get_max_x() - 23,
                format!("r: {}", row_status.iter().map(|s| if *s { "1" } else { "0" }).join("")),
            );
            window.mvprintw(
                3,
                window.get_max_x() - 23,
                format!(
                    "c: {}",
                    column_status.iter().map(|s| if *s { "1" } else { "0" }).join("")
                ),
            );
        }

        for (i, constraint) in picross.row_constraints().iter().enumerate() {
            let mut offset = 0;
            for (j, entry) in constraint.iter().enumerate() {
                window.mvprintw(
                    board_base.0 + i as i32,
                    board_base.1 - row_label_len + offset,
                    format!("{:width$}", entry.size, width = row_sizes[j]),
                );
                offset += row_sizes[j] as i32 + 1;
            }
        }

        let mut offset = 0;
        for (i, constraint) in picross.column_constraints().iter().enumerate() {
            for (j, entry) in constraint.iter().enumerate() {
                window.mvprintw(
                    board_base.0 - col_label_len + j as i32,
                    board_base.1 + offset,
                    format!("{:width$}", entry.size, width = col_sizes[i]),
                );
            }
            offset += col_sizes[i] as i32 + 1;
        }

        for (r, c, cell) in picross.cells() {
            if solved {
                window.attron(COLOR_PAIR(COLOR_SOLVED as _));
            }

            if r == pos.0 && c == pos.1 {
                window.attron(COLOR_PAIR(if solved {
                    COLOR_SELECTION_SOLVED
                } else {
                    COLOR_SELECTION
                } as _));
            }

            window.mvaddch(
                board_base.0 + board_pos[&(r, c)].0,
                board_base.1 + board_pos[&(r, c)].1,
                SimpleCell::char_repr(cell),
            );

            if solved {
                window.attroff(COLOR_PAIR(COLOR_SOLVED as _));
            }

            if r == pos.0 && c == pos.1 {
                window.attroff(COLOR_PAIR(if solved {
                    COLOR_SELECTION_SOLVED
                } else {
                    COLOR_SELECTION
                } as _));
            }
        }

        match window.getch() {
            Some(Input::KeyDC) => break,
            Some(Input::KeyLeft) => pos.1 = (pos.1 + picross.width() - 1) % picross.width(),
            Some(Input::KeyRight) => pos.1 = (pos.1 + picross.width() + 1) % picross.width(),
            Some(Input::KeyUp) => pos.0 = (pos.0 + picross.height() - 1) % picross.height(),
            Some(Input::KeyDown) => pos.0 = (pos.0 + picross.height() + 1) % picross.height(),
            Some(Input::Character('c')) => {
                solved = match picross.get(pos.0, pos.1) {
                    Cell::Empty | Cell::Filled(_) => picross.cross_out(pos.0, pos.1),
                    Cell::CrossedOut => picross.clear_at(pos.0, pos.1),
                }
            }
            Some(Input::Character(' ')) => {
                solved = match picross.get(pos.0, pos.1) {
                    Cell::Empty | Cell::CrossedOut => picross.place_at(SimpleCell, pos.0, pos.1),
                    Cell::Filled(_) => picross.clear_at(pos.0, pos.1),
                }
            }
            _ => {}
        };
    }

    endwin();
}
