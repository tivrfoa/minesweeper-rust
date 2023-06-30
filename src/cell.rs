use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use crate::cell_signal::CellSignal;
use crate::cell_state::CellState;
use crate::channel::Channel;
use crate::game_signal::GameSignal;

pub struct Cell {
    row: i32,
    col: i32,
    mine: bool,
    neighbor_mines: i32,

    in_channel: Channel<CellSignal>,
    out_channels: Vec<Channel<CellSignal>>,
    state_channel: Channel<GameSignal>,

    revealed: bool,
    flagged: bool,
    neighbor_flags: i32,
}

impl Cell {
    fn start(&mut self) {
        /*let in_channel = self.in_channel.clone();
        let out_channels = self.out_channels.clone();
        let state_channel = self.state_channel.clone();

        thread::spawn(move || {
            self.run();
        });*/
		self.run();
    }

    fn run(&mut self) {
        loop {
            match self.in_channel.take() {
                CellSignal::LeftClick => {
                    if self.revealed &&
							self.neighbor_mines == self.neighbor_flags {
                        self.reveal_neighbors();
                    } else {
                        self.reveal_me();
                    }
                }
                CellSignal::RightClick => {
                    if self.revealed &&
							self.neighbor_mines == self.neighbor_flags {
                        self.reveal_neighbors();
                    } else if !self.revealed {
                        self.flagged = !self.flagged;
                        self.tell_neighbors_about_flag();
                    }
                }
                CellSignal::NeighborReveal => self.reveal_me(),
                CellSignal::NeighborFlagSet => self.neighbor_flags += 1,
                CellSignal::NeighborFlagUnset => self.neighbor_flags -= 1,
                CellSignal::Stop => {
                    return;
                }
            }
            self.send_game_signal();
        }
    }

    fn send_game_signal(&self) {
        let cell_state = CellState {
            row: self.row,
            col: self.col,
            mine: self.mine,
            revealed: self.revealed,
            flagged: self.flagged,
            neighbor_mines: self.neighbor_mines,
        };
        self.state_channel.put(GameSignal::StateChange(cell_state));
    }

    fn reveal_me(&mut self) {
        if !self.flagged && !self.revealed {
            self.revealed = true;
            if !self.mine && self.neighbor_mines == 0 {
                self.reveal_neighbors();
            }
        }
    }

    fn reveal_neighbors(&mut self) {
        self.tell_neighbors(CellSignal::NeighborReveal);
    }

    fn tell_neighbors_about_flag(&mut self) {
        let signal = if self.flagged {
            CellSignal::NeighborFlagSet
        } else {
            CellSignal::NeighborFlagUnset
        };
        self.tell_neighbors(signal);
    }

    fn tell_neighbors(&mut self, signal: CellSignal) {
        for ch in &mut self.out_channels {
            ch.put(signal.clone());
        }
    }
}

