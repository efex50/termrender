use std::time::Duration;

use crate::{Ret, game::{Game, get_logger}};






// internal functions of Game
impl<'a> Game<'a>{
    pub fn log(&self,l:String){
        let logs = get_logger();
        let mut logs = logs.lock().unwrap();
        logs.push(l);
    }
    pub fn log_arr(&mut self,mut l:Vec<String>){
        let logs = get_logger();
        let mut logs = logs.lock().unwrap();
        logs.append(&mut l);
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
        self.screen.queque(title)?;
        Ok(())
        
    }

}