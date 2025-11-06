use std::fmt::Debug;

use crate::{Ret, RetType, components::{Attribute, Attributes, Components}, game::screen::Screen, math::Vec2};


pub struct ObjectBuilder{
    pub attributes:Attributes,
    components:Vec<Components>,
}
impl ObjectBuilder {
    pub fn new() -> Self{
        Self { attributes: Attributes::new(), components: Vec::new() }
    }
    pub fn with_attribute(mut self,att:Attribute) -> Self{
        match att{
            Attribute::Location(vec2)       => self.attributes.insert_Location(vec2),
            Attribute::Velocity(vec2)       => self.attributes.insert_Velocity(vec2),
            Attribute::Acc(vec2)        => self.attributes.insert_Acc(vec2),
            Attribute::Mass(a)       => self.attributes.insert_Mass(a),
            Attribute::Texture(a)       => self.attributes.insert_Texture(a),
            Attribute::Col(aabb)        => self.attributes.insert_Col(aabb),
        };
        self
        
    }
    pub fn with_component(mut self,comp:Components) -> Self {
        if !self.components.contains(&comp){
            self.components.push(comp);
        }
        self
    }
    pub fn build(self) -> ObjectHeader{
        ObjectHeader{
            attributes:self.attributes,
            changed:true,
            components:self.components,
            id:0,
            previus:Vec2::new(0, 0)
        }
    }

}

pub struct ObjectHeader{
    pub id:usize,
    pub(crate) previus:Vec2,
    changed:bool,
    pub attributes:Attributes,
    pub components:Vec<Components>,
}
impl ObjectHeader {
    pub fn get_cords(&self) -> Option<Vec2>{
        if let Some(a) = self.attributes.get_Location(){
            return Some(a.clone());
        }
        None
    }
    pub fn set_cords(&mut self,x:i32,y:i32) -> Ret{

        if let Some(v) = self.attributes.get_Location(){
            
            let new = (x,y).into();
            if new != *v {
                self.previus = *v;
                self.attributes.insert_Location(new);
                self.changed = true;
            }
        }

        Ok(())
    }
    pub fn print(&mut self,out:&mut Screen) -> RetType<bool>{

        if let Some(loc) = self.attributes.get_Location() && let Some(textur) = self.attributes.get_Texture(){
            textur.print(out, *loc)?;
            self.changed = false;
            Ok(true)
        }else {
            Ok(false)
        }
    }
    pub fn clearself(&self,out:&mut Screen) -> Ret{
        if let Some(loc) = self.attributes.get_Location() && let Some(textur) = self.attributes.get_Texture(){
            textur.clearself(out, *loc, self.previus)?;
        }
        Ok(())
    }
    pub fn should_render(&self) -> bool{
        if self.attributes.check_Location() && self.attributes.check_Texture(){
            return self.changed;
        }
        return false;

    }
}
impl Debug for ObjectHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObjectHeader").field("id", &self.id).field("previus", &self.previus).field("changed", &self.changed).field("attributes", &self.attributes).field("components", &self.components).finish()
    }
}



pub struct SquarePrint{
    pub tl:Vec2,
    pub br:Vec2,
}
