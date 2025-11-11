use std::any::Any;

use crate::{Ret, game::{Flags, Game, get_logger}};






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
    pub fn setup(&mut self) -> Ret{
        self.screen.init_term()?;

        self.screen.print_title(None)?;

        Ok(())
    }
    pub fn exit(&mut self) -> Ret{
        self.screen.deinit_term()?;
        Ok(())
    }


    pub fn set_flag(&self,flag:Flags) {
        let g = self.get_self_mut();
        g.flags.insert(flag);
    }
    pub fn has_flag(&self,flag:Flags) -> bool {
        self.flags.contains(&flag)
    }
    pub fn unset_flag(&self,flag:Flags) {
        let g = self.get_self_mut();
        g.flags.remove(&flag);
    }



    /// send an any signal
    /// 
    /// 
    pub fn send_signal<S1:Into<String>,S2:Into<String>>(&self,hint:S1 ,to: S2,msg: Box<dyn Any + Send>){
        let (from,to) = (hint.into(),to.into());
        let sys = self.get_systems_mut();
        sys.send_signal(&from, &to, msg);
    }
    
    /// register signal to systems with name
    /// 
    /// 
    pub fn register_signal<S1:Into<String>,S2:Into<String>>(&self,sig_name:S1 ,sys_name: S2) -> Ret{
        let (sig_name,sys_name) = (sig_name.into(),sys_name.into());
        let sys = self.get_systems_mut();
        sys.register_signal(&sys_name, &sig_name)
    }
    pub fn unregister_signal<S1:Into<String>,S2:Into<String>>(&self,sig_name:S1 ,sys_name: S2)  -> Ret{
        let (sig_name,sys_name) = (sig_name.into(),sys_name.into());
        let sys = self.get_systems_mut();
        sys.unregister_signal(&sys_name, &sig_name)
    }

}