use std::ops;


#[derive(Debug, Copy, Clone, PartialEq,Default)]
pub struct Vec2f{
    pub x:f32,
    pub y:f32,
}
impl From<Vec2f> for (f32,f32) {
    fn from(value: Vec2f) -> Self {
        (value.x,value.y)
    }
}
impl From<Vec2f> for (u16,u16) {
    fn from(value: Vec2f) -> Self {
        (value.x as u16,value.y as u16)
    }
}
impl From<(u16,u16)> for Vec2f{
    fn from(value: (u16,u16)) -> Self {
        Self { x: value.0 as f32, y:value.1 as f32 }
    }
}
impl From<(f32,f32)> for Vec2f{
    fn from(value: (f32,f32)) -> Self {
        Self { x: value.0, y:value.1 }
    }
}
impl ops::Add for Vec2f {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self{
            x:self.x + rhs.x,
            y:self.y + rhs.y
        }
    }
}
impl ops::Sub for Vec2f {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            x:self.x - rhs.x,
            y:self.y - rhs.y
        }
    }
}
impl ops::AddAssign for Vec2f {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl ops::SubAssign for Vec2f {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}



#[derive(Debug, Copy, Clone, PartialEq,Hash,Eq,Default,PartialOrd)]
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
impl From<Vec2> for (u16,u16) {
    fn from(value: Vec2) -> Self {
        (value.x as u16,value.y as u16)
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
impl From<u16> for Vec2{
    fn from(value: u16) -> Self {
        Self { x: value as i32, y:value as i32 }
    }
}
impl From<i32> for Vec2{
    fn from(value: i32) -> Self {
        Self { x: value, y:value }
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






#[cfg(test)]
mod test{
    use crate::math::Vec2;

    #[test]
    fn order(){
        let v1 = Vec2::from((1,0));
        let v2 = Vec2::from((1,3));
        println!("{}",v1 < v2)
    }
}