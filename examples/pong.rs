use termrender::prelude::*;

// made by claude

fn main() -> std::io::Result<()> {
    let mut game = Game::new("Pong".to_string(), 60., 60., 30.);

    let pong = pong::PongSystem::new();
    game.systems._new_system(pong, "pong", true);

    game.setup()?;
    game.main_loop()?;
    game.exit()?;
    drop(game);

    Ok(())
}

mod pong {
    use std::time::Duration;
    use termrender::{
        impl_sys,
        prelude::{math::*, signals::RESIZED, *},
    };

    // --- Signal names ---
    const SCORE_P1: &str = "score::p1";
    const SCORE_P2: &str = "score::p2";

    // --- Arena constants (cells) ---
    // The arena is drawn from (ARENA_X, ARENA_Y) to (ARENA_X+W, ARENA_Y+H)
    const ARENA_W: i32 = 80;
    const ARENA_H: i32 = 24;
    const ARENA_X: i32 = 0;
    const ARENA_Y: i32 = 0;

    // Paddle dimensions (terminal chars: 2 wide, PADDLE_H tall)
    const PADDLE_W: i32 = 2;
    const PADDLE_H: i32 = 4;
    // Ball is 2 wide, 1 tall (block char)
    const BALL_W: i32 = 2;

    // --- Components as string tags ---
    const COMP_BALL: &str = "ball";
    const COMP_PADDLE_L: &str = "paddleL";
    const COMP_PADDLE_R: &str = "paddleR";
    const COMP_SCORE: &str = "score";

    // ================================================================
    //  ScoreSystem  – holds scores, renders scoreboard, listens to signals
    // ================================================================
    #[derive(Default)]
    struct ScoreSystem {
        p1: i32,
        p2: i32,
        score_id: usize,
    }
    impl ScoreSystem {
        fn make_tex(&self) -> GameTexture {
            make_texture!(
                ((0, 0) format!("P1: {}   P2: {}", self.p1, self.p2))
            )
        }
    }
    impl GameSystem for ScoreSystem {
        impl_sys!();

        fn _setup(&mut self, _name: &String, g: &mut Game) -> RetTick {
            // Place score display centred above the arena
            let sx = ARENA_X + ARENA_W / 2 - 7;
            let sy = ARENA_Y + ARENA_H + 1;
            let obj = ObjectBuilder::new()
                .with_attribute(Attribute::Location(Vec2::new(sx, sy)))
                .with_attribute(Attribute::Texture(self.make_tex()))
                .with_component(Components::custom(COMP_SCORE))
                .build();
            self.score_id = g.get_wolrd_mut().insert_object_head(obj)?;
            Ok(true)
        }

        fn _on_signal_recieved(
            &mut self,
            _sys_name: &String,
            g: &mut Game,
            signal: &mut Message,
            from: &String,
        ) -> RetTick {
            match from.as_str() {
                SCORE_P1 => self.p1 += 1,
                SCORE_P2 => self.p2 += 1,
                RESIZED => {}
                _ => {}
            }
            // Refresh score text
            if let Some(obj) = g.get_wolrd_mut().get_with_id(self.score_id) {
                obj.attributes.set_Texture(self.make_tex());
                obj.force_rerender();
            }
            Ok(true)
        }
    }

    // ================================================================
    //  BorderSystem  – draws the arena walls
    // ================================================================
    #[derive(Default)]
    struct BorderSystem;
    impl GameSystem for BorderSystem {
        impl_sys!();
        fn _setup(&mut self, _name: &String, g: &mut Game) -> RetTick {
            // Top border: row ARENA_Y-1
            // Bottom border: row ARENA_Y+ARENA_H
            // Left/Right thin pillars
            let x0 = ARENA_X as i32;
            let y0 = ARENA_Y as i32;
            let x1 = (ARENA_X + ARENA_W) as i32;
            let y1 = (ARENA_Y + ARENA_H) as i32;

            let border_tex = make_texture!(
                // top row
                ((x0 - 1, y0 - 1) => (x1 + 1, y0 - 1) ("\u{2584}")),
                // bottom row
                ((x0 - 1, y1) => (x1 + 1, y1) ("\u{2580}")),
            );
            let border = ObjectBuilder::new()
                .with_attribute(Attribute::Location(Vec2::new(0, 0)))
                .with_attribute(Attribute::Texture(border_tex))
                .with_component(Components::Wall)
                .build();
            g.get_wolrd_mut().insert_object_head(border)?;
            Ok(true)
        }
    }

    // ================================================================
    //  PaddleSystem  – one system controls both paddles
    // ================================================================
    struct PaddleSystem {
        left_id: usize,
        right_id: usize,
        // y positions (top of paddle)
        left_y: i32,
        right_y: i32,
    }
    impl PaddleSystem {
        fn new() -> Self {
            let mid = ARENA_Y + ARENA_H / 2 - PADDLE_H / 2;
            Self {
                left_id: 0,
                right_id: 0,
                left_y: mid,
                right_y: mid,
            }
        }
        fn make_paddle_tex(color: GColor) -> GameTexture {
            // PADDLE_H rows of "██"
            let mut parts: Vec<PrintTypes> = Vec::new();
            for row in 0..PADDLE_H {
                let s = format!("{}{}", "\u{2588}\u{2588}", "");
                parts.push(PrintTypes::String {
                    pos: Vec2::new(0, row),
                    char: TermPrint::from((s.as_str(), (), color)),
                });
            }
            GameTexture::new(parts)
        }
        fn clamp_y(y: i32) -> i32 {
            y.max(ARENA_Y).min(ARENA_Y + ARENA_H - PADDLE_H)
        }
    }
    impl GameSystem for PaddleSystem {
        impl_sys!();
        fn _setup(&mut self, _name: &String, g: &mut Game) -> RetTick {
            let mid = ARENA_Y + ARENA_H / 2 - PADDLE_H / 2;

            // Left paddle: x = ARENA_X + 1
            let lx = ARENA_X + 1;
            let left = ObjectBuilder::new()
                .with_attribute(Attribute::Location(Vec2::new(lx, mid)))
                .with_attribute(Attribute::Texture(Self::make_paddle_tex(GColor::Cyan)))
                .with_attribute(Attribute::Col(AABB::from(((0, 0), (PADDLE_W - 1, PADDLE_H - 1)))))
                .with_component(Components::custom(COMP_PADDLE_L))
                .build();
            self.left_id = g.get_wolrd_mut().insert_object_head(left)?;

            // Right paddle: x = ARENA_X + ARENA_W - PADDLE_W - 1
            let rx = ARENA_X + ARENA_W - PADDLE_W - 1;
            let right = ObjectBuilder::new()
                .with_attribute(Attribute::Location(Vec2::new(rx, mid)))
                .with_attribute(Attribute::Texture(Self::make_paddle_tex(GColor::Magenta)))
                .with_attribute(Attribute::Col(AABB::from(((0, 0), (PADDLE_W - 1, PADDLE_H - 1)))))
                .with_component(Components::custom(COMP_PADDLE_R))
                .build();
            self.right_id = g.get_wolrd_mut().insert_object_head(right)?;

            Ok(true)
        }

        fn _physics_loop(&mut self, _delta: Duration, _name: &String, g: &mut Game) -> RetTick {
            let speed = 1;

            // Left paddle: W/S
            if g.input.pressed_keys.contains(&Keys::Up) {
                self.left_y = Self::clamp_y(self.left_y - speed);
            }
            if g.input.pressed_keys.contains(&Keys::Down) {
                self.left_y = Self::clamp_y(self.left_y + speed);
            }

            // Right paddle: E/Q (mapped to Up/Down via Keys::E and Keys::Q)
            if g.input.pressed_keys.contains(&Keys::E) {
                self.right_y = Self::clamp_y(self.right_y - speed);
            }
            if g.input.pressed_keys.contains(&Keys::Q) {
                self.right_y = Self::clamp_y(self.right_y + speed);
            }

            // Apply positions
            if let Some(obj) = g.get_wolrd_mut().get_with_id(self.left_id) {
                let cur = obj.get_cords().unwrap();
                obj.set_cords(cur.x, self.left_y)?;
            }
            if let Some(obj) = g.get_wolrd_mut().get_with_id(self.right_id) {
                let cur = obj.get_cords().unwrap();
                obj.set_cords(cur.x, self.right_y)?;
            }

            Ok(true)
        }
    }

    // ================================================================
    //  BallSystem  – ball movement, bouncing, scoring
    // ================================================================
    struct BallSystem {
        ball_id: usize,
        // velocity in half-cells per physics tick
        vx: i32,
        vy: i32,
        // sub-pixel accumulators (×100 fixed point)
        fx: i32,
        fy: i32,
        // tick counter to control speed
        tick: u32,
        speed_ticks: u32, // move every N ticks
    }
    impl BallSystem {
        fn new() -> Self {
            Self {
                ball_id: 0,
                vx: 2,
                vy: 1,
                fx: 0,
                fy: 0,
                tick: 0,
                speed_ticks: 3, // moves every 3 physics ticks (~20 moves/sec at 60hz)
            }
        }
        fn reset_ball(&mut self, g: &mut Game, to_left: bool) {
            let cx = ARENA_X + ARENA_W / 2;
            let cy = ARENA_Y + ARENA_H / 2;
            if let Some(ball) = g.get_wolrd_mut().get_with_id(self.ball_id) {
                ball.set_cords(cx, cy).ok();
            }
            self.vx = if to_left { -2 } else { 2 };
            self.vy = 1;
        }
    }
    impl GameSystem for BallSystem {
        impl_sys!();

        fn _setup(&mut self, _name: &String, g: &mut Game) -> RetTick {
            let cx = ARENA_X + ARENA_W / 2;
            let cy = ARENA_Y + ARENA_H / 2;

            let ball_tex = make_texture!(((0, 0) ("██", (), GColor::Yellow)));
            let ball = ObjectBuilder::new()
                .with_attribute(Attribute::Location(Vec2::new(cx, cy)))
                .with_attribute(Attribute::Texture(ball_tex))
                .with_attribute(Attribute::Col(AABB::from(((0, 0), (BALL_W - 1, 0)))))
                .with_component(Components::custom(COMP_BALL))
                .build();
            self.ball_id = g.get_wolrd_mut().insert_object_head(ball)?;
            Ok(true)
        }

        fn _physics_loop(&mut self, _delta: Duration, _name: &String, g: &mut Game) -> RetTick {
            self.tick += 1;
            if self.tick < self.speed_ticks {
                return Ok(true);
            }
            self.tick = 0;

            // Get current ball position
            let ball_pos = {
                let ball = g.get_wolrd_mut().get_with_id(self.ball_id).unwrap();
                ball.get_cords().unwrap()
            };

            let nx = ball_pos.x + self.vx;
            let ny = ball_pos.y + self.vy;

            // --- Top / bottom wall bounce ---
            let mut ny = ny;
            let mut vx = self.vx;
            let mut vy = self.vy;

            if ny < ARENA_Y {
                ny = ARENA_Y;
                vy = -vy;
            } else if ny >= ARENA_Y + ARENA_H {
                ny = ARENA_Y + ARENA_H - 1;
                vy = -vy;
            }

            // --- Score / left-right wall check ---
            let mut nx = nx;
            let mut scored = false;
            let mut score_signal = "";

            if nx < ARENA_X {
                // P2 scores
                score_signal = SCORE_P2;
                scored = true;
            } else if nx >= ARENA_X + ARENA_W - BALL_W + 1 {
                // P1 scores
                score_signal = SCORE_P1;
                scored = true;
            }

            if scored {
                g.send_signal(score_signal, COMP_SCORE, Box::new(()));
                let to_left = score_signal == SCORE_P2;
                self.reset_ball(g, to_left);
                self.vx = if to_left { -2 } else { 2 };
                self.vy = 1;
                g.set_flag(termrender::game::Flags::Rerender);
                return Ok(true);
            }

            // --- Paddle collision ---
            let ball_world = AABB::from(((nx, ny), (nx + BALL_W - 1, ny)));

            // Left paddle
            let left_paddle_col = {
                let obj = g.get_wolrd_mut().query_comp(Components::custom(COMP_PADDLE_L));
                obj.map(|o| {
                    let loc = o.get_cords().unwrap();
                    let col = *o.attributes.get_Col().unwrap();
                    col + loc
                })
            };
            // Right paddle
            let right_paddle_col = {
                let obj = g.get_wolrd_mut().query_comp(Components::custom(COMP_PADDLE_R));
                obj.map(|o| {
                    let loc = o.get_cords().unwrap();
                    let col = *o.attributes.get_Col().unwrap();
                    col + loc
                })
            };

            if let Some(lc) = left_paddle_col {
                if ball_world.is_colliding(&lc) && vx < 0 {
                    vx = -vx;
                    nx = lc.cor2.x + 1;
                }
            }
            if let Some(rc) = right_paddle_col {
                if ball_world.is_colliding(&rc) && vx > 0 {
                    vx = -vx;
                    nx = rc.cor1.x - BALL_W;
                }
            }

            self.vx = vx;
            self.vy = vy;

            // Move the ball
            if let Some(ball) = g.get_wolrd_mut().get_with_id(self.ball_id) {
                ball.set_cords(nx, ny).ok();
            }

            Ok(true)
        }
    }

    // ================================================================
    //  HelpSystem  – shows controls at the bottom
    // ================================================================
    #[derive(Default)]
    struct HelpSystem {
        obj_id: usize,
    }
    impl GameSystem for HelpSystem {
        impl_sys!();
        fn _setup(&mut self, _name: &String, g: &mut Game) -> RetTick {
            let sy = ARENA_Y + ARENA_H + 2;
            let help_tex = make_texture!(
                ((0, 0) ("P1: W/S (arrows)   P2: Q/E   ESC: quit", (), GColor::DarkGrey))
            );
            let obj = ObjectBuilder::new()
                .with_attribute(Attribute::Location(Vec2::new(ARENA_X, sy)))
                .with_attribute(Attribute::Texture(help_tex))
                .build();
            self.obj_id = g.get_wolrd_mut().insert_object_head(obj)?;
            Ok(true)
        }
    }

    // ================================================================
    //  PongSystem  – top-level, creates all sub-systems
    // ================================================================
    pub struct PongSystem;
    impl PongSystem {
        pub fn new() -> Self {
            Self
        }
    }
    impl GameSystem for PongSystem {
        impl_sys!();
        fn _setup(&mut self, _name: &String, g: &mut Game) -> RetTick {
            // Register sub-systems
            g.get_systems_mut()
                ._new_system(BorderSystem, "border", true);

            g.get_systems_mut()
                ._new_system(PaddleSystem::new(), "paddles", true);

            g.get_systems_mut()
                ._new_system(BallSystem::new(), "ball", true);

            // Score system listens for scoring signals
            g.get_systems_mut().new_system_with_helper(
                Box::new(ScoreSystem::default()),
                COMP_SCORE,
                true,
                Box::new(|s| {
                    s.register_signal(SCORE_P1);
                    s.register_signal(SCORE_P2);
                    s.register_signal(RESIZED);
                }),
            );

            g.get_systems_mut()
                ._new_system(HelpSystem::default(), "help", true);

            Ok(true)
        }
    }
}