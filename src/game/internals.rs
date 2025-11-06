use std::{any::Any, borrow::Borrow, time::Duration};

use crate::{Ret, game::{Game, get_logger}};






// internal functions of Game
impl<'a> Game<'a>{
    pub fn set_log_path(&self,path:Option<String>){
        let l = get_logger();
        let mut log = l.lock().unwrap();
        log.set_path(path);
    }

    /// 0 means maximum
    pub fn set_target_fps(&mut self ,fps:f32) -> Result<(),&str>{
        if fps < 0. {
            return Err("fps must be positive");
        }
        self.timing.physics_fps = fps;
        Ok(())
    }
    pub fn should_render(&self) -> bool{
        let frame_time = Duration::from_secs_f32(1.0 / self.timing.physics_fps);
        self.timing._last_physics_frame.elapsed() >= frame_time 
    }
    pub fn setup(&mut self) -> Ret{
        self.screen.init_term()?;

        self.print_title(None)?;

        Ok(())
    }
    pub fn exit(&mut self) -> Ret{
        self.screen.deinit_term()?;
        Ok(())
    }
    pub(super) fn print_title(&mut self,extra:Option<String>) -> Ret{
        let title = format!("\x1b]0;{}{}\x07",self.title,extra.unwrap_or("".to_string()));
        self.screen.print_raw(title)?;
        Ok(())
        
    }

    /// send an any signal
    /// 
    /// 
    pub fn send_signal<S:Borrow<String>>(&self,hint:S ,to: S,msg: Box<dyn Any + Send>){
        let (from,to) = (hint.borrow(),to.borrow());
        let sys = self.get_systems_mut();
        sys.send_signal(from, to, msg);
    }

}