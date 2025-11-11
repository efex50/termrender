
use termrender::prelude::*;
use crate::ticks::{FoodSystem, SnakeHead};

const START:&str = "sim.play";

fn main() -> std::io::Result<()> {

    let mut _main = Game::new("zort".to_string(),30.1,100.,30.);
    // logging to a file
    
    //{
    //    let logpath = format!("{}",file!());
    //    let logpath = logpath.trim_end_matches(".rs");
    //    _main.set_log_path(Some(logpath.to_string()));
    //}
    
    
    
    //_main.systems.new_system(Box::new(Redrawer::new()), "redrawer", true);

    //let psys = PlayerSystem::new((50,25));
    //_main.systems.new_system_with_helper(Box::new(psys),"player".to_string(),true,Box::new(|s| {
    //}));

    LOG!(Debug,"starting the game");


    let snek = SnakeHead::new((50,25));
    
    _main.systems._new_system(snek, "yılan", true);
    

    _main.setup()?;
    _main.main_loop()?;
    _main.systems.get_system_with_registered(&"skkor=".to_string());
    _main.exit()?;
    drop(_main);
    
    return Ok(());
}


mod ticks{

    
    use std::time::Duration;
    
    use rand::Rng;
    use termrender::{comp, game::screen::Screen, impl_sys, prelude::{math::*, signals::RESIZED, *}};
    
    use crate::START;
    
    const SCORE_COMP:&str = "skkor";
    const SCORE_INC:&str = "skkor+";
    const SCORE_DEC:&str = "skkor-";
    const SCORE_BY:&str = "skkor=";

    #[derive(Default)]
    struct BorderSystem{
        area:AABB,
        min_screen:Vec2,
        skor:i32,
        self_id:usize,
    }
    impl BorderSystem {
        fn regen_tex(&self) -> GameTexture{
            make_texture!(
                ((0,0) format!("score:{}",self.skor))
            )
        }
        fn error_tex(&self,errmsg:String) -> GameTexture{
            let len_err: usize = errmsg.chars().count();
            let current_size = Screen::get_size().unwrap();
            let current_str = format!("current {}x{}",current_size.x,current_size.y);
            let curr_len = current_str.chars().count();
            let c_loc = PrintTypes::String { pos: Vec2 { x: -((curr_len / 2) as i32), y: 1 }, char: TermPrint::from((current_str,GColor::DarkBlue))};
            let t = PrintTypes::String { pos: Vec2 { x: -((len_err / 2) as i32), y: 0 }, char: TermPrint::from(errmsg)  };
            make_texture!(
                t,
                c_loc,
            )
        }
        fn calculate_screen(&mut self,g:&Game) -> RetTick{
            let borders = g.get_wolrd_mut().query_comp(Components::Wall).unwrap();
            let score_comp = g.get_wolrd_mut().query_comp(Components::Custom(SCORE_COMP.to_string())).unwrap();
            let screen_size = g.get_screen()._get_size().unwrap();
            let mut desired = self.area.cor2;
            desired.x += 3;
            desired.y += 3;
            // if small
            if screen_size.x < desired.x || screen_size.y < desired.y{
                borders.attributes.set_Render(false);
                let mut err_pos = screen_size.clone();
                err_pos.x /= 2;
                if err_pos.x <0{
                    err_pos.x = 0;
                }
                err_pos.y /= 2;
                if err_pos.y < 0{
                    err_pos.y = 0
                }
                score_comp.attributes.set_Location(err_pos);
                score_comp.attributes.set_Texture(self.error_tex(format!("size should be {}x{}",desired.x,desired.y)));
                // bütün objeleri gizle
                g.get_wolrd_mut().map_objects( |a|{
                    if !(a.id == score_comp.id){
                        a.attributes.set_Render(false);
                    }
                });
                g.unset_flag(termrender::game::Flags::Custom(START.to_string()));
                g.set_flag(termrender::game::Flags::Rerender);
            }else {

                let corners = self.area.get_corners();
                borders.attributes.set_Render(true);
                let score_pos = Vec2::from((0,self.area.cor2.y+2));
                score_comp.attributes.set_Location(score_pos);
                score_comp.attributes.set_Texture(self.regen_tex());


                g.set_flag(termrender::game::Flags::Custom(START.to_string()));
                g.set_flag(termrender::game::Flags::Rerender);

            }
            Ok(true)
        }
    }
    impl GameSystem for BorderSystem {
        impl_sys!();
        fn _setup(&mut self,_sys_name:&String,_game:&mut Game) -> RetTick {

            let corners = self.area.get_corners();
            let borders = ObjectBuilder::new()
                .with_attribute(Attribute::Location((0,0).into()))
                .with_attribute(Attribute::Tag("Borders".to_string()))
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
                    )
                ))
                .with_component(Components::Wall)
                .build();
            let scoreboard = ObjectBuilder::new()
                .with_attribute(Attribute::Location(Vec2::from((0,corners.3.y+2))))
                .with_component(Components::Custom(SCORE_COMP.to_string()))
                .with_attribute(Attribute::Texture(self.regen_tex()))
                .with_attribute(Attribute::Tag("score".to_string()))
                .build();
            let world = _game.get_wolrd_mut();
            // set the minimum screen size
            let mut desired = self.area.cor2;
            desired.x += 3;
            desired.y += 3;
            self.min_screen = desired;

            world.insert_object_head(borders)?;
            let se= world.insert_object_head(scoreboard)?;
            self.self_id = se;
            self.calculate_screen(&_game)?;
            Ok(true)
        }
        fn _on_signal_recieved(&mut self,_sys_name:&String,   _game:&mut Game,_signal:&mut Message,_from:&String) -> RetTick {
            let w = _game.get_wolrd_mut();
            let o = w.query_comp(Components::Custom(SCORE_COMP.to_string()));
            if o.is_none(){
                return Ok(true);
            }
            let o = o.unwrap();
            let hint = _signal.hint.as_str();
            match hint {
                SCORE_DEC |
                SCORE_INC |
                SCORE_BY => {
                    match hint {
                        SCORE_DEC => self.skor -= 1,
                        SCORE_INC => self.skor += 1,
                        SCORE_BY  => {
                            let amount = _signal.msg.downcast_ref::<i32>().unwrap();
                            self.skor += amount;
                        },
                        _ => LOG!(Error,"unrecognized signal")
                    };

                    o.attributes.set_Texture(self.regen_tex());
                    o.force_rerender();
                },
                RESIZED => {
                    self.calculate_screen(&_game)?;
                },
                _ => {  
                    LOG!(Error,"Unrecognized signal:{}",_signal.hint);
                }
            };
            
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
        pub arena_col: AABB,    // allowed area (player must stay inside)

    }
    impl SnakeHead {
        pub fn new(arena_size: (i32, i32)) -> Self {
            Self {
                head_id: 0,
                dir: Direction::Right,
                // head AABB 1x1 starting at 0,0 (ayarla istersen)
                head_aabb: AABB { cor1: Vec2 { x: 0, y: 0 }, cor2: Vec2 { x: 1, y: 0 } },
                // arena: örn (1,1) .. (arena.0*2, arena.1)
                arena_col: AABB::from((Vec2::from((1,1)), Vec2::from((arena_size.0*2, arena_size.1)))),
            }
        }
        fn should_play(&self,g:&Game) -> bool{
            g.has_flag(termrender::game::Flags::Custom(START.to_string()))
        }

    }
    impl GameSystem for SnakeHead {
        impl_sys!();

        fn _setup(&mut self, _name: &String, g: &mut Game) -> RetTick {
            LOG!(Warn,"snakeHead setup started");
            let w = g.get_wolrd_mut();
            let bor = BorderSystem{
                area:self.arena_col,
                ..Default::default()
            };
            let food_sys = FoodSystem::new(self.arena_col);    

            g.get_systems_mut().new_system_with_helper(Box::new(bor), SCORE_COMP, true,Box::new(|a|{
                a.register_signal(RESIZED);
            }));
            g.get_systems_mut()._new_system(food_sys, "Food", true);

            
            // Head texture: tek kare kırmızı blok (veya istediğin)
            let t = make_texture!(((0,0)("██",(),GColor::Red)));
            let player = ObjectBuilder::new()
                .with_attribute(Attribute::Location(Vec2::from((3,3))))
                .with_attribute(Attribute::Texture(t))
                .with_attribute(Attribute::Col(self.head_aabb))
                .with_attribute(Attribute::Tag("SnakeHead".to_string()))
                .with_component(Components::custom("SnakeHead"))
                .with_component(Components::Player)
                .build();
            let id = w.insert_object_head(player)?;
            self.head_id = id;

            let tail_sys = SnakeTail::new(self.head_id);
            g.get_systems_mut().new_system(Box::new(tail_sys), "SnakeTail".to_string(), true);
            Ok(true)
        }

        fn _physics_loop(&mut self, _delta:Duration,_name: &String, g: &mut Game) -> RetTick {
            let head_obj = g.get_self_mut().world.get_with_id(self.head_id).unwrap();
            

            if !self.should_play(&g){
                head_obj.attributes.set_Render(false);
            }else{
                head_obj.attributes.set_Render(true);
                if !head_obj.is_printed(){
                    head_obj.force_rerender();
                }
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
            }
            Ok(true)
        }
        
    }




    pub struct SnakeTail {
        head_id: usize,          // Head objesi ID
        segments: Vec<usize>,    // Kuyruk segment objeleri ID
        grow: usize,             // kaç segment büyüyecek
    }

    impl SnakeTail {
        pub fn new(head_id: usize) -> Self {
            Self {
                head_id,
                segments: Vec::new(),
                grow: 0,
            }
        }

        pub fn grow(&mut self, n: usize) {
            self.grow += n;
        }
    }

    impl GameSystem for SnakeTail {
        impl_sys!();

        fn _setup(&mut self, _name: &String, _game: &mut Game) -> RetTick {
            // Başlangıçta kuyruk yok
            Ok(true)
        }

        fn _physics_loop(&mut self, _delta: Duration, _name: &String, g: &mut Game) -> RetTick {
            let head_obj = g.get_self_mut().world.get_with_id(self.head_id).unwrap();
            let head_pos = head_obj.get_cords().unwrap();

            let mut prev_pos = head_pos;

            for &seg_id in &self.segments {
                if let Some(seg) = g.get_self_mut().world.get_with_id(seg_id) {
                    let seg_pos = seg.get_cords().unwrap();
                    seg.set_cords(prev_pos.x, prev_pos.y)?;
                    if g.has_flag(termrender::game::Flags::Custom(START.to_string())){
                        seg.attributes.set_Render(true);
                    }
                    prev_pos = seg_pos;
                }   
            }

            // Eğer grow > 0 ise yeni segment ekle
            if self.grow > 0 {
                let t = make_texture!(((0,0)("██",(),GColor::Green)));
                let new_seg = ObjectBuilder::new()
                    .with_attribute(Attribute::Location(prev_pos))
                    .with_attribute(Attribute::Texture(t))
                    .with_attribute(Attribute::Col(AABB { cor1: Vec2::from(0), cor2: Vec2::from((1,0)) }))
                    .with_component(comp!(SnakeTail))
                    .with_attribute(Attribute::Tag("SnakeTail".to_string()))
                    .build();
                let id = g.get_wolrd_mut().insert_object_head(new_seg)?;
                self.segments.push(id);
                self.grow -= 1;
            }

            Ok(true)
        }
    }





    pub struct FoodSystem {
        food_id: Option<usize>,       // Aktif yemin objesi
        arena: AABB,                  // Yem arena sınırı
    }

    impl FoodSystem {
        pub fn new(arena: AABB) -> Self {
            Self { food_id: None, arena }
        }
        fn move_food(&mut self, g: &mut Game) -> RetTick {
            if let Some(id) = self.food_id {
                let mut rng = rand::rng();
                let x = rng.random_range(self.arena.cor1.x..self.arena.cor2.x);
                let y = rng.random_range(self.arena.cor1.y..self.arena.cor2.y);
                let pos = Vec2::from((x, y));

                if let Some(food_obj) = g.get_wolrd_mut().get_with_id(id) {
                    food_obj.set_cords(pos.x, pos.y)?;
                }
            }
            Ok(true)
        }

    }
    impl GameSystem for FoodSystem {
        impl_sys!();
    
        fn _setup(&mut self, _name: &String, g: &mut Game) -> RetTick {
            // Başlangıçta food spawn
            let mut rng = rand::rng();
            let x = rng.random_range(self.arena.cor1.x..self.arena.cor2.x);
            let y = rng.random_range(self.arena.cor1.y..self.arena.cor2.y);
            let pos = Vec2::from((x, y));

            let t = make_texture!(((0,0)("██",(),GColor::Yellow)));
            let food = ObjectBuilder::new()
                .with_attribute(Attribute::Location(pos))
                .with_attribute(Attribute::Col(AABB::from(((0,0),(1,0)))))
                .with_attribute(Attribute::Texture(t))
                .with_attribute(Attribute::Tag("Food".to_string()))
                .with_component(comp!(Food))
                .build();
            let id = g.get_wolrd_mut().insert_object_head(food)?;
            self.food_id = Some(id);
            Ok(true)
        }

        fn _physics_loop(&mut self, _delta: Duration, _name: &String, g: &mut Game) -> RetTick {
            let head_obj = g.get_self_mut().world.query_comp(comp!(SnakeHead)).unwrap();
            let head_loc = head_obj.get_cords().unwrap();
            let head_col_local = head_obj.attributes.get_Col().unwrap();
            let head_col_world = *head_col_local + head_loc; // senin AABB + Vec2 impl'in var gibi kullanalım
            if let Some(food_id) = self.food_id {
                // al head obj ve food obj

                let food_obj = g.get_self_mut().world.get_with_id(food_id).unwrap();
                let food_loc = food_obj.get_cords().unwrap();
                let food_col_local = food_obj.attributes.get_Col().unwrap();
                let food_col_world = *food_col_local + food_loc;

                if head_col_world.is_colliding(&food_col_world) {
                    LOG!(Debug, "Head collided with food!");
                
                    // Skor sistemine +1 sinyali gönder
                    g.send_signal(SCORE_INC.to_string(),SCORE_COMP.to_string(), Box::new(()));
                
                    // SnakeTail büyüsün
                    if let Some(snake_tail) = g.get_systems_mut().get_system_mut_as::<SnakeTail, &str>("SnakeTail") {
                        snake_tail.grow(1);
                    }
                
                    // Yem pozisyonunu değiştir (delete değil)
                    self.move_food(g)?;
                }
            }
            Ok(true)
        }
    }



}

