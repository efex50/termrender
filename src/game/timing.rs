use std::time::{Duration, Instant};


pub struct Timing {
    pub(crate) _last_physics_frame: Instant,
    pub(crate) _last_process_frame: Instant,
    pub(crate) _last_render_frame: Instant,
    pub(crate) render_fps: f32,
    pub(crate) physics_fps: f32,
    pub(crate) process_fps:f32,
}
impl Timing {
    pub fn new() -> Self{
    let now = Instant::now();
    Self{
        _last_physics_frame:now,
        _last_process_frame:now,
        physics_fps:30.,
        process_fps:30.,
        _last_render_frame: now,
        render_fps: 30.,
    }
}
pub fn get_delta_physics(&self) -> Duration{
    self._last_physics_frame.elapsed()
}
pub fn get_delta_process(&self) -> Duration{
    self._last_process_frame.elapsed()
}
pub fn get_delta_render(&self) -> Duration{
    self._last_render_frame.elapsed()
}
pub fn should_render(&self) -> bool{
    let frame_time = Duration::from_secs_f32(1.0 / self.render_fps);
    self._last_render_frame.elapsed() >= frame_time 
}
pub fn should_process(&self) -> bool{
    let frame_time = Duration::from_secs_f32(1.0 / self.process_fps);
    self._last_process_frame.elapsed() >= frame_time 
}
pub fn should_physics(&self) -> bool{
    let frame_time = Duration::from_secs_f32(1.0 / self.physics_fps);
    self._last_physics_frame.elapsed() >= frame_time 
}
pub fn update_physics_delta(&mut self){
    self._last_physics_frame = Instant::now();
}
pub fn update_process_delta(&mut self){
    self._last_process_frame = Instant::now();
}
pub fn update_render_delta(&mut self){
    self._last_render_frame = Instant::now();
}
pub fn set_render_fps(&mut self,new:f32){
    self.render_fps = new;
}
pub fn set_process_fps(&mut self,new:f32){
    self.process_fps = new;
}
pub fn set_physics_fps(&mut self,new:f32){
    self.physics_fps = new;
}
}
