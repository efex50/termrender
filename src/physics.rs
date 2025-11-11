use std::ops::{Add, AddAssign};

use crate::math::Vec2;

/// karesel alan
/// 
/// cor1 = sol üst köşe
/// 
/// cor2 = sağ alt köşe
#[derive(Debug,Hash,Default,Clone, Copy)]
pub struct AABB{
    pub cor1:Vec2,
    pub cor2:Vec2,
}
impl AABB {
    /// sırayla
    /// 
    /// solüst, sağüst, sağalt, solalt
    pub fn get_corners(&self) -> (Vec2,Vec2,Vec2,Vec2){
        (
            self.cor1,
            Vec2::from((self.cor2.x,self.cor1.y)),
            self.cor2,
            Vec2::from((self.cor1.x,self.cor2.y)))
    }
    pub fn new(start:Vec2,end:Vec2) -> Self{
        let start:Vec2 = {
            let x: i32 = {
                if start.x > end.x{
                    end.x
                }
                else {
                    start.x
                }
            };
            let y:i32 = {
                if start.y > end.y{
                    end.y
                }else {
                    end.y
                }
            };
            Vec2 { x, y }
        };
        let end:Vec2 = {
            let x: i32 = {
                if start.x < end.x{
                    end.x
                }
                else {
                    start.x
                }
            };
            let y:i32 = {
                if start.y < end.y{
                    end.y
                }else {
                    end.y
                }
            };
            Vec2 { x, y }
        };
        
        AABB { cor1: start, cor2: end }
    }
}
impl From<(Vec2,Vec2)> for AABB {
    fn from(value: (Vec2,Vec2)) -> Self {
        AABB { cor1: value.0, cor2: value.1 }
    }
}
impl From<((i32,i32),(i32,i32))> for AABB {
    fn from(value: ((i32,i32),(i32,i32))) -> Self {
        AABB { cor1: Vec2::from(value.0), cor2: Vec2::from(value.1) }
    }
}
impl From<i32> for AABB {
    fn from(value: i32) -> Self {
        AABB { cor1: Vec2::from(0), cor2: Vec2::from(value) }
    }
}
impl Add for AABB {
    type Output = AABB;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
            cor1:self.cor1 + rhs.cor1,
            cor2:self.cor2 + rhs.cor2
        }
    }
}
impl Add<Vec2> for AABB {
    type Output = AABB;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self{
            cor1:self.cor1+rhs,
            cor2:self.cor2+rhs
        }
    }
}
impl AddAssign<Vec2> for AABB {
    fn add_assign(&mut self, rhs: Vec2) {
        self.cor1 += rhs;
        self.cor2 += rhs;
    }
}


impl CollisionCheck for AABB {
    fn is_colliding(&self,other:&Self) -> bool {
        if self.cor1.x > other.cor2.x{
            false
        }
        else if self.cor1.y > other.cor2.y{
            false
        }
        else if self.cor2.x < other.cor1.x {
            false
        }
        else if self.cor2.y < other.cor1.y {
            false
        }
        else  {
            true
        }
    }
}

pub trait CollisionCheck<Rhs = Self>{
    fn is_colliding(&self,other:&Rhs) -> bool;
}
pub trait ColCheck {
    fn is_colliding(&self,other:i32)   -> bool;
}