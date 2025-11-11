use std::{any::Any, borrow::Cow, collections::{HashMap, HashSet}, fmt::Debug, time::Duration};

use crate::{LOG, Ret, RetTick, game::Game};

#[macro_export]
macro_rules! impl_sys {
    () => {
        fn as_any(&self) -> &dyn std::any::Any{
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any{
            self
        }

    };
}
pub struct Message{
    pub hint:String,
    pub target:String,
    pub msg:Box<dyn Any + Send>,
}

pub struct Systems<'a>{
    pub(crate) sys:HashMap<String,System<'a>>,
    pub(crate) pending_signals:Vec<Message>,
}
impl<'a> Systems<'a> {
    pub fn new() -> Self{
        Self { sys: HashMap::new() , pending_signals:Vec::new()}
    }
    pub fn new_system<St:Into<String>>(&mut self,s:Box<dyn GameSystem>,name:St,active:bool){
        
        let name = name.into();
        let sys = System::new(name.clone(), s,active);
        self.sys.insert(name.clone(),sys);
    }
    pub fn _new_system<St:Into<String>>(&mut self,s:impl GameSystem + 'static , name:St,active:bool){
        self.new_system(Box::new(s), name, active);
    }
    pub fn new_system_with_helper<St:Into<String>>(&mut self,s:Box<dyn GameSystem>,name:St,active:bool,mut helper:Closure<'a>){
        let name = name.into();
        let mut sys = System::new(name.clone(), s,active);
        (helper)(&mut sys);
        self.sys.insert(name.clone(),sys);
    }
    pub fn push_system<>(&'a mut self,s:System<'a>){
        self.sys.insert(s._name.to_string(), s);
    }
    pub fn push_system_builder<>(&'a mut self,s:SystemBuilder){
        self.sys.insert(s._name.to_string(), s.build());
    }
    pub(crate) fn send_signal(&mut self,hint:&'a str,to:&'a str,msg:Box<dyn Any + Send>){
        let m: Message = Message{
            hint:hint.to_string(),
            target:to.to_string(),
            msg:msg
        };
        self.pending_signals.push(m);
    }
    pub(crate) fn register_signal(&mut self,sys_name:&'a str,sig_name:&'a str,) -> Ret{
        let s = self.sys.get_mut(sys_name)
            .map_or(Err(std::io::Error::from_raw_os_error(31)), |a| Ok(a))?;
        s._registered_signals.insert(sig_name.to_owned());
        Ok(())
    }
    pub(crate) fn unregister_signal(&mut self,sys_name:&'a str,sig_name:&'a str) -> Ret{

        let s = self.sys.get_mut(sys_name)
            .map_or(Err(std::io::Error::from_raw_os_error(31)), |o| Ok(o))?;

        s.unregister_signal(sig_name);
        Ok(())
    }
    // sinyalleri tarar ve hedeflere gönderir
    pub(crate)  fn deliver_signals(&mut self,g:&mut Game) -> Ret{
        for msg in self.clone_self().pending_signals.iter_mut(){
            if let Some(target) = self.clone_self().sys.get_mut(&msg.target){
                target.fun._on_signal_recieved(&target._name, g, msg, &msg.hint.clone())?;
            }else {
                let sigs = self.get_system_with_registered(&msg.hint);
                for x in sigs{
                    if let  Some(x) = self.clone_self().sys.get_mut(&x){
                        x.fun._on_signal_recieved(&x._name, g, msg, &msg.hint.clone())?;
                    }
                }
            }
        }
        self.pending_signals.clear();
        Ok(())
    }
    pub fn get_system_with_registered(&self,sig_name:&String) -> Vec<String>{
        let mut sigs = Vec::new();
        for x in self.sys.iter(){
            if x.1._registered_signals.contains(sig_name){
                sigs.push(x.0.clone());
            }
        };
        sigs
    }
    pub fn get_system_with_name<St:Into<String>>(&mut self,name:St) -> Option<&mut System<'a>> {
        let name = name.into();
        self.sys.get_mut(&name)
    }
    /// get system with its name
    pub fn get_system_as<T: 'static, St: Into<String>>(&'a self, name: St) -> Option<&'a T> {
        let name = name.into();
        self.sys.get(&name)
            .and_then(|s| s.fun.as_any().downcast_ref::<T>())
    }
    /// get system with its name
    pub fn get_system_mut_as<T: 'static, St: Into<String>>(&'a mut self, name: St) -> Option<&'a mut T> {
        let name = name.into();
        self.sys.get_mut(&name)
            .and_then(|s| s.fun.as_any_mut().downcast_mut::<T>())
    }

    pub fn clone_self(&self) -> &mut Systems<'_>{
        unsafe {
            (((self as *const _) as usize) as *mut Systems).as_mut().unwrap()
        }

    }
}


type Closure<'a> = Box<dyn FnMut(&mut System) + 'a>;
pub struct SystemBuilder{
    _name:String,
    //_config:impl FnMut(&mut System),
    fun:Box<dyn GameSystem>,
}
impl SystemBuilder {
    pub fn new(sys:Box<dyn GameSystem>) -> Self{
        let generated_name = uuid::Uuid::new_v4();
        Self { 
            _name: generated_name.to_string(),
            fun:sys
        }
    }
    pub fn with_name<S:Into<String>>(mut self,name:S) -> Self{
        let name = name.into();
        self._name = name;
        self
    }
    pub fn build<'a>(self) -> System<'a>{
        let mut s = System::new(self._name.to_string(), self.fun, false);
        s
    }
}

pub struct System<'a>{
    pub(crate) _name:Cow<'a,String>,
    pub fun:Box<dyn GameSystem>,
    _init:bool,
    _active:bool,
    pub(crate) destroyed:bool,
    _registered_signals:HashSet<String>,
}
impl<'a> System<'a> {
    pub fn new(name: String, sys: Box<dyn GameSystem>,active:bool) -> Self {
        let n= Cow::<'a,String>::Owned(name);
        Self { _name:n, fun: sys,_init:false,_active: active ,_registered_signals:HashSet::new(),destroyed:false}
    }
    pub fn activate(&mut self,game:&mut Game) -> RetTick{
        self._active = true;
        if !self._init {            
            self._init = true;
            self.fun._setup(&self._name, game)?;
        }
        
        Ok(true)
    }
    pub fn deactivate(&mut self) -> RetTick{
        self._active = false;
        Ok(true)
    }
    pub fn is_active(&self) -> bool{
        if self.destroyed{
            false
        }else {   
            self._active
        }
    }
    pub fn is_init(&self) -> bool{
        self._init
    }
    pub fn register_signal<S:Into<String>>(&mut self,sig:S) -> bool{
        let s = sig.into();

        self._registered_signals.insert(s)
    }
    pub fn destroy(&mut self,game:&mut Game) -> RetTick{
        // waiting for removal from vector
        self.destroyed = true;
        self.fun._kill_system(&self._name, game)
    }
    pub fn unregister_signal<S:Into<String>>(&mut self,sig:S) -> bool{
        let s = sig.into();

        self._registered_signals.remove(&s)
    }

}

impl<'a> Debug for System<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("System").field("_name", &self._name).field("fun", &"Sys").field("_helper", &"helper").field("_init", &self._init).field("_active", &self._active).field("destroyed", &self.destroyed).field("_registered_signals", &self._registered_signals).finish()
    }
}

/// call impl_sys!() for auto generate some boilerplate
pub trait GameSystem{
    fn _setup(&mut self,_sys_name:&String,   _game:&mut Game) -> RetTick{Ok(true)}
    fn _physics_loop(&mut self,_delta:Duration,_sys_name:&String,   _game:&mut Game) -> RetTick{Ok(true)}
    fn _process_loop(&mut self,_delta:Duration,_sys_name:&String,   _game:&mut Game) -> RetTick{Ok(true)}
    fn _kill_system(&mut self,_sys_name:&String,   _game:&mut Game) -> RetTick{Ok(true)}
    fn _on_signal_recieved(&mut self,_sys_name:&String,   _game:&mut Game,_signal:&mut Message,_from:&String) -> RetTick{Ok(true)}


    // auto traits with impl_sys!() macro
    // todo!
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;    
}   
