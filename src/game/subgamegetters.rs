use crate::{game::{Game, Timing, World, screen::Screen, systems::Systems}, prelude::Input};


macro_rules! ptr_clone {
    ($self:expr) => {
        unsafe {
            ($self as *mut Game).as_mut().unwrap()
        }
    };
    ($self:expr, mut) => {
        unsafe {
            ((($self as *const Game) as usize) as *mut Game).as_mut().unwrap()
        }
    };
    ($self:expr, ptr) => {
        unsafe {
            ($self as *const Game).as_ref().unwrap()
        }
    };
}

/// unsafe getters
impl<'a> Game<'a> {

    pub fn get_input(&self) -> &Input{
        let g= ptr_clone!(self,ptr);
        &g.input
    }
    pub fn get_input_mut(&self) -> &mut Input{
        let g= ptr_clone!(self,mut);
        &mut g.input
    }
    pub fn get_screen(&self) -> &Screen{
        let g= ptr_clone!(self,ptr);
        &g.screen
    }
    pub fn get_screen_mut(&self) -> &mut Screen{
        let g= ptr_clone!(self,mut);
        &mut g.screen
    }
    pub fn get_timing(&self) -> &Timing{
        let g= ptr_clone!(self,ptr);
        &g.timing
    }
    pub fn get_timing_mut(&self) -> &mut Timing{
        let g= ptr_clone!(self,mut);
        &mut g.timing
    }
    pub fn get_world(&self) -> &World{
        let g= ptr_clone!(self,ptr);
        &g.world
    }
    pub fn get_wolrd_mut(&self) -> &mut World{
        let g= ptr_clone!(self,mut);
        &mut g.world
    }
    pub fn get_systems(&self) -> &Systems<'_>{
        let g= ptr_clone!(self,ptr);
        &g.systems
    }
    pub fn get_systems_mut(&self) -> &mut Systems<'a>{
        let g= ptr_clone!(self,mut);
        &mut g.systems
    }
    pub fn get_self_mut(&self) -> &mut Game<'_>{
        ptr_clone!(self,mut)
    }

    
}