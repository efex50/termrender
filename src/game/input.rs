use std::{collections::HashSet, time::Duration};



pub struct Input {
    pub pressed_keys: HashSet<Keys>,
    pub just_pressed_keys: HashSet<Keys>,
    poll_timeout:Duration
}
impl Input {
    pub fn new(timeout:Duration) -> Self{
        Self { pressed_keys: HashSet::new(), just_pressed_keys: HashSet::new(), poll_timeout: timeout }
    }
    pub(crate) fn poll_keys(&mut self) -> Ret{
        while event::poll(self.poll_timeout)? {
            if let Event::Key(k) = event::read()? {
                use crossterm::event::KeyEventKind::*;
                let code = Keys::from(k.code);
                match k.kind {
                    Press=> {
                        self.just_pressed_keys.insert(code);
                        self.pressed_keys.insert(code);
                    },
                    Repeat  => {self.pressed_keys.insert(code);},
                    Release => { self.pressed_keys.remove(&code); }
                }
            }
        }
                
        Ok(())
    }

}



use crossterm::event::{self, Event, KeyCode};

use crate::Ret;

/// keys for the game
#[derive(Hash,PartialEq,Eq,Debug,Clone,Copy)]
pub enum Keys {
    Up,
    Down,
    Left,
    Right,
    Esc,
    Debug,
    Refresh,
    Null,
    Space,

    E,
    Q,
    R,
    T,

}

impl From<KeyCode> for Keys {
    fn from(value: KeyCode) -> Self {
        match value {
            KeyCode::Left => Self::Left,
            KeyCode::Right => Self::Right,
            KeyCode::Up => Self::Up,
            KeyCode::Down => Self::Down,
            KeyCode::Char(c) => {
                match c {
                    'w' | 'W' => Self::Up,
                    's' | 'S' => Self::Down,
                    'a' | 'A' => Self::Left,
                    'd' | 'D' => Self::Right,
                    'e' | 'E' => Self::E,
                    'q' | 'Q' => Self::Q,
                    'r' | 'R' => Self::R,
                    't' | 'T' => Self::T,
                    ' ' => Self::Space,
                    _ => Self::Null
                }
            },
            KeyCode::Esc => Self::Esc,
            KeyCode::F(4) => Self::Debug,
            KeyCode::F(5) => Self::Refresh,

            _ => Self::Null,
        }
    }
}
