
use termrender::prelude::*;
use crate::ticks::{PlayerSystem, Redrawer};


fn main() -> std::io::Result<()> {



    LOG!(Warn,"warn");
    LOG!(Err,"err");
    LOG!(Info,"info");
    LOG!(Debug,"debug");
    LOG!(RandomAss, "zorrt");
    
    
    let mut _main = Game::new("zort".to_string(),30.1,100.);
    _main.systems.new_system(Box::new(Redrawer::new()), "redrawer", true);

    let psys = PlayerSystem::new();
    _main.systems.new_system(Box::new(psys),"player".to_string(),true);

    _main.setup()?;
    _main.main_loop()?;
    
    _main.exit()?;
    
    return Ok(());
}


mod ticks{

    use std::time::{Duration, Instant};

    use termrender::prelude::*;

    pub struct PlayerSystem{
        arena:(u16,u16),
        player_id:usize,
    }
    impl PlayerSystem {
        pub fn new() -> Self{
            Self {
                arena:(100,40),
                player_id: 0,
            }
        }
    }
    impl GameSystem for PlayerSystem {
        fn _setup(&mut self,_name:&String,g:&mut Game) -> RetTick {
            let new_line = ObjectBuilder::new()
                .with_attribute(Attribute::Location((0,0).into()))
                .with_attribute(Attribute::Texture(
                    make_texture!(
                        // line vert
                        ((0,0) => (0,self.arena.1) "|"),
                        // line horzt
                        ((0,0) => (self.arena.0,0) "-"),
                        ((self.arena.0,0) => (self.arena.0,self.arena.1) "|"),
                        ((0,self.arena.1) => (self.arena.0,self.arena.1) "-"),
                    )
                ))
                .with_component(Components::Wall)
                .build();
            let id = g.insert_object_head(new_line)?;

            LOG!(Debug,"Çerçeveler oluşturuldu id:{}",id);

            let t =  make_texture!(
                ((0,0)("██",(),GColor::Red)),
                ((-2,-1)"zarsst"),
                ((0,1)"z"),
            );
            let player = ObjectBuilder::new()
                .with_attribute(Attribute::Location(Vec2::from((10,10))))
                .with_attribute(Attribute::Texture(t))
                .with_component(Components::Player)
                .build()
            ;

            let id = g.insert_object_head(player)?;

            self.player_id = id;
            LOG!(Debug,"Oyuncu oluşturuldu id:{}",id);

            Ok(true)
        }


    
        fn _physics_loop(&mut self,_name:&String,g:&mut Game) -> RetTick {
            
            
            //g.log(format!("sa"));


            let player = g.world.get_with_id(self.player_id).unwrap();
            let mut pos = player.get_cords().unwrap();
            for key in &g.input.pressed_keys { 
                match key {
                    Keys::Up => pos.y -=1,
                    Keys::Down => pos.y +=1,
                    Keys::Left => pos.x -=1,
                    Keys::Right => pos.x +=1,
                    _ => (),
                }
            }
            player.set_cords(pos.x, pos.y)?;
            Ok(true)

        }
    }

    pub struct Redrawer{
        timeout:Duration,
        now:Instant,
    }
    impl Redrawer {
        pub fn new() -> Self{
            Self{
                timeout:Duration::from_millis(130),
                now:Instant::now()
            }
        }
    }
    impl GameSystem for Redrawer {
        fn _setup(&mut self,_sys_name:&String,_:&mut Game) -> RetTick {
            self.now = Instant::now(); 
            Ok(true)
        }
        fn _process_loop(&mut self,_sys_name:&String,game:&mut Game)  -> RetTick {
            if self.now.elapsed() >= self.timeout{
                self.now = Instant::now();
                LOG!(Debug,"rerendered");
                game.force_rerender()?;
            }
            Ok(true)
        }
    }

}