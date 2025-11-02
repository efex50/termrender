use std::{borrow::Cow, collections::HashMap};

use crate::{game::Game, RetTick};


pub struct Systems<'a>{
    pub(crate) sys:HashMap<String,System<'a>>
}
impl<'a> Systems<'a> {
    pub fn new() -> Self{
        Self { sys: HashMap::new() }
    }
    pub fn new_system<St:Into<String>>(&mut self,s:Box<dyn GameSystem>,name:St,active:bool ){
        let name = name.into();
        self.sys.insert(name.clone(),System::new(name, s,active));
    }
}

pub struct System<'a>{
    name:Cow<'a,String>,
    pub fun:Box<dyn GameSystem>,
    active:bool,
}
impl<'a> System<'a> {
    pub fn new(name: String, sys: Box<dyn GameSystem>,active:bool) -> Self {
        let n= Cow::<'a,String>::Owned(name);
        Self { name:n, fun: sys,active }
    }
    fn activate(&mut self,world:&mut Game) -> RetTick{
        self.active = true;
        self.fun._setup(&self.name, world)?;
        
        Ok(true)
    }
}
pub trait GameSystem{
    fn _setup(&mut self,                _sys_name:&String,   _game:&mut Game)                     -> RetTick{Ok(true)}
    fn _physics_loop(&mut self,         _sys_name:&String,   _game:&mut Game)                     -> RetTick{Ok(true)}
    fn _process_loop(&mut self,         _sys_name:&String,   _game:&mut Game)                     -> RetTick{Ok(true)}
    fn _om_signal_recieved(&mut self,   _sys_name:&String,   _game:&mut Game,     _signal:&String) -> RetTick{Ok(true)}
    fn _kill_system(&mut self,          _sys_name:&String,   _game:&mut Game)                     -> RetTick{Ok(true)}
}