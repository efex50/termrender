pub use crate::{
    print::*,
    make_texture,
    gameobject::{SquarePrint,ObjectHeader,ObjectBuilder},
    math::*,
    game::{
        Game,
        systems::GameSystem,
        input::*,
        logger::{
            log,
            LogLevel,
            LogType
        }
    },
    Ret,
    RetTick,
    components::{Attribute,Attributes,Components},
    LOG,
};
pub mod log{
    pub use crate::{
        game::logger::{GLOBAL_LOGGER,LogLevel,Logger,get_logger,log},
    };
}