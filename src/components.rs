#[allow(non_snake_case)]

#[allow(dead_code)]
use std::{collections::HashSet, hash::{Hash, Hasher}};
use paste::paste;

use crate::{math::Vec2, physics::AABB, print::GameTexture};

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
                std::mem::discriminant(self) == std::mem::discriminant(other)
            }
        }
        impl Eq for AttributeAny {}
        impl Hash for AttributeAny {
            fn hash<H: Hasher>(&self, state: &mut H) {
                std::mem::discriminant(self).hash(state)
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




attributes!(
    Location:Vec2,
    Velocity:Vec2,
    Acc:Vec2,
    Mass:i32,
    Texture:GameTexture,
    Col:AABB
);
#[test]
fn att_test(){
    let mut m = Attributes::new();
    m.set_Mass(100);
    m.set_Mass(30);
    let _mass = m.get_Mass().unwrap();
    println!("{:?}",m);
}