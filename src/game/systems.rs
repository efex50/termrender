use std::{any::Any, borrow::Cow, collections::HashMap, time::Duration};

use crate::{LOG, Ret, RetTick, game::Game};

#[macro_export]
macro_rules! impl_sys {
    () => {
//        fn as_any(&self) -> &dyn std::any::Any{
//            self
//        }
//        fn as_any_mut(&mut self) -> &mut dyn std::any::Any{
//            self
//        }

    };
}
pub struct Message{
    pub hint:String,
    pub to:String,
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
    pub fn new_system<St:Into<String>>(&mut self,s:Box<dyn GameSystem>,name:St,active:bool ){
        let name = name.into();
        let sys = System::new(name.clone(), s,active);
        self.sys.insert(name,sys);
    }
    pub(crate) fn send_signal(&mut self,hint:&'a str,to:&'a str,msg:Box<dyn Any + Send>){
        let m: Message = Message{
            hint:hint.to_string(),
            to:to.to_string(),
            msg:msg
        };
        self.pending_signals.push(m);
    }
    // sinyalleri tarar ve hedeflere gönderir
    pub(crate)  fn deliver_signals(&mut self,g:&mut Game) -> Ret{
        for msg in self.pending_signals.iter_mut(){
            if let Some(target) = self.sys.get_mut(msg.to.as_str()){
                target.fun._on_signal_recieved(&target._name, g, msg, &msg.hint.clone())?;
            }else {
                LOG!(Err,"sended signal from {}, target not found: {}",msg.hint,msg.to);
            }
        }
        self.pending_signals.clear();
        Ok(())
    }
}


type Closure<'a> = Box<dyn Fn(&mut dyn GameSystem) + 'a>;
pub struct SystemBuilder<'a>{
    _name:Cow<'a,String>,
    _config:Closure<'a>,
    fun:Box<dyn GameSystem>,

}
impl<'a> SystemBuilder<'a> {
    pub fn new(sys:Box<dyn GameSystem>) -> Self{
        Self { 
            _name: Cow::Owned("unnamed".to_string()),
            _config:Box::new(|_|{}),
            fun:sys
        }
    }
    pub fn with_name<S:Into<String>>(mut self,name:S) -> Self{
        let name = name.into();
        self._name = Cow::Owned(name);
        self
    }
    pub fn with_config(mut self,config:Closure<'a>) -> Self{
        self._config = config;
        //self;
        todo!("not yet implemented the config system and there is no need for that");
    }
    pub fn build(self) -> System<'a>{
        System { _name: self._name, fun: self.fun, _config: self._config, _init: false, _active: false }
    }
}

pub struct System<'a>{
    pub(crate) _name:Cow<'a,String>,
    pub fun:Box<dyn GameSystem>,
    _config:Closure<'a>,
    _init:bool,
    _active:bool,
}
impl<'a> System<'a> {
    pub fn new(name: String, sys: Box<dyn GameSystem>,active:bool) -> Self {
        let n= Cow::<'a,String>::Owned(name);
        Self { _name:n, fun: sys,_init:false,_active: active ,_config:Box::new(|_|{})}
    }
    pub fn activate(&mut self,game:&mut Game) -> RetTick{
        self._active = true;
        if !self._init {
            (self._config)(&mut *self.fun);
            self.fun._setup(&self._name, game)?;
        }
        self._init = true;
        
        Ok(true)
    }
    pub fn deactivate(&mut self) -> RetTick{
        self._active = false;
        Ok(true)
    }
    pub fn is_active(&self) -> bool{
        self._active
    }
    pub fn is_init(&self) -> bool{
        self._init
    }
    
}
/// call impl_sys!() for auto generate some boilerplate
pub trait GameSystem{
    fn _setup(&mut self,_sys_name:&String,   _game:&mut Game) -> RetTick{Ok(true)}
    fn _physics_loop(&mut self,_delta:Duration,_sys_name:&String,   _game:&mut Game) -> RetTick{Ok(true)}
    fn _process_loop(&mut self,_delta:Duration,_sys_name:&String,   _game:&mut Game) -> RetTick{Ok(true)}
    fn _kill_system(&mut self,_sys_name:&String,   _game:&mut Game) -> RetTick{Ok(true)}
    fn _on_signal_recieved(&mut self,_sys_name:&String,   _game:&mut Game,_signal:&mut Message,_from:&String) -> RetTick{Ok(true)}
    //fn as_any(&self) -> &dyn Any;
    //fn as_any_mut(&mut self) -> &mut dyn Any;
}

/*
impl dyn GameSystem {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
    pub fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut::<T>()
    }

}



        */


