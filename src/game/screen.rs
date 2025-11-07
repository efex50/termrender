use std::{borrow::Borrow, io::Write};

#[cfg(not(windows))]
use crossterm::event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags};
use crossterm::{QueueableCommand, cursor::{self, MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp},style::{Color, Print, SetBackgroundColor, SetForegroundColor}, terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}};

use crate::{Ret, RetType, math::Vec2, prelude::ObjectHeader, print::{GColor, TermPrint}};

pub enum CursorMoveTo{
    Up(u16),
    Down(u16),
    Left(u16),
    Right(u16),
    /// takes vec2
    /// example
    /// 
    /// Pos(Vec2::from((x,y)))
    /// 
    /// or
    /// 
    /// Pos((x,y))
    /// 
    Pos((u16,u16)),
}

pub struct Screen {
    pub out: std::io::Stdout,
    pub posy: u16,
    pub screen: Vec2,
    pub bg: (GColor, GColor, char),
    pub(crate)  title :String
}
impl Screen {
    pub fn cursor_queque_obj(&mut self,p:&mut ObjectHeader) -> Ret {
        p.print(self)?;
        Ok(())
    }
    pub fn cursor_queque<T>(&mut self,print:&T) -> Ret where T:core::fmt::Display{
        self.out.queue(Print(print))?;
        
        Ok(())
    }
    pub fn cursor_move(&mut self,pos:CursorMoveTo) -> Ret{
        match pos {
            CursorMoveTo::Up(m) => {
                self.out.queue(MoveUp(m))?;

            },
            CursorMoveTo::Down(m) => {
                self.out.queue(MoveDown(m))?;
            },
            CursorMoveTo::Left(m) => {
                self.out.queue(MoveLeft(m))?;
            },
            CursorMoveTo::Right(m) => {
                self.out.queue(MoveRight(m))?;
            },
            CursorMoveTo::Pos(v) => {
                self.out.queue(MoveTo(v.0 as u16 ,v.1 as u16))?;
            },
        }
        Ok(())
    }
    pub fn flush(&mut self) -> Ret{
        self.out.flush()?;
        Ok(())
    }


    pub fn init_term(&mut self) -> Ret{
        self.out.queue(EnterAlternateScreen)?;
        enable_raw_mode()?;
        self.out.queue(cursor::Hide)?;
        #[cfg(not(windows))]
        self.out.queue(PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES))?;
        self.out.flush()?;


        Ok(())
    }
    pub fn deinit_term(&mut self) -> Ret{
        #[cfg(not(windows))]
        self.out.queue(PopKeyboardEnhancementFlags)?;
        self.out.queue(cursor::Show)?;
        disable_raw_mode()?;
        self.out.queue(LeaveAlternateScreen)?;
        self.flush()?;
        Ok(())

    }

    pub fn queue<T:Borrow<TermPrint>>(&mut self,s:T) -> Ret{
        let mut s = s.borrow().clone();
        if s.background == GColor::Default{
            s.background = self.bg.0;
        }
        if s.fore == GColor::Default{
            s.fore = self.bg.1;
        }
        self.out.queue(Print(s))?;
        Ok(())
    }
    pub fn print_raw<S:Into<String>>(&mut self,str:S) -> Ret{
        let str = str.into();
        print!("{}",str);
        Ok(())
    }
    pub fn get_size() -> RetType<Vec2>{
        Ok(Vec2::from(crossterm::terminal::size()?))
    }
    pub fn _get_size(&self) -> RetType<Vec2>{
        Ok(Vec2::from(crossterm::terminal::size()?))
    }

    pub fn print_title(&mut self,extra:Option<String>) -> Ret{
        let title = format!("\x1b]0;{}{}\x07",self.title,extra.unwrap_or("".to_string()));
        self.print_raw(title)?;
        Ok(())
        
    }

    pub(crate) fn reset_color(&mut self) -> Ret{
        let bg = TermPrint::from(("",self.bg.0,self.bg.1));

        self.out.queue(Print(&bg))?;
        Ok(())
    }

    pub(crate) fn clear_bg(&mut self) -> std::io::Result<()>{
        let (column,line) = crossterm::terminal::size()?;
        
        self.out.queue(Clear(ClearType::All))?;
        let str = self.bg.2.to_string().repeat((column * line ).into());
        self.out.queue(cursor::MoveTo(0, 0))?;
        self.out.queue(SetBackgroundColor(Color::from(self.bg.0)))?;
        self.out.queue(SetForegroundColor(Color::from(self.bg.1)))?;
        self.out.queue(crossterm::style::Print(str))?;
    
        Ok(())
    }


}
