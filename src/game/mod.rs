use std::{backtrace::Backtrace, collections::{HashSet}, io::{stdout, Write}, sync::{Arc, Mutex}, time::{Duration, Instant}};
use crate::{GAME_STARTED, Ret, RetTick, RetType, TermPrint, game::{input::{Input, Keys}, logger::{ get_logger}, screen::Screen, systems::Systems}, gameobject::ObjectHeader, math::Vec2, print::RGB};


pub mod systems;
pub mod input;
pub mod subgamegetters;
pub mod internals;
pub mod logger;
pub mod screen;

pub struct Timing {
    _last_physics_frame: Instant,
    _last_process_frame: Instant,
    physics_fps: f32,
    process_fps:f32,
}
impl Timing {
    pub fn new() -> Self{
        let now = Instant::now();
        Self{
            _last_physics_frame:now,
            _last_process_frame:now,
            physics_fps:30.,
            process_fps:30.,
        }
    }
    pub fn get_delta_physics(&self) -> Duration{
        self._last_physics_frame.elapsed()
    }
}


pub struct World {
    pub objects: Vec<ObjectHeader>,
}
impl World {
    pub fn get_with_id(&mut self,idx:usize) -> Option<&mut ObjectHeader>{

        if let Some(obj) = self.objects.get_mut(idx){
            let p = unsafe {
                (obj as *mut ObjectHeader).as_mut().unwrap()
            };
            Some(p)
        }else {
            None
        }
    }    
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Flags{
    Debug,
    Custom(String),
}

pub struct Game<'a>{
    pub title: String,
    panic_logs: logger::Logger,
    flags: HashSet<Flags>,
    is_started: bool,

    pub screen:     Screen,
    pub timing:     Timing,
    pub input:      Input,
    pub world:      World,
    pub systems:    Systems<'a>,

}


impl<'a> Game<'a> {
    pub fn new(title:String,physics_fps:f32,process_fps:f32) -> Self{
        
        
        let panic_logs: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let hook_logs = Arc::clone(&panic_logs);
        std::panic::set_hook(Box::new(move |info| {

            let msg = format!("panic: {info}");
            eprintln!("{msg}");
            // also push into logs
            if let Ok(mut l) = hook_logs.lock() {
                l.push(format!("{}",msg));
            }
        }));

        let screen = Vec2::from(crossterm::terminal::size().unwrap());
        let terminal = Screen {
            bg:(RGB { r: 50, g: 100, b: 150 ,res:false},RGB::from(true),' '),
            out:stdout(),
            screen,
            posy:0
        };

        let world = World {
            objects:Vec::new()
        };
        let mut timing = Timing::new();
        timing.physics_fps = physics_fps;
        timing.process_fps = process_fps;

        let input = Input::new(Duration::from_millis(1));
        // init the timestamp of the game start
        let _ = GAME_STARTED.elapsed();
        Self { 
            input,
            screen: terminal,
            timing,
            world,
            title,
            flags:HashSet::new(),
            is_started:false,
            systems: Systems::new(),
            panic_logs
        }
    }   
    pub fn update_delta(&mut self){
        self.timing._last_physics_frame = Instant::now();
    }
    pub fn print(&mut self,text:String) -> Ret{
        self.screen.cursor_move(screen::CursorMoveTo::Pos((0, self.screen.posy)))?;
        print!("{}",text);
        self.screen.posy += 1;
        let (_column,_line) = crossterm::terminal::size()?;
        if self.screen.posy == _line {
            self.screen.posy = 0;
            self.screen.clear_bg()?;
        }
        self.screen.flush()?;
        Ok(())
    }
    pub fn insert_object_head(&mut self,header: ObjectHeader) -> RetType<usize> {
        let mut o = ObjectHeader::from(header);
        let id = self.world.objects.len();
        o.id = id;
        if let Some(p) = o.attributes.get_Location(){
            o.previus = *p;
        }
        self.world.objects.push(o);
        Ok(id)
    }

    fn tick_once(&mut self) -> RetTick{
        self.screen.clear_bg()?;

        for (name,sys) in &mut self.get_systems_mut().sys{
            if !sys.fun._setup(name,self.get_self_mut())?{
                return Ok(false);
            }
        }
        self.rerender_all()?;
        Ok(true)
    }
    fn tick_enter(&mut self) -> RetTick{

        if !self.is_started{
            if !self.tick_once()?{
                return Ok(false);
            }

            self.is_started = !self.is_started;
        };
        let screen = Vec2::from(crossterm::terminal::size()?);
        if screen != self.screen.screen{
            self.rerender_all()?;
            self.screen.screen = screen;
        }
        
        //self.clear_bg()?;

        // change the debug flag
        if self.input.just_pressed_keys.contains(&Keys::Debug){
            if self.flags.contains(&Flags::Debug){
                self.print_title(None)?;
                self.flags.remove(&Flags::Debug);
            }else {
                self.flags.insert(Flags::Debug);
            }
        };
        if self.input.just_pressed_keys.contains(&Keys::Refresh){
            self.rerender_all()?;
        };
        if self.flags.contains(&Flags::Debug) {
            let fps = 1.0 / self.timing._last_physics_frame.elapsed().as_secs_f32();
            self.print_title(Some(format!(" fps:{:.1}  press {:?} screen {:?}",fps,self.input.pressed_keys,self.screen.screen)))?;

        }
        Ok(true)
    }
    fn tick_exit(&mut self) -> RetTick{
        if self.input.pressed_keys.contains(&Keys::Esc){
            return Ok(false);
        }
        let mut bg = TermPrint::from((self.screen.bg.2.to_string().as_str(),self.screen.bg.0,self.screen.bg.1));
        bg.text = "".to_string();
        
        
        for x in &mut self.get_wolrd_mut().objects{
            if x.attributes.check_Texture() && x.attributes.check_Location() && x.should_render(){
                let mut scr = self.get_screen_mut();
                scr.reset_color()?;
                x.clearself(&mut scr)?;
                x.print(&mut scr)?;
            }else {
            }
        }
        self.screen.out.flush()?;
        self.input.just_pressed_keys.clear();
        self.update_delta();
        Ok(true)
    }
    
    fn tick(&mut self) -> RetTick{
        let mut res;
        // code that runs before any ticl
        res = self.tick_enter()?;
        if !res{return Ok(res)}

        
        for (name,sys) in &mut self.get_systems_mut().sys{
            res = sys.fun._physics_loop(name,self.get_self_mut())? & sys.fun._process_loop(name,self.get_self_mut())?;

            if !res {return Ok(res);}
        }
     
        // code that runs after any tick
        self.tick_exit()
    }

    pub fn main_loop(&mut self) -> Ret{
        loop {

            self.input.poll_keys()?;

            if self.should_render(){ 

                let r = self.tick()?;
                if !r {break};

            }        
        }
        Ok(())
    }

    pub fn force_rerender(&mut self) ->Ret{
        self.rerender_all()
    }

    fn rerender_all(&mut self) -> Ret{
        let mut screen = self.get_screen_mut();
        screen.clear_bg()?;
        for x in &mut self.get_wolrd_mut().objects{
            //x.clearself(&mut self.terminal.out)?;
            x.print(&mut screen)?;
        }
        screen.flush()?;
        Ok(())
    }
}


impl<'a> Drop for Game<'a> {
    fn drop(&mut self) {
        self.exit().unwrap();
        
        if std::thread::panicking() {
            eprintln!("game crashed lol");
            let plogs = self.panic_logs.lock().unwrap();
            for x in plogs.iter(){
                eprintln!("{}",x)
            };
            let bt = Backtrace::capture();
            match bt.status(){
                std::backtrace::BacktraceStatus::Captured => {
                    eprintln!("{}",bt)
                },
                _ =>  eprintln!("note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace"),
            }
        }else {
            println!("game exited")
        }
        println!("start log");
        let logs = get_logger();
        let logs = logs.lock().unwrap();

        for x in logs.iter(){
            println!("{}",x)
        }
        println!("end log")
    }
}