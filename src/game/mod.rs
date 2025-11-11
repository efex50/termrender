use std::{backtrace::Backtrace, collections::HashSet,fs::OpenOptions, io::{Write, stdout}, sync::{Arc, Mutex}, time::Duration};
use crate::{GAME_STARTED, LOG, Ret, RetTick, RetType, game::{input::{Input, Keys}, logger::get_logger, screen::Screen, systems::Systems, timing::Timing}, gameobject::ObjectHeader, math::Vec2, prelude::{Components, signals::RESIZED}, print::GColor};


pub mod systems;
pub mod input;
pub mod subgamegetters;
pub mod internals;
pub mod logger;
pub mod screen;

pub mod timing;
pub mod signal_types;

pub const GAME_NAMESPACE:&str = "game";

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
    pub fn get_with_component(&mut self,comp:Components) -> Vec<&mut ObjectHeader>{
        let w = self.self_p();
        let mut v = Vec::new();
        for x in w.objects.iter_mut(){
            if x.components.contains(&comp){
                unsafe {
                    v.push(
                        (x as *mut ObjectHeader).as_mut().unwrap()
                    );
                };
            }
        }
        v
    }
    pub fn get_with_components(&mut self,comp:Vec<Components>) -> Vec<&mut ObjectHeader>{
        let w = self.self_p();
        let mut v = Vec::new();
        for obj in w.objects.iter_mut(){
            let has_all = {
                let mut complete = false;
                'main:for x in &comp{
                    for y in &obj.components{
                        if x == y{
                            complete = true;
                            continue;
                        }
                        complete = false;
                        break 'main;
                    }
                }
                complete
            };
            if has_all{
                unsafe {
                    v.push(
                        (obj as *mut ObjectHeader).as_mut().unwrap()
                    );
                };
            }
        }
        v
    }
    pub fn query_comp(&self,comp:Components) -> Option<&mut ObjectHeader>{
        let w = self.self_p();
        for x in w.objects.iter_mut(){
            if x.components.contains(&comp){
                let p = unsafe{(x as *mut ObjectHeader).as_mut().unwrap()};
                return Some(p);
            }
        };
        None
    }
    pub fn insert_object_head(&mut self,header: ObjectHeader) -> RetType<usize> {
        let mut o = ObjectHeader::from(header);
        let id = self.objects.len();
        o.id = id;
        if let Some(p) = o.attributes.get_Location(){
            o.previus = *p;
        }
        self.objects.push(o);
        Ok(id)
    }


    /// iterates every object in wordl
    /// 
    /// don't do it
    pub fn map_objects(&mut self,mut map:impl FnMut(&mut ObjectHeader)){
        for x in self.objects.iter_mut(){
            (map)(x)
        }
    }

    fn self_p(&self) -> &mut Self{
        let a = unsafe { (((self as *const _) as usize) as *mut Self).as_mut().unwrap() };
        a
    }

}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Flags{
    Debug,
    Rerender,
    Resized,
    ForceRerender,
    Custom(String),
}

pub struct Game<'a>{
    panic_logs: Arc<Mutex<Vec<String>>>,
    flags: HashSet<Flags>,
    is_started: bool,

    pub screen:     Screen,
    pub timing:     Timing,
    pub input:      Input,
    pub world:      World,
    pub systems:    Systems<'a>,

}


impl<'a> Game<'a> {
    pub fn new(title:String,physics_fps:f32,process_fps:f32,render_fps:f32) -> Self{
        
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
        let screen = Screen {
            bg:(GColor::RGB(50, 100, 150),GColor::White,' '),
            out:stdout(),
            screen,
            posy:0,
            title:title,
        };

        let world = World {
            objects:Vec::new()
        };
        let mut timing = Timing::new();
        timing.physics_fps = physics_fps;
        timing.process_fps = process_fps;
        timing.render_fps  = render_fps;

        let input = Input::new(Duration::from_millis(1));
        // init the timestamp of the game start
        let _ = GAME_STARTED.elapsed();
        Self { 
            input,
            screen,
            timing,
            world,
            flags:HashSet::new(),
            is_started:false,
            systems: Systems::new(),
            panic_logs
        }
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
    fn tick_once(&mut self) -> RetTick{
        self.screen.clear_bg()?;
        for (_,sys) in &mut self.get_systems_mut().sys{
            LOG!(Debug,"activating {:?}",sys);
            sys.activate(self.get_self_mut())?;
        }
        Ok(true)
    }
    fn tick_once_last(&mut self) -> RetTick{
        self.rerender_all()?;
        self.is_started = !self.is_started;
        Ok(true)
    }
    fn tick_enter(&mut self) -> RetTick{

        if !self.is_started{
            if !self.tick_once()?{
                return Ok(false);
            }
        };
        let screen = Screen::get_size()?;
        if screen != self.screen.screen{
            self.set_flag(Flags::Resized);
            self.get_systems_mut().send_signal(RESIZED, GAME_NAMESPACE,Box::new(self.screen._get_size()?));
            self.set_flag(Flags::Rerender);
            // send resized signal
            self.screen.screen = screen;
        }
        
        //self.clear_bg()?;

        // handle the signals
        let s = self.get_systems_mut();
        let g = self.get_self_mut();
        s.deliver_signals(g)?;
        Ok(true)
    }
    
    fn tick(&mut self) -> RetTick{
        if self.input.pressed_keys.contains(&Keys::Esc){
            return Ok(false);
        }

        let mut res;
        // code that runs before any ticl
        res = self.tick_enter()?;
        if !res{return Ok(res)}

        let g = self.get_self_mut();
        let t = self.get_timing_mut();
        // every physics tics
        if t.should_physics(){
            for (name,sys) in &mut self.get_systems_mut().sys{
                if sys.is_active(){
                    if !sys.is_init(){
                        LOG!(Debug,"Activating {:?}",sys);
                        sys.activate(g)?;
                    }
                    res = sys.fun._physics_loop(t.get_delta_physics(),name,g)?
                }
                if !res {return Ok(res);}
            }
            t.update_physics_delta();
        }
        // every process tick
        // basicly same for now except timing
        if t.should_process(){
            for (name,sys) in &mut self.get_systems_mut().sys{
                if sys.is_active(){
                    if !sys.is_init(){
                        LOG!(Debug,"Activating {:?}",sys);
                        sys.activate(g)?;
                    }
                    res = sys.fun._process_loop(t.get_delta_process(),name,g)?
                }
                if !res {return Ok(res);}
            }
            t.update_process_delta();
        }
     
        // rendering
        if t.should_render(){
            self.tick_render()?;
            self.get_timing_mut().update_render_delta();
        };
        if !self.is_started{
            if !self.tick_once_last()?{
                return Ok(false);
            }
        };
        self.unset_flag(Flags::Resized);
        self.input.just_pressed_keys.clear();
        Ok(true)
    }



    fn tick_render(&mut self) -> RetTick{            
        // change the debug flag
        if self.has_flag(Flags::Debug){
            if self.has_flag(Flags::Custom("d".to_string())){
                self.screen.print_title(None)?;
                self.unset_flag(Flags::Custom("d".to_string()));
            }else {
                let fps = 1.0 / self.timing.get_delta_render().as_secs_f32();
                self.screen.print_title(Some(format!(" fps:{:.1}  press {:?} screen {:?}",fps,self.input.pressed_keys,self.screen.screen)))?;
                self.set_flag(Flags::Custom("d".to_string()));
            }
            self.unset_flag(Flags::Debug);
        }else if self.has_flag(Flags::Custom("d".to_string())){
                let fps = 1.0 / self.timing.get_delta_render().as_secs_f32();
                self.screen.print_title(Some(format!(" fps:{:.1}  press {:?} screen {:?}",fps,self.input.pressed_keys,self.screen.screen)))?;
                self.set_flag(Flags::Custom("d".to_string()));

        }
        


        if self.has_flag(Flags::Rerender) || self.has_flag(Flags::ForceRerender){
            self.rerender_all()?;
            self.unset_flag(Flags::Rerender);
            self.unset_flag(Flags::ForceRerender);
        }else{


            for x in &mut self.get_wolrd_mut().objects{
                let mut scr = self.get_screen_mut();
                scr.reset_color()?;
            
                if self.has_flag(Flags::Resized){
                    self.set_flag(Flags::Rerender);
                }else {
                    match x.should_render() {
                        crate::gameobject::ShouldRender::ForceRerender |
                        crate::gameobject::ShouldRender::Changed  => {
                            scr.reset_color()?;
                            x.clearself(&mut scr)?;
                            x.print(&mut scr)?;

                        },
                        crate::gameobject::ShouldRender::Unchanged => (),
                        crate::gameobject::ShouldRender::Disabled |
                        crate::gameobject::ShouldRender::Clear => {
                            scr.reset_color()?;
                            x.clearself(&mut scr)?;

                        },
                    };
                }
            }
        }
        self.screen.out.flush()?;
        
        Ok(true)
    }

    pub fn main_loop(&mut self) -> Ret{
        loop {

            self.input.poll_keys()?;
            self.organize_flags();
            let r = self.tick()?;
            if !r {break};

        }
        Ok(())
    }
    pub(crate) fn organize_flags(&mut self){
        if self.input.just_pressed_keys.contains(&Keys::Debug){
            self.set_flag(Flags::Debug);
        }
        if self.input.just_pressed_keys.contains(&Keys::Refresh){
            self.set_flag(Flags::Rerender);
        }
    }
    fn rerender_all(&mut self) -> Ret{
        let mut screen = self.get_screen_mut();
        screen.clear_bg()?;
        for x in &mut self.get_wolrd_mut().objects{
            //x.clearself(&mut self.terminal.out)?;
            match x.should_render() {
                crate::gameobject::ShouldRender::ForceRerender |
                crate::gameobject::ShouldRender::Unchanged |
                crate::gameobject::ShouldRender::Changed => {
                    x.print(&mut screen)?;
                },
                crate::gameobject::ShouldRender::Clear |
                crate::gameobject::ShouldRender::Disabled => {
                    x.clearself(&mut screen)?;
                },

            };
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
        let log = get_logger();
        let logs = log.lock().unwrap();
        if let Some(path) = &logs.log_path {
            let mut tries = 0;
            loop {
                let num ={if tries == 0 {"".to_string()} else {format!(".{}",tries)}};
                let pathname = format!("{}{}.log",path,num);
                let p = std::path::Path::new(&pathname);
                if p.exists(){
                    tries += 1;
                    continue;
                }else {
                    let mut file = OpenOptions::new()
                        .create(true)   // create if not exists
                        .append(true)   // open for appending
                        .open(p).unwrap(); 

                    for x in logs.logs.iter(){
                        let clean = strip_ansi_escapes::strip_str(x);
                        file.write_all(clean.as_bytes()).unwrap();
                        file.write(&[b'\n']).unwrap();
                    }
                    break;
                }
            }
        }

        for x in logs.logs.iter(){
            println!("{}",x)
        }
        println!("end log")
    }
}