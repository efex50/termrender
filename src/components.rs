#[allow(non_snake_case)]
#[allow(dead_code)]
use std::{collections::HashSet, hash::{Hash, Hasher}};
use paste::paste;

use crate::{math::Vec2, print::GameTexture};

#[derive(Debug,PartialEq, Eq)]
pub enum Components{
    Player,
    BulletPlayer,
    Enemy,
    BulletEnemy,
    Wall,
    Object(String),
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
                    pub fn [<insert_ $name>](&mut self, val: $ty) {
                        self.set.replace(AttributeAny::$name($name(val)));
                    }

                    pub fn [<get_ $name>](&self) -> Option<&$ty> {
                        if let Some(AttributeAny::$name(inner)) =
                            self.set.get(&AttributeAny::$name($name(Default::default())))
                        {
                            Some(&inner.0)
                        } else {
                            None
                        }
                    }
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
    Texture:GameTexture
);
#[test]
fn att_test(){
    let mut m = Attributes::new();
    m.insert_Mass(100);
    m.insert_Mass(30);
    let mass = m.get_Mass().unwrap();
    println!("{:?}",m);
}