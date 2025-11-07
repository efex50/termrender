use core::fmt;
use std::fmt::Debug;

use crossterm::style::Color;

use crate::{LOG, Ret, RetType, game::screen::{CursorMoveTo, Screen}, math::Vec2, physics::AABB};

#[derive(Debug,Clone, Copy,PartialEq)]
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
    RGB(u8,u8,u8),
    ResetColor,
    Default,
}

#[derive(Debug,Clone, Copy)]
pub(crate)  struct RGB{
    pub r:u8,
    pub g:u8,
    pub b:u8,
    pub res:bool,
}
impl Default for RGB {
    fn default() -> Self {
        Self { r: Default::default(), g: Default::default(), b: Default::default(), res: false }
    }
}
impl From<(u8,u8,u8)> for RGB {
    fn from(value: (u8,u8,u8)) -> Self {
        Self { r: value.0, g: value.1, b: value.2, res: false }
    }
}
impl From<(u8,u8,u8)> for GColor {
    fn from(value: (u8,u8,u8)) -> Self {
        Self::RGB(value.0, value.1, value.2)
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
        match value {
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
                    let n = value - 16;
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
                    let level = (value - 232) * 10 + 8;
                    RGB {
                        r: level,
                        g: level,
                        b: level,
                        res:false
                    }
                }
            }
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
            GColor::RGB(r,g ,b )             => Self{r, g, b,res:false},
            GColor::ResetColor => Self { r: 0, g: 0, b: 0, res: true },
            GColor::Default => panic!("no impl engine should add the default value"),
        }
    }
}
impl From<RGB> for GColor {
    fn from(value: RGB) -> Self {
        match value {
            RGB{r:0, g:0, b:0,res:false} => GColor::Black,
            RGB{r:128, g:128, b:128,res:false} => GColor::DarkGrey,
            RGB{r:255, g:255, b:255,res:false} => GColor::White,
            RGB{r:192, g:192, b:192,res:false} => GColor::Grey,
            RGB{r:128, g:0, b:0,res:false} => GColor::DarkRed,
            RGB{r:255, g:0, b:0,res:false} => GColor::Red,
            RGB{r:0, g:128, b:0,res:false} => GColor::DarkGreen,
            RGB{r:0, g:255, b:0,res:false} => GColor::Green,
            RGB{r:128, g:128, b:0,res:false} => GColor::DarkYellow,
            RGB{r:255, g:255, b:0,res:false} => GColor::Yellow,
            RGB{r:0, g:0, b:128,res:false} => GColor::DarkBlue,
            RGB{r:0, g:0, b:255,res:false} => GColor::Blue,
            RGB{r:128, g:0, b:128,res:false} => GColor::DarkMagenta,
            RGB{r:255, g:0, b:255,res:false} => GColor::Magenta,
            RGB{r:0, g:128, b:128,res:false} => GColor::DarkCyan,
            RGB{r:0, g:255, b:255,res:false} => GColor::Cyan,
            RGB { r, g, b, res:false } => GColor::RGB(r, g, b),
            RGB{r: _r,g: _,b: _b,res:true} => GColor::ResetColor,
        }
    }
}
pub fn ansi_to_gcolor(code: u8) -> GColor {
    match code {
        // --- Standard colors (0..=15) ---
        0 =>  GColor::Black,                // Black
        1 =>  GColor::Red,                  // Red
        2 =>  GColor::Green,                // Green
        3 =>  GColor::Yellow,               // Yellow
        4 =>  GColor::Blue,                 // Blue
        5 =>  GColor::Magenta,              // Magenta
        6 =>  GColor::Cyan,                 // Cyan
        7 =>  GColor::White,                // White (light gray)
        8 =>  GColor::RGB(128, 128, 128),   // Bright Black (dark gray)
        9 =>  GColor::RGB(255, 0, 0),       // Bright Red
        10 => GColor::RGB(0, 255, 0),       // Bright Green
        11 => GColor::RGB(255, 255, 0),     // Bright Yellow
        12 => GColor::RGB(0, 0, 255),       // Bright Blue
        13 => GColor::RGB(255, 0, 255),     // Bright Magenta
        14 => GColor::RGB(0, 255, 255),     // Bright Cyan
        15 => GColor::RGB(255, 255, 255),   // Bright White

        // --- 6×6×6 color cube (16..=231) ---
        16..=231 => {
            let n = code - 16;
            let r = n / 36;
            let g = (n % 36) / 6;
            let b = n % 6;
            GColor::RGB (
                r * 51,
                g * 51,
                b * 51,
            )
        }

        // --- Grayscale ramp (232..=255) ---
        232..=255 => {
            let level = (code - 232) * 10 + 8;
            GColor::RGB (
                level,
                level,
                level,
            )
        }
    }
}
impl From<GColor> for Color {
    fn from(value: GColor) -> Self {
        match value{
            GColor::Black => Color::Black,
            GColor::DarkGrey => Color::DarkGrey,
            GColor::White => Color::White,
            GColor::Grey => Color::Grey,
            GColor::DarkRed => Color::DarkRed,
            GColor::Red => Color::Red,
            GColor::DarkGreen => Color::DarkGreen,
            GColor::Green => Color::Green,
            GColor::DarkYellow => Color::DarkYellow,
            GColor::Yellow => Color::Yellow,
            GColor::DarkBlue => Color::DarkBlue,
            GColor::Blue => Color::Blue,
            GColor::DarkMagenta => Color::DarkMagenta,
            GColor::Magenta => Color::Magenta,
            GColor::DarkCyan => Color::DarkCyan,
            GColor::Cyan => Color::Cyan,
            GColor::RGB(r, g, b) => Color::Rgb { r, g, b },
            GColor::ResetColor => Color::Reset  ,
            GColor::Default => panic!("no impl engine should add the default value"),
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
            out.reset_color()?;
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
// teşekkürler çatgbt
/// Texture oluşturma makrosu
/// 
/// düz string => ((posx,posy) , ("char",GColor,Gcolor))
/// 
/// line => ((sposx,sposy) => (fposx,fposy) ("char",GColor,Gcolor))
/// 
/// square => [aabb, ("char",GColor,Gcolor)]
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

    // Square variant
    (@item [$aabb:expr, $ch:expr]) =>{
        PrintTypes::Square{
            pos:$aabb,
            char:TermPrint::from($ch)
        }
    };
    // PrintType variant
    (@item $type:expr) =>{
        $type
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
    Square{
        pos:AABB,
        char:TermPrint,
    }
}

impl PrintTypes {
    pub fn print(&self,out:&mut Screen,_pos:Vec2) -> Ret{
        match self {
            PrintTypes::Line { start, end ,char} => Self::line_print(out, *start + _pos, *end + _pos, char),
            PrintTypes::String { pos, char } => Self::str_print(out, *pos + _pos, char),
            PrintTypes::Square { pos, char } => Self::square_print(out, *pos + _pos, char),
        }?;
        Ok(())
    }
    pub fn clearself(&self,out:&mut Screen,_pos:Vec2,old:Vec2) -> RetType<bool>{
        match self {
            PrintTypes::Line { start, end, char } => Self::line_clearself(out, old,*start + _pos, *end + _pos,char),
            PrintTypes::String { pos, char } => Self::str_clearself(out, old + *pos, char),
            PrintTypes::Square { pos, char } => Self::square_clearself(out, *pos + old, char),
        }?;

        Ok(true)
    }

    fn line_print(out:&mut Screen,start:Vec2,end:Vec2,char:&TermPrint) -> Ret{
        // vertical
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

            for _ in _start.._end+1{
                out.reset_color()?;
                out.queue(char.clone())?;
                out.cursor_move(CursorMoveTo::Down(1))?;
                out.cursor_move(CursorMoveTo::Left(shift_len as u16))?;
            }
        }
        // horizontal
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
            let new = TermPrint::from((str.as_str(),char.fore,char.background));
            out.queue(new)?;
        }
        // 45 dergree
        // todo
        else if {
            (start.x - end.x).abs() == (start.y - end.y).abs()
        }{
            let (start,end) = {
                if start.x < end.x{
                    (start,end)
                }else {
                    (end,start)
                }
            };
            // 45
            out.cursor_move(CursorMoveTo::Pos((start.x as u16,start.y as u16)))?;
            if start.y > end.y{
                for _x in start.x..end.x+1{
                    out.queue(char)?;
                    out.cursor_move(CursorMoveTo::Up(1))?;
                }
            }
            // 315
            else {
                for _x in start.x..end.x+1{
                    out.queue(char)?;
                    out.cursor_move(CursorMoveTo::Down(1))?;
                }

            }

        }
        // cross
        else {
            bresenham_line_i32(out, start, end, char)?;
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

            for _ in start..end+1{
                out.queue(TermPrint::from(" "))?;
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
            let str = TermPrint::from(" ".repeat(len as usize).as_str());

            out.queue(str)?;
        }
        // cross
        else {
            bresenham_line_i32(out, start, end, &TermPrint::from(" "))?;
        }
        Ok(true)

    }

    fn str_print(out:&mut Screen,pos:Vec2,char:&TermPrint) -> Ret {
        out.cursor_move(CursorMoveTo::Pos(pos.into()))?;
        out.queue(char.clone())?;

        Ok(())
    }
    fn str_clearself(out:&mut Screen,old:Vec2,char:&TermPrint) -> RetType<bool> {
        let newt = {
            let len = char.text.chars().collect::<Vec<char>>().len();
            TermPrint::from(" ".repeat(len).as_str())
        };
        out.cursor_move(CursorMoveTo::Pos(old.into()))?;
        out.queue(newt)?;

        Ok(true)
    }

    fn square_print(out:&mut Screen,pos:AABB,char:&TermPrint) -> Ret{
        let corners = pos.get_corners();
        let lines = corners.3.y - corners.0.y;
        let rows = corners.1.x - corners.0.x;
        let row = format!("{}",char).repeat(rows as usize);
        let new = TermPrint::from((row.as_str(),char.background,char.fore));
        out.cursor_move(CursorMoveTo::Pos((corners.0.x as u16,corners.0.y as u16)))?;
        for r in 0..lines+1{
            out.queue(new.clone())?;
            out.cursor_move(CursorMoveTo::Pos((corners.0.x as u16,(corners.0.y+r) as u16)))?;
            out.cursor_move(CursorMoveTo::Down(1))?;
        }; 
        Ok(())
    }
    fn square_clearself(out:&mut Screen,pos:AABB,_char:&TermPrint) -> RetType<bool>{
        let corners = pos.get_corners();
        let lines = corners.3.y - corners.0.y;
        let rows = corners.1.x - corners.0.x;
        let row = " ".repeat(rows as usize);
        let new = TermPrint::from(row.as_str());
        out.cursor_move(CursorMoveTo::Pos((corners.0.x as u16,corners.0.y as u16)))?;
        for r in 0..lines+1{
            out.queue(new.clone())?;
            out.cursor_move(CursorMoveTo::Pos((corners.0.x as u16,(corners.0.y+r) as u16)))?;
            out.cursor_move(CursorMoveTo::Down(1))?;
        }; 


        Ok(true)
    }
}


fn bresenham_line_i32(out: &mut Screen, start: Vec2, end: Vec2, ch: &TermPrint) -> Ret{
    let dx = (end.x - start.x).abs();
    let dy = (end.y - start.y).abs();
    let sx = if start.x < end.x { 1 } else { -1 };
    let sy = if start.y < end.y { 1 } else { -1 };

    let mut x = start.x;
    let mut y = start.y;

    let mut err = if dx > dy { dx } else { -dy } / 2;

    loop {
        // karakteri yaz
        out.cursor_move(CursorMoveTo::Pos((x as u16, y as u16)))?;
        out.queue(ch.clone())?;

        if x == end.x && y == end.y {
            break;
        }

        let e2 = err;

        if e2 > -dx {
            err -= dy;
            x += sx;
        }
        if e2 < dy {
            err += dx;
            y += sy;
        }
    }

    Ok(())
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
    pub background:GColor,
    pub fore:GColor,
    pub text:String,
}
impl TermPrint {
    pub fn as_plain(&self) -> String{
        self.text.to_string()
    }
}

impl From<&str> for TermPrint{
    fn from(value: &str) -> Self {
        Self { background: GColor::Default, fore: GColor::Default, text: value.to_string() }
    }
}

impl From<(&str,RGB)> for TermPrint{
    fn from(value: (&str,RGB)) -> Self {
        Self { background: GColor::from(value.1), fore: GColor::Default, text: value.0.to_string() }
    }
}
impl From<(&str,(),RGB)> for TermPrint{
    fn from(value: (&str,(),RGB)) -> Self {
        Self { background: GColor::Default, fore: GColor::from(value.2), text: value.0.to_string() }
    }
}
impl From<(&str,RGB,RGB)> for TermPrint{
    fn from(value: (&str,RGB,RGB)) -> Self {
        Self { background: GColor::from(value.1), fore: GColor::from(value.2), text: value.0.to_string() }
    }
}



impl From<(&str,GColor)> for TermPrint{
    fn from(value: (&str,GColor)) -> Self {
        Self { background: value.1, fore: GColor::Default, text: value.0.to_string() }
    }
}
impl From<(&str,(),GColor)> for TermPrint{
    fn from(value: (&str,(),GColor)) -> Self {
        Self { background: GColor::Default, fore: value.2, text: value.0.to_string() }
    }
}
impl From<(&str,GColor,GColor)> for TermPrint{
    fn from(value: (&str,GColor,GColor)) -> Self {
        Self { background: value.1, fore: value.2, text: value.0.to_string() }
    }
}
impl From<String> for TermPrint{
    fn from(value: String) -> Self {
        Self { background: GColor::Default, fore: GColor::Default, text: value }
    }
}
impl From<(String,GColor)> for TermPrint{
    fn from(value: (String,GColor)) -> Self {
        Self { background: value.1, fore: GColor::Default, text: value.0 }
    }
}
impl From<(String,(),GColor)> for TermPrint{
    fn from(value: (String,(),GColor)) -> Self {
        Self { background: GColor::Default, fore: value.2, text: value.0 }
    }
}
impl From<(String,GColor,GColor)> for TermPrint{
    fn from(value: (String,GColor,GColor)) -> Self {
        Self { background: value.1, fore: value.2, text: value.0 }
    }
}

impl fmt::Display for TermPrint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let back = {
            match self.background {
                GColor::Default => "".to_string(),
                _ => {
                    let rgb = RGB::from(self.background);
                    if rgb.res{
                        "".to_string()
                    }else {   
                        format!("\x1b[48;2;{};{};{}m",rgb.r,rgb.g,rgb.b)
                    }
                },
            }
        };
        let fore = {
            match self.fore {
                GColor::Default => "".to_string(),
                _ => {
                    let rgb = RGB::from(self.fore);
                    if rgb.res{
                        "".to_string()
                    }else {   
                        format!("\x1b[38;2;{};{};{}m",rgb.r,rgb.g,rgb.b)
                    }
                },
            }
        };
        write!(
            f,
            "{}{}{}",
            back,
            fore,
            self.text,
            //"\x1b[0m"
        )
    }
}



#[test]
fn forr(){
    for x in 0..10{
        println!("{}",x)
    }
}
