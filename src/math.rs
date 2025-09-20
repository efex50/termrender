use std::ops;





#[derive(Debug, Copy, Clone, PartialEq,Hash,Eq)]
pub struct Vec2{
    pub x:i32,
    pub y:i32,
}
impl Vec2 {
    pub fn new(x:i32,y:i32) -> Self{
        Self { x, y }
    }
}

impl From<Vec2> for (i32,i32) {
    fn from(value: Vec2) -> Self {
        (value.x,value.y)
    }
}
impl From<(u16,u16)> for Vec2{
    fn from(value: (u16,u16)) -> Self {
        Self { x: value.0 as i32, y:value.1 as i32 }
    }
}
impl From<(i32,i32)> for Vec2{
    fn from(value: (i32,i32)) -> Self {
        Self { x: value.0, y:value.1 }
    }
}
impl ops::Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self{
            x:self.x + rhs.x,
            y:self.y + rhs.y
        }
    }
}
impl ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            x:self.x - rhs.x,
            y:self.y - rhs.y
        }
    }
}
impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}