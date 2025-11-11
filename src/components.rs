use std::{any::Any, fmt::Debug};
#[allow(non_snake_case)]

#[allow(dead_code)]
use std::{collections::HashSet, hash::{Hash, Hasher}};
use paste::paste;

use crate::{math::Vec2, physics::AABB, print::GameTexture};
#[macro_export]
macro_rules! comp {
    ($a:ident) => {
        {
            let s = stringify!($a);

            $crate::components::Components::from_str(s)
        }
    };
}


#[derive(Debug,PartialEq, Eq)]
#[repr(u8)]
pub enum Components{
    Player,
    BulletPlayer,
    Enemy,
    BulletEnemy,
    Wall,
    AreaSquare,
    /// for dropped objects that needs reallocation
    Destroyed,
    Custom(String),
}
impl Components {
    pub fn custom<S:Into<String>>(c:S) -> Self {
        let s = c.into();
        Self::Custom(s)

    }
    pub fn from_str<S:Into<String>>(c:S) -> Components {
        let s = c.into();
        match &*s {
            "AreaSquare" => Self::AreaSquare,
            "BulletEnemy" => Self::BulletEnemy,
            "BulletPlayer" => Self::BulletPlayer,
            "Destroyed" => Self::Destroyed,
            "Enemy" => Self::Enemy,
            "Player" => Self::Player,
            "Wall" => Self::Wall,
            _=> Self::Custom(s)
        }
    }
}
pub fn custom_comp<S:Into<String>>(c:S) -> Components{
    let s = c.into();
    Components::Custom(s)
}


macro_rules! attributes {
    ( $( $name:ident : $ty:ty ),* $(,)? ) => {
        $(
            #[derive(Debug)]
            pub struct $name(pub $ty);
        )*

        #[derive(Debug)]
        pub struct Attributes {
            set: HashSet<AttributeAny>,
        }

        #[derive(Debug)]
        enum AttributeAny {
            $(
                $name($name),
            )*
        }
        #[derive(Debug)]
        pub enum Attribute{
            $(
                $name($ty),
            )*
        }

        // equality & hash only depend on variant
        impl PartialEq for AttributeAny {
            fn eq(&self, other: &Self) -> bool {
                match (self,other){
                    (Self::Custom(a), Self::Custom(b)) => a == b,
                    _ => std::mem::discriminant(self) == std::mem::discriminant(other),

                }
            }
        }
        impl Eq for AttributeAny {}
        impl Hash for AttributeAny {
            fn hash<H: Hasher>(&self, state: &mut H) {
                match self {
                    Self::Custom(a) => {
                        std::mem::discriminant(self).hash(state);
                        a.0.name.hash(state);
                    }
                    _ => std::mem::discriminant(self).hash(state),
                }
            
                //std::mem::discriminant(self).hash(state)
            }
        }

        impl Attributes {
            pub fn new() -> Self {
                Self { set: HashSet::new() }
            }

            paste! {
                $(
                    #[allow(non_snake_case)]
                    pub fn [<set_ $name>](&mut self, val: $ty) {
                        self.set.replace(AttributeAny::$name($name(val)));
                    }
                    #[allow(non_snake_case)]
                    pub fn [<get_ $name>](&self) -> Option<&$ty> {
                        if let Some(AttributeAny::$name(inner)) =
                            self.set.get(&AttributeAny::$name($name(Default::default())))
                        {
                            Some(&inner.0)
                        } else {
                            None
                        }
                    }
                    #[allow(non_snake_case)]
                    pub fn [<check_ $name>](&self) -> bool {
                        if let Some(AttributeAny::$name(_)) =
                            self.set.get(&AttributeAny::$name($name(Default::default())))
                        {
                            true
                        } else {
                            false
                        }
                    }
                )*
            }
        }
    };
}


pub struct CustomAttr{
    pub name:String,
    pub cus:Box<dyn Any>
}
impl Default for CustomAttr {
    fn default() -> Self {
        Self { name: Default::default(), cus: Box::new(()) }
    }
}
impl Debug for CustomAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(_a) = self.cus.downcast_ref::<Box<dyn Debug>>(){
            
        }
        
        f.debug_struct("CustomAttr").field("name", &self.name).field("cus", &"Dynamic Field").finish()
    }
}
impl CustomAttr {
    pub fn new<S:Into<String>>(name: S, cus: Box<dyn Any>) -> Self {
        Self { name:name.into(), cus }
    }
}
impl PartialEq for Custom {
    fn eq(&self, other: &Self) -> bool {
        self.0.name == other.0.name
    }
}

attributes!(
    Location:Vec2,
    Velocity:Vec2,
    Acc:Vec2,
    Mass:i32,
    Texture:GameTexture,
    Col:AABB,
    Render:bool,
    Custom:CustomAttr,
    Tag:String
);
#[test]
fn att_test(){
    let mut m = Attributes::new();
    m.set_Mass(100);
    m.set_Mass(30);
    let _mass = m.get_Mass().unwrap();
    println!("{:?}",m);
}
#[test]
fn custom_test(){
    let mut a = Attributes::new();
    a.set_Custom(CustomAttr::new("lol", Box::new(1)));
    a.set_Custom(CustomAttr::new("lul", Box::new(())));
    println!("{:?}",a)
}

#[test]
fn any_debug(){
    let a: Box<dyn Any> = Box::new(2);
    if let Ok(b) = a.downcast::<Box<dyn Debug>>(){
        println!("{:?}",b)
    }else {
        println!("olmadı abe")
    }
}


#[test]
fn tag_test(){
    let mut a = Attributes::new();
    a.set_Tag("asfasfsa".to_string());
    a.set_Tag("31".to_string());
    a.set_Tag("67".to_string());
    println!("{:?}",a)
}