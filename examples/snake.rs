
use termrender::prelude::*;
use crate::ticks::{PlayerSystem, SnakeHead};


fn main() -> std::io::Result<()> {



    LOG!(Warn,      "Test TEST");
    LOG!(Err,       "Test TEST");
    LOG!(Info,      "Test TEST");
    LOG!(Debug,     "Test TEST");
    LOG!(RandomAss, "Test TEST");
    LOG!(FileName,  file!());
    
    let mut _main = Game::new("zort".to_string(),30.1,100.);
    let logpath = format!("{}",file!());
    let logpath = logpath.trim_end_matches(".rs");
    _main.set_log_path(Some(logpath.to_string()));
    //_main.systems.new_system(Box::new(Redrawer::new()), "redrawer", true);

    let psys = PlayerSystem::new((50,25));
    _main.systems.new_system(Box::new(psys),"player".to_string(),true);
    let snek = SnakeHead::new((50,25));
    _main.systems.new_system(Box::new(snek), "yılan", true);
    _main.setup()?;
    _main.main_loop()?;
    _main.exit()?;
    
    return Ok(());
}


mod ticks{


    use std::time::{Duration, Instant};

    use termrender::{impl_sys, prelude::*,prelude::math::*};


    struct BorderSystem{
        area:AABB
    }
    impl GameSystem for BorderSystem {
        fn _setup(&mut self,_sys_name:&String,   _game:&mut Game) -> RetTick {
            
            let corners = self.area.get_corners();
            let borders = ObjectBuilder::new()
                .with_attribute(Attribute::Location((0,0).into()))
                .with_attribute(Attribute::Texture(
                    make_texture!(
                        // üst
                        ((corners.0.x-1,corners.0.y-1) => (corners.1.x+2,corners.1.y-1) ("\u{2584}")),
                        // sol
                        ((corners.0.x-1,corners.0.y) => (corners.3.x-1,corners.3.y) ("\u{2588}")),
                        // sağ
                        ((corners.1.x+1,corners.1.y) => (corners.2.x+1,corners.2.y) ("\u{2588}")),
                        //alt 
                        ((corners.3.x-1,corners.3.y+1) => (corners.2.x+2,corners.2.y+1) ("\u{2580}")),
                        // köşeler
                        //((corners.0.x-1,corners.0.y-1) ("\u{250F}")),
                        //((corners.1.x+1,corners.1.y-1) ("\u{2513}")),
                        //((corners.2.x+1,corners.2.y+1) ("\u{251B}")),
                        //((corners.3.x-1,corners.3.y+1) ("\u{2517}")),
                        //[self.arena_col,("x",GColor::Black,GColor::Red)],
                    )
                ))
                .with_component(Components::Wall)
                .build();

            let world = _game.get_wolrd_mut();
            let id = world.insert_object_head(borders)?;
            LOG!(Debug,"Çerçeveler oluşturuldu id:{}",id);

            Ok(true)
        }
    }

    pub struct PlayerSystem{
        player_id:usize,
        arena_draw:AABB,
        arena_col:AABB,
        player_head:AABB,
    }
    impl PlayerSystem {
        pub fn new(arena:(i32,i32)) -> Self{
            Self {
                player_head:AABB { cor1: Vec2 { x: 0, y: 0 }, cor2: Vec2 { x: 0, y: 0 } },
                arena_col:AABB::from((Vec2::from((1,1)),Vec2::from(((arena.0)*2,arena.1)))),
                arena_draw:AABB::from((Vec2::from((1,1)),Vec2::from(((arena.0)*2,arena.1)))),
                player_id: 0,
            }
        }
    }
    impl GameSystem for PlayerSystem {
        impl_sys!();
        fn _setup(&mut self,_name:&String,g:&mut Game) -> RetTick {

            let bor = BorderSystem{
                area:self.arena_draw
            };
            g.systems.new_system(Box::new(bor), "border_sys", true);

            g.send_signal(_name, &"yılan".to_string(), Box::new("naber yarrrrak"));

            let w = g.get_wolrd_mut();
            let t =  make_texture!(
                ((0,0)("██",(),GColor::Green)),
                //((-2,-1)"zarsst"),
                //((0,1)"z"),
            );
            let player = ObjectBuilder::new()
                .with_attribute(Attribute::Location(Vec2::from((3,3))))
                .with_attribute(Attribute::Texture(t))
                .with_attribute(Attribute::Col(self.player_head))
                .with_component(Components::Player)
                .build()
            ;
        

            LOG!(Debug,"Oyuncu componentleri:{:?}",player.components);
            let id = w.insert_object_head(player)?;


            let p = w.get_with_component(Components::Player);
            LOG!(Debuh,"{:?}",p);
            self.player_id = id;
            LOG!(Debug,"Oyuncu oluşturuldu id:{}",id);

            Ok(true)
        }


    
        fn _physics_loop(&mut self,delta:Duration,_name:&String,g:&mut Game) -> RetTick {
            
            let player = g.world.get_with_id(self.player_id).unwrap();
            let pos = player.get_cords().unwrap();
                
            // Şu anki pozisyon
            let mut new_pos = pos;
                
            // Hareket farkı
            let mut delta = Vec2 { x: 0, y: 0 };
            for key in &g.input.pressed_keys {
                match key {
                    Keys::Up => delta.y -= 1,
                    Keys::Down => delta.y += 1,
                    Keys::Left => delta.x -= 2,
                    Keys::Right => delta.x += 2,
                    _ => (),
                }
            }
        
            // --- X ekseninde dene ---
            let mut col_x = self.player_head;
            let try_pos_x = Vec2 { x: pos.x + delta.x, y: pos.y };
            col_x = col_x + try_pos_x;
        
            // Eğer yeni konum hala arena içindeyse → hareket et
            if col_x.is_colliding(&self.arena_col) {
                new_pos.x += delta.x;
            }
        
            // --- Y ekseninde dene ---
            let mut col_y = self.player_head;
            let try_pos_y = Vec2 { x: new_pos.x, y: pos.y + delta.y };
            col_y = col_y + try_pos_y;
        
            // Eğer yeni konum hala arena içindeyse → hareket et
            if col_y.is_colliding(&self.arena_col) {
                new_pos.y += delta.y;
            }
        
            player.set_cords(new_pos.x, new_pos.y)?;
            Ok(true)

        }
    }

    // --------------------- yılan kısmı
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    impl Direction {
        fn delta(&self) -> Vec2 {
            match self {
                Direction::Up => Vec2 { x: 0, y: -1 },
                Direction::Down => Vec2 { x: 0, y: 1 },
                Direction::Left => Vec2 { x: -2, y: 0 },
                Direction::Right => Vec2 { x: 2 , y: 0 },
            }
        }

        fn is_opposite(&self, other: &Direction) -> bool {
            matches!(
                (self, other),
                (Direction::Up, Direction::Down)
                    | (Direction::Down, Direction::Up)
                    | (Direction::Left, Direction::Right)
                    | (Direction::Right, Direction::Left)
            )
        }
    }
    pub struct SnakeHead {
        head_id: usize,
        dir: Direction,
        head_aabb: AABB,    // relative AABB for the head (origin at 0,0)
        arena_col: AABB,    // allowed area (player must stay inside)
    }
    impl SnakeHead {
        pub fn new(arena_size: (i32, i32)) -> Self {
            Self {
                head_id: 0,
                dir: Direction::Right,
                // head AABB 1x1 starting at 0,0 (ayarla istersen)
                head_aabb: AABB { cor1: Vec2 { x: 0, y: 0 }, cor2: Vec2 { x: 0, y: 0 } },
                // arena: örn (1,1) .. (arena.0*2, arena.1)
                arena_col: AABB::from((Vec2::from((1,1)), Vec2::from((arena_size.0*2, arena_size.1)))),
            }
        }
    }
    impl GameSystem for SnakeHead {
        impl_sys!();

        fn _setup(&mut self, _name: &String, g: &mut Game) -> RetTick {
            let w = g.get_wolrd_mut();

            // Head texture: tek kare kırmızı blok (veya istediğin)
            let t = make_texture!(((0,0)("██",(),GColor::Red)));
            let player = ObjectBuilder::new()
                .with_attribute(Attribute::Location(Vec2::from((3,3))))
                .with_attribute(Attribute::Texture(t))
                .with_attribute(Attribute::Col(self.head_aabb))
                .with_component(Components::Player)
                .build();
            let id = w.insert_object_head(player)?;
            self.head_id = id;
            Ok(true)
        }

        fn _physics_loop(&mut self, delta:Duration,_name: &String, g: &mut Game) -> RetTick {
            // 1) input -> yön değişikliği (zıt yön engelle)
            for key in &g.input.pressed_keys {
                let new_dir = match key {
                    Keys::Up => Some(Direction::Up),
                    Keys::Down => Some(Direction::Down),
                    Keys::Left => Some(Direction::Left),
                    Keys::Right => Some(Direction::Right),
                    _ => None,
                };
                if let Some(nd) = new_dir {
                    // zıt yön değilse uygula
                    if !nd.is_opposite(&self.dir) {
                        self.dir = nd;
                    }
                }
            }

            // 2) mevcut baş objesini al ve pozisyonu bul
            let head_obj = g.world.get_with_id(self.head_id).unwrap();
            let pos = head_obj.get_cords().unwrap(); // Vec2

            // 3) denenen yeni pozisyon = pos + dir.delta()
            let delta = self.dir.delta();
            let try_pos = Vec2 { x: pos.x + delta.x, y: pos.y + delta.y };

            // 4) head AABB'yi bu yeni pozisyona taşı ve arena ile "çarpışıyor mu?" kontrol et
            let moved_head = self.head_aabb + try_pos;
            // burada is_colliding true ise HEAD ARENA İÇİNDE demektir (senin kullandığın implementasyona göre)
            if moved_head.is_colliding(&self.arena_col) {
                // izin var → pozisyonu uygula
                head_obj.set_cords(try_pos.x, try_pos.y)?;
            } else {
                // arena dışına çıkacaksa hareket etme (şimdi olduğu yerde kal)
                // istersen burada bir ses/efekt veya döngü dönüşü yapabilirsin
            }

            Ok(true)
        }

        fn _on_signal_recieved(&mut self,_sys_name:&String,   _game:&mut Game,_signal:&mut Message,_from:&String) -> RetTick {
            
            let s = _signal.msg.downcast_ref::<&str>().unwrap();
            LOG!(Debug,"signal recieved from {}: {}",_signal.hint,s);

            
            Ok(true)
        }
        
    }



}


mod test{
    use std::{any::Any, fmt::Debug};

    struct Mess{
        m:Box<dyn Any + Send>
    }
    #[derive(Debug)]
    enum Mesag{
        Break,
        Helo
    }


   #[test]
    fn t(){
    let _m1 = Mess{
        m:Box::new("asfasf".to_string())
    };
    let _m2 = Mess{
        m:Box::new(2)
    };
    let _m3 = Mess{
        m:Box::new(Mesag::Break)
    };
    let _m4 = Mess{
        m:Box::new(Mesag::Helo)
    };
    let d = _m1.m.downcast::<String>().unwrap();
    println!("{:?}",d);
    let d = _m2.m.downcast::<i32>().unwrap();
    println!("{:?}",d);
    let d = _m3.m.downcast::<Mesag>().unwrap();
    println!("{:?}",d);
    let d = _m4.m.downcast::<Mesag>().unwrap();
    println!("{:?}",d);

    }
}