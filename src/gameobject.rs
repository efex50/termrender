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
            Attribute::Location(vec2)       => self.attributes.set_Location(vec2),
            Attribute::Velocity(vec2)       => self.attributes.set_Velocity(vec2),
            Attribute::Acc(vec2)        => self.attributes.set_Acc(vec2),
            Attribute::Mass(a)       => self.attributes.set_Mass(a),
            Attribute::Texture(a)       => self.attributes.set_Texture(a),
            Attribute::Col(aabb)        => self.attributes.set_Col(aabb),
            Attribute::Render(a) => self.attributes.set_Render(a),
            Attribute::Custom(custom_attr) => self.attributes.set_Custom(custom_attr),
            Attribute::Tag(s) => self.attributes.set_Tag(s),
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
            printed:false,
            id:0,
            previus:Vec2::new(0, 0),
            force_rerender:false
        }
    }

}

#[derive(Debug)]
pub enum ShouldRender{
    Unchanged,
    Changed,
    ForceRerender,
    Clear,
    Disabled,
}
pub struct ObjectHeader{
    pub id:usize,
    pub attributes:Attributes,
    pub components:Vec<Components>,
    pub(crate) previus:Vec2,
    pub(crate) printed:bool,
    changed:bool,
    force_rerender:bool,
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
                if !self.changed{
                    self.previus = *v;
                }
                self.attributes.set_Location(new);
                self.changed = true;
            }
        }

        Ok(())
    }
    pub fn print(&mut self,out:&mut Screen) -> RetType<bool>{

        if let Some(loc) = self.attributes.get_Location() && let Some(textur) = self.attributes.get_Texture(){
            textur.print(out, *loc)?;
            self.changed = false;
            self.printed = true;
            Ok(true)
        }else {
            Ok(false)
        }
    }
    pub fn clearself(&mut self,out:&mut Screen) -> Ret{
        if let Some(loc) = self.attributes.get_Location() && let Some(textur) = self.attributes.get_Texture(){
            if self.printed {
                textur.clearself(out, *loc, self.previus)?;
                self.printed = false;
            }
            self.printed = false;
        }
        Ok(())
    }
    pub fn is_printed(&self) -> bool{
        self.printed
    }
    /// force rerender the object
    pub fn force_rerender(&mut self){
        self.force_rerender = true;
    }

    pub fn _should_render(&mut self) -> ShouldRender {
        if self.attributes.check_Location() && self.attributes.check_Texture() {
            if self.force_rerender {
                self.force_rerender = false; // mutlaka sıfırla
                return ShouldRender::ForceRerender;
            }

            if self.attributes.check_Render() {
                if let Some(render) = self.attributes.get_Render() {
                    if !*render {
                        return if self.printed {
                            ShouldRender::Clear
                        } else {
                            ShouldRender::Disabled
                        };
                    }
                }
            }

            return if self.changed {
                ShouldRender::Changed
            } else {
                ShouldRender::Unchanged
            };
        }

        ShouldRender::Disabled
    }
    pub fn should_render(&mut self) -> ShouldRender{
        
        if self.attributes.check_Location() && self.attributes.check_Texture(){
            if self.force_rerender{
                self.force_rerender = false;
                return ShouldRender::ForceRerender;
            }
            if self.attributes.check_Render(){
                if !*self.attributes.get_Render().unwrap(){
                    if self.printed{
                        return ShouldRender::Clear;
                    }
                    return ShouldRender::Disabled;
                }
            }
            return match self.changed{
                true => ShouldRender::Changed,
                false => ShouldRender::Unchanged,
            };
        }
        return ShouldRender::Disabled;
        


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
