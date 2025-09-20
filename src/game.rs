use std::{collections::HashSet, io::{stdout, Cursor, Write}, time::{Duration, Instant}};
use crossterm::{cursor, event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags}, execute, style::{Color, SetBackgroundColor, SetForegroundColor}, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, QueueableCommand};
use crate::{math::Vec2, GameObject, Keys, Object, TermPrint};
type Ret = std::io::Result<()>;
type RetTick = std::io::Result<bool>;


macro_rules! impl_tick {
    ($($num:literal => $meth:ident),* $(,)?) => {
        pub fn tick(&mut self, id: i32) -> RetTick {
            // code that runs before any tick
            let mut res;
            res = self.tick_enter()?;
            if !res{return Ok(res)}

            res = match id {
                $(
                    $num => self.$meth()?,
                )*
                _ => panic!("no tick found for id {id}"),
            };
            if !res {return Ok(res)}
            // code that runs after any tick
            res = self.tick_exit()?;

            Ok(res)
        }
    };
}




pub struct Game{
    pub out:std::io::Stdout,
    pub title:String,
    pub last_frame: Instant,
    pub pressed_keys:HashSet<Keys>,
    pub just_pressed_keys:HashSet<Keys>,
    pub target_fps:f32,
    pub flags:bool,
    pub posy:u16,
    pub logs:Vec<String>,
    pub player:GameObject,
}

impl Game {
    pub fn new(title:String,target_fps:f32) -> Self{
        
        
        let mut texture = Vec::new();
        
        texture.push(crate::PrintThing { rel_pos: Vec2::from((0,0)), text: TermPrint::from("--------") });
        texture.push(crate::PrintThing { rel_pos: Vec2::from((7,1)), text: TermPrint::from("-")});
        texture.push(crate::PrintThing { rel_pos: Vec2::from((-1,-1)), text: TermPrint::from("@")});
        
        let player = GameObject{
            pos:Vec2 { x: 2, y: 2 },
            texture
        };
        


        
        Self { out: stdout(), title,
            player,
            target_fps,
            last_frame: Instant::now(),
            pressed_keys: HashSet::new(),
            just_pressed_keys: HashSet::new(),
            flags:false,
            posy:0,
            logs:Vec::new(),
        }
    }   
    pub fn log(&mut self,l:String){
        self.logs.push(l);
    }
    pub fn set_target_fps(&mut self ,fps:f32) -> Result<(),&str>{
        if fps < 0. {
            return Err("fps must be positive");
        }
        self.target_fps = fps;
        Ok(())
    }
    pub fn should_render(&self) -> bool{
        let frame_time = Duration::from_secs_f32(1.0 / self.target_fps);
        self.last_frame.elapsed() >= frame_time 

    }
    pub fn setup(&mut self) -> Ret{
        self.out.execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        self.out.execute(cursor::Hide)?;
        #[cfg(not(windows))]
        self.out.execute(PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES))?;
        print!("\x1b]0;{}\x07",self.title);
        self.out.flush()?;
        Ok(())
        
    }
    pub fn exit(&mut self) -> Ret{
        #[cfg(not(windows))]
        self.out.execute(PopKeyboardEnhancementFlags)?;
        self.out.execute(cursor::Show)?;
        disable_raw_mode()?;
        self.out.execute(LeaveAlternateScreen)?;
        Ok(())
    }
    fn print_title(&self,extra:Option<String>){
        print!("\x1b]0;{}{}\x07",self.title,extra.unwrap_or("".to_string()));
    }
    pub fn update_delta(&mut self){
        self.last_frame = Instant::now();
    }
    pub fn print(&mut self,text:String) -> Ret{
        self.out.execute(cursor::MoveTo(0,self.posy))?;
        print!("{}",text);
        self.posy += 1;
        let (_column,_line) = crossterm::terminal::size()?;
        if self.posy == _line {
            self.posy = 0;
            self.clear_bg()?;
        }
        self.out.flush()?;
        Ok(())
    }
    pub fn poll_keys(&mut self) -> Ret{
        while event::poll(Duration::from_millis(1))? {
            if let Event::Key(k) = event::read()? {
                self.log(format!("polled event {:?}",k));

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

    pub fn tick_enter(&mut self) -> RetTick{
        self.clear_bg()?;
        if self.just_pressed_keys.contains(&Keys::Debug){
            if self.flags{
                self.print_title(None);
            }
            self.flags = !self.flags;
        };
        if self.flags {
            let fps = 1.0 / self.last_frame.elapsed().as_secs_f32();
            self.print_title(Some(format!(" fps:{:.1}  p {:?}",fps,self.pressed_keys)));

        }
        Ok(true)
    }
    pub fn tick_exit(&mut self) -> RetTick{
        if self.pressed_keys.contains(&Keys::Esc){
            return Ok(false);
        }
        self.out.flush()?;
        self.just_pressed_keys.clear();
        self.update_delta();
        Ok(true)
    }

    impl_tick!(
        1 => tick1,
        2 => tick2,
    );

    pub fn tick2(&mut self) -> RetTick{

        let mut p = Vec2::from((0,0));
        
        let mut pr = TermPrint{
            background : crate::RGB { r: 0, g: 255, b: 0, res: false },
            foreground : crate::RGB { r: 0, g: 0, b: 0, res: true },
            text:"sa".to_string()
        };
        
        self.out.queue(cursor::MoveTo(0,0))?;
        print!("{}",pr);
        p.y += 1;
        self.out.queue(cursor::MoveTo(p.x as u16,p.y as u16))?;
        pr.text = "as".to_string();
        print!("{}",pr);


        Ok(true)
    }
    pub fn tick1(&mut self) -> RetTick{

        self.player.print(&mut self.out)?;
        
        
        let mut pos = self.player.get_cords();

        for key in &self.pressed_keys { 
            match key {
                Keys::Up => pos.y -=1,
                Keys::Down => pos.y +=1,
                Keys::Left => pos.x -=2,
                Keys::Right => pos.x +=2,
                _ => (),
            }
        }
        self.player.set_cords(pos.x, pos.y);
        
        Ok(true)
    }
    pub fn clear_bg(&mut self) -> std::io::Result<()>{
        let (column,line) = crossterm::terminal::size()?;
        
        self.out.queue(Clear(ClearType::All))?;
        let str = " ".repeat((column * line ).into());
        self.out.queue(cursor::MoveTo(0, 0))?;
        self.out.queue(SetBackgroundColor(Color::Rgb { r: 50, g: 100, b: 50 }))?;
        //self.out.queue(SetForegroundColor(Color::Rgb { r: 50, g: 100, b: 50 }))?;
        self.out.queue(crossterm::style::Print(str))?;
    
        Ok(())
    }
}




impl Drop for Game {
    fn drop(&mut self) {
        self.exit().unwrap();
        
        if std::thread::panicking() {
            eprintln!("game crashed lol")
        }else {
            println!("game exited")
        }
        println!("start log");
        for x in &self.logs{
            println!("{}",x)
        }
        println!("end log")
    }
}