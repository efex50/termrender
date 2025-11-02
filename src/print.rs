use core::fmt;
use std::fmt::Debug;

use crossterm::style::Color;

use crate::{RESET_COLOR, Ret, RetType, game::{screen::{CursorMoveTo, Screen}}, math::Vec2};

#[derive(Debug,Clone, Copy)]
pub enum GColor{
    Black,
    DarkGrey,
    White,
    Grey,
    DarkRed,
    Red,
    DarkGreen,
    Green,
    DarkYellow,
    Yellow,
    DarkBlue,
    Blue,
    DarkMagenta,
    Magenta,
    DarkCyan,
    Cyan,
}

#[derive(Debug,Clone, Copy)]
pub struct RGB{
    pub r:u8,
    pub g:u8,
    pub b:u8,
    pub res:bool,
}
impl Default for RGB {
    fn default() -> Self {
        Self { r: Default::default(), g: Default::default(), b: Default::default(), res: true }
    }
}
impl From<(u8,u8,u8)> for RGB {
    fn from(value: (u8,u8,u8)) -> Self {
        Self { r: value.0, g: value.1, b: value.2, res: false }
    }
}
impl From<bool> for RGB{
    fn from(value: bool) -> Self {
        Self { r: 0, g: 0, b: 0, res: value }
    }
}
/// ansi
impl From<u8> for RGB{
    fn from(value: u8) -> Self {
        ansi_to_rgb(value)
    }
}
impl From<GColor> for RGB {
    fn from(value: GColor) -> Self {
        match value {
            GColor::Black            => Self{r:0, g:0, b:0,res:false},
            GColor::DarkGrey         => Self{r:128, g:128, b:128,res:false},
            GColor::White            => Self{r:255, g:255, b:255,res:false},
            GColor::Grey             => Self{r:192, g:192, b:192,res:false},
            GColor::DarkRed          => Self{r:128, g:0, b:0,res:false},
            GColor::Red              => Self{r:255, g:0, b:0,res:false},
            GColor::DarkGreen        => Self{r:0, g:128, b:0,res:false},
            GColor::Green            => Self{r:0, g:255, b:0,res:false},
            GColor::DarkYellow       => Self{r:128, g:128, b:0,res:false},
            GColor::Yellow           => Self{r:255, g:255, b:0,res:false},
            GColor::DarkBlue         => Self{r:0, g:0, b:128,res:false},
            GColor::Blue             => Self{r:0, g:0, b:255,res:false},
            GColor::DarkMagenta      => Self{r:128, g:0, b:128,res:false},
            GColor::Magenta          => Self{r:255, g:0, b:255,res:false},
            GColor::DarkCyan         => Self{r:0, g:128, b:128,res:false},
            GColor::Cyan             => Self{r:0, g:255, b:255,res:false},
        }
    }
}
/*
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
*/
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
impl From<RGB> for Color {
    fn from(value: RGB) -> Self {
        if value.res {
            Color::Reset
        } else {
            Color::Rgb {
                r: value.r,
                g: value.g,
                b: value.b,
            }
        }
    }
}




#[derive(Debug,Clone,Default)]
pub struct GameTexture{
    textures:Vec<PrintTypes>
}

impl GameTexture {
    pub fn new(t:Vec<PrintTypes>) -> Self{
        Self { textures: t }
    }
    pub fn print(&self,out:&mut Screen,pos:Vec2) -> Ret{
        for x in &self.textures{
            x.print(out, pos)?;
            out.queque(RESET_COLOR.to_string())?;
        }
        Ok(())
    }

    pub fn clearself(&self,out:&mut Screen,pos:Vec2,old:Vec2) -> RetType<bool> {
        for x in &self.textures{
            x.clearself(out,pos,old)?;
        }
        Ok(true)
    }

}


#[macro_export]
macro_rules! make_texture {
    // Tek tek girdileri `texture!` benzeri alt kurala paslar
    ($($rep:tt),* $(,)?) => {
        {
            let v = vec![
                $(
                    make_texture!(@item $rep)
                ),*
            ];
            GameTexture::new(v)
        }
    };

    // String variant
    (@item (($x:expr, $y:expr) $c:expr)) => {
        PrintTypes::String {
            pos: Vec2::from(($x, $y)),
            char: TermPrint::from($c),
        }
    };

    // Line variant
    (@item (($sx:expr, $sy:expr) => ($ex:expr, $ey:expr) $c:expr)) => {
        PrintTypes::Line {
            start: Vec2::from(($sx, $sy)),
            end: Vec2::from(($ex, $ey)),
            char: TermPrint::from($c),
        }
    };
}




/// relative positions to the super obj
#[derive(Debug,Clone)]
pub enum PrintTypes{
    Line{
        start:Vec2,
        end:Vec2,
        char:TermPrint,
    },
    String{
        pos:Vec2,
        char:TermPrint
    },
}

impl PrintTypes {
    pub fn print(&self,out:&mut Screen,_pos:Vec2) -> Ret{
        out.reset_color()?;
        match self {
            PrintTypes::Line { start, end ,char} => Self::line_print(out, *start + _pos, *end + _pos, char),
            PrintTypes::String { pos, char } => Self::str_print(out, *pos + _pos, char),
        }?;
        out.reset_color()?;
        Ok(())
    }
    pub fn clearself(&self,out:&mut Screen,_pos:Vec2,old:Vec2) -> RetType<bool>{
        out.reset_color()?;
        match self {
            PrintTypes::Line { start, end, char } => Self::line_clearself(out, old,*start + _pos, *end + _pos,char),
            PrintTypes::String { pos, char } => Self::str_clearself(out, old + *pos, char),
        }?;
        out.reset_color()?;

        Ok(true)
    }

    fn line_print(out:&mut Screen,start:Vec2,end:Vec2,char:&TermPrint) -> Ret{
        // horizontal
        if start.x == end.x{
            let (_start,_end) = {
                if start.y < end.y{
                    (start.y,end.y)
                }else {
                    (end.y,start.y)
                }
            };
            let shift_len = char.text.chars().collect::<Vec<char>>().len();
            out.cursor_move(CursorMoveTo::Pos((start.x as u16,_start as u16)))?;

            for _ in _start.._end{
                out.queque(&char.to_string())?;
                out.cursor_move(CursorMoveTo::Down(1))?;
                out.cursor_move(CursorMoveTo::Left(shift_len as u16))?;
            }
        }
        // vertical
        else if start.y == end.y {
            
            let _start = {
                if start.x < end.x{
                    start.x
                }else {
                    end.x
                }
            };
            out.cursor_move(CursorMoveTo::Pos((_start as u16,start.y as u16)))?;

            let len = (end.x - start.x).abs();
            let str = char.text.repeat(len as usize);
            out.queque(str)?;
        }
        // cross
        else {
            panic!("this type of lines are not yet implemented")
        }
        Ok(())

    }
    fn line_clearself(out:&mut Screen,old:Vec2,start:Vec2,end:Vec2,char:&TermPrint) -> RetType<bool> {
        if start == old{
            return Ok(true);
        }
        let end_old = end - (start - old);
        // horizontal
        if old.x == end_old.x{
            let (start,end) = {
                if old.y < end_old.y{
                    (old.y,end_old.y)
                }else {
                    (end_old.y,old.y)
                }
            };
            let shift_len = char.text.chars().collect::<Vec<char>>().len();
            out.cursor_move(CursorMoveTo::Pos(old.into()))?;

            for _ in start..end{
                out.queque(" ")?;
                out.cursor_move(CursorMoveTo::Down(1))?;
                out.cursor_move(CursorMoveTo::Left(shift_len as u16))?;
            }
        }
        // vertical
        else if old.y == end_old.y {
            
            let _start = {
                if old.x < end_old.x{
                    old.x
                }else {
                    end_old.x
                }
            };
            out.cursor_move(CursorMoveTo::Pos((_start as u16,old.y as u16)))?;

            let len = (end_old.x - old.x).abs();
            let str = " ".repeat(len as usize);
            out.queque(&str)?;
        }
        // cross
        else {
            panic!("this type of lines are not yet implemented")
        }
        Ok(true)

    }

    fn str_print(out:&mut Screen,pos:Vec2,char:&TermPrint) -> Ret {
        out.cursor_move(CursorMoveTo::Pos(pos.into()))?;
        out.queque(char.to_string())?;

        Ok(())
    }
    fn str_clearself(out:&mut Screen,old:Vec2,char:&TermPrint) -> RetType<bool> {
        let newt = {
            let len = char.text.chars().collect::<Vec<char>>().len();
            " ".repeat(len)
        };
        out.cursor_move(CursorMoveTo::Pos(old.into()))?;
        out.queque(newt.to_string())?;

        Ok(true)
    }


}

#[test]
fn tstaastas(){
    println!("{}","".len())
}

impl Default for PrintTypes {
    fn default() -> Self {
        Self::String{
            pos:Vec2::from((0,0)),
            char:TermPrint::from(" "),
        }
    }
}


#[test]
fn test(){
    let _t = GameTexture::new(Vec::new());
}

#[derive(Debug,Clone,)]
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
/*
impl From<(&str,Color)> for TermPrint{
    fn from(value: (&str,Color)) -> Self {
        Self { background: RGB::from(value.1), foreground: RGB { r: 0, g: 0, b: 0, res: true }, text: value.0.to_string() }
    }
}
impl From<(&str,(),Color)> for TermPrint{
    fn from(value: (&str,(),Color)) -> Self {
        Self { background: RGB { r: 0, g: 0, b: 0, res: true }, foreground: RGB::from(value.2), text: value.0.to_string() }
    }
}
impl From<(&str,Color,Color)> for TermPrint{
    fn from(value: (&str,Color,Color)) -> Self {
        Self { background: RGB::from(value.1), foreground: RGB::from(value.2), text: value.0.to_string() }
    }
}

*/

impl From<(&str,GColor)> for TermPrint{
    fn from(value: (&str,GColor)) -> Self {
        Self { background: RGB::from(value.1), foreground: RGB { r: 0, g: 0, b: 0, res: true }, text: value.0.to_string() }
    }
}
impl From<(&str,(),GColor)> for TermPrint{
    fn from(value: (&str,(),GColor)) -> Self {
        Self { background: RGB { r: 0, g: 0, b: 0, res: true }, foreground: RGB::from(value.2), text: value.0.to_string() }
    }
}
impl From<(&str,GColor,GColor)> for TermPrint{
    fn from(value: (&str,GColor,GColor)) -> Self {
        Self { background: RGB::from(value.1), foreground: RGB::from(value.2), text: value.0.to_string() }
    }
}



impl From<(&str,RGB)> for TermPrint{
    fn from(value: (&str,RGB)) -> Self {
        Self { background: value.1, foreground: RGB { r: 0, g: 0, b: 0, res: true }, text: value.0.to_string() }
    }
}
impl From<(&str,(),RGB)> for TermPrint{
    fn from(value: (&str,(),RGB)) -> Self {
        Self { background: RGB { r: 0, g: 0, b: 0, res: true }, foreground: value.2, text: value.0.to_string() }
    }
}
impl From<(&str,RGB,RGB)> for TermPrint{
    fn from(value: (&str,RGB,RGB)) -> Self {
        Self { background: value.1, foreground: value.2, text: value.0.to_string() }
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


