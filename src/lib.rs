use core::fmt;
use std::io::Stdout;

use crossterm::{cursor::{self, position}, event::KeyCode, style::{Color, Print}, QueueableCommand};

use crate::math::Vec2;
pub mod game;
pub mod math;

type Ret = std::io::Result<()>;

pub struct RenderObject{
    
}



#[derive(Debug)]
pub struct RGB{
    pub r:u8,
    pub g:u8,
    pub b:u8,
    pub res:bool,
}
impl From<crossterm::style::Color> for RGB {
    fn from(value: crossterm::style::Color) -> Self {
        match value {
            Color::Black            => Self{r:0, g:0, b:0,res:false},
            Color::DarkGrey         => Self{r:128, g:128, b:128,res:false},
            Color::White            => Self{r:255, g:255, b:255,res:false},
            Color::Grey             => Self{r:192, g:192, b:192,res:false},
            Color::DarkRed          => Self{r:128, g:0, b:0,res:false},
            Color::Red              => Self{r:255, g:0, b:0,res:false},
            Color::DarkGreen        => Self{r:0, g:128, b:0,res:false},
            Color::Green            => Self{r:0, g:255, b:0,res:false},
            Color::DarkYellow       => Self{r:128, g:128, b:0,res:false},
            Color::Yellow           => Self{r:255, g:255, b:0,res:false},
            Color::DarkBlue         => Self{r:0, g:0, b:128,res:false},
            Color::Blue             => Self{r:0, g:0, b:255,res:false},
            Color::DarkMagenta      => Self{r:128, g:0, b:128,res:false},
            Color::Magenta          => Self{r:255, g:0, b:255,res:false},
            Color::DarkCyan         => Self{r:0, g:128, b:128,res:false},
            Color::Cyan             => Self{r:0, g:255, b:255,res:false},
            Color::Rgb { r, g, b }  => Self{r, g, b,res:false},
            Color::AnsiValue(c) => ansi_to_rgb(c),
            Color::Reset => RGB { r: 0, g: 0, b: 0, res: true },
        }
    }
}
pub fn ansi_to_rgb(code: u8) -> RGB {
    match code {
        // --- Standard colors (0..=15) ---
        0 => RGB { r:   0, g: 0, b: 0,           res:false},     // Black
        1 => RGB { r:   128, g: 0, b: 0,         res:false},     // Red
        2 => RGB { r:   0, g: 128, b: 0,         res:false},     // Green
        3 => RGB { r:   128, g: 128, b: 0,       res:false},      // Yellow
        4 => RGB { r:   0, g: 0, b: 128,         res:false},      // Blue
        5 => RGB { r:   128, g: 0, b: 128,       res:false},      // Magenta
        6 => RGB { r:   0, g: 128, b: 128,       res:false},      // Cyan
        7 => RGB { r:   192, g: 192, b: 192,     res:false},      // White (light gray)
        8 => RGB { r:   128, g: 128, b: 128,     res:false},      // Bright Black (dark gray)
        9 => RGB { r:   255, g: 0, b: 0,         res:false},      // Bright Red
        10 => RGB { r:  0, g: 255, b: 0,        res:false},      // Bright Green
        11 => RGB { r:  255, g: 255, b: 0,      res:false},      // Bright Yellow
        12 => RGB { r:  0, g: 0, b: 255,        res:false},      // Bright Blue
        13 => RGB { r:  255, g: 0, b: 255,      res:false},      // Bright Magenta
        14 => RGB { r:  0, g: 255, b: 255,      res:false},      // Bright Cyan
        15 => RGB { r:  255, g: 255, b: 255,    res:false},      // Bright White

        // --- 6×6×6 color cube (16..=231) ---
        16..=231 => {
            let n = code - 16;
            let r = n / 36;
            let g = (n % 36) / 6;
            let b = n % 6;
            RGB {
                r: r * 51,
                g: g * 51,
                b: b * 51,
                res: false
            }
        }

        // --- Grayscale ramp (232..=255) ---
        232..=255 => {
            let level = (code - 232) * 10 + 8;
            RGB {
                r: level,
                g: level,
                b: level,
                res:false
            }
        }
    }
}


pub struct Line{
    pub start:Vec2,
    pub end:Vec2,
    pub char:TermPrint
}


// renderable
impl fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p = Vec2::from(position().unwrap());
        todo!()
    }
}

#[derive(Debug)]
pub struct GameObject{
    pub pos:Vec2,
    pub texture:Vec<PrintThing>,
}
impl Object for GameObject{
    fn get_cords(&self) -> Vec2 {
        self.pos
    }

    fn set_cords(&mut self,x:i32,y:i32) {
        self.pos = Vec2::from((x,y))
    }

    fn print(&self,out:&mut Stdout) -> Ret {
        for x in &self.texture{
            let nev = self.pos + x.rel_pos;
            out.queue(cursor::MoveTo(nev.x as u16,nev.y as u16))?;
            out.queue(Print(&x.text))?;
        };
        Ok(())

    }
}
impl GameObject {
    pub fn new(v:Vec2) -> Self{
        Self { pos: v, texture: Vec::new() }
    }
    pub fn print(&mut self,out:&mut Stdout) -> std::io::Result<()>{
        for x in &self.texture{
            let nev = self.pos + x.rel_pos;
            out.queue(cursor::MoveTo(nev.x as u16,nev.y as u16))?;
            out.queue(Print(&x.text))?;
        };
        Ok(())
    }
}
#[derive(Debug)]
pub struct PrintThing{
    pub rel_pos:Vec2,
    pub text:TermPrint,
}
#[derive(Debug)]
pub struct TermPrint {
    pub background:RGB,
    pub foreground:RGB,
    pub text:String,
}

impl From<&str> for TermPrint{
    fn from(value: &str) -> Self {
        Self { background: RGB { r: 0, g: 0, b: 0, res: true }, foreground: RGB { r: 0, g: 0, b: 0, res: true }, text: value.to_string() }
    }
}

impl fmt::Display for TermPrint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let back = {
            if self.background.res{
                "".to_string()
            }else {
                format!("\x1b[48;2;{};{};{}m",self.background.r,self.background.g,self.background.b)
            }
        };
        let front = {
            if self.foreground.res{
                "".to_string()
            }else {
                format!("\x1b[38;2;{};{};{}m",self.foreground.r,self.foreground.g,self.foreground.b)
            }
        };
        write!(
            f,
            "{}{}{}",    
            back,
            front,
            self.text
        )
    }
}




trait Object{
    fn get_cords(&self) -> Vec2;
    fn set_cords(&mut self,x:i32,y:i32);
    fn print(&self,out:&mut Stdout) -> Ret;
}



/// keys for the game
#[derive(Hash,PartialEq,Eq,Debug,Clone,Copy)]
pub enum Keys {
    Up,
    Down,
    Left,
    Right,
    Esc,
    Debug,
    Null,
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
                    _ => Self::Null
                }
            },
            KeyCode::Esc => Self::Esc,
            KeyCode::F(4) => Self::Debug,

            _ => Self::Null,
        }
    }
}




