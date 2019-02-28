

use ggez::event::Mod;
use ggez::event::Keycode;
use std::collections::HashMap;
use ggez::Context;
use ggez::error::GameResult;

use ggez::graphics::DrawParam;
use ggez::graphics::{Point2};
use ggez::{graphics, event, conf};
use std::time::{Instant};
use ggez::graphics::Rect;

#[macro_use] extern crate maplit;


mod drawer;

use drawer::{Drawer, ImageKey};

struct MainState {
    drawer: Drawer,
    entities: HashMap<Eid, Entity>,
    sprite_finder: SpriteFinder,
    protag_eid: Eid,
}

#[derive(Debug)]
enum Direction {
    Left, Right, Up, Down, None,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Eid(u32);

#[derive(Debug)]
struct Entity {
    param: DrawParam,
    depth: f32,
    image_key: ImageKey,
    movement: Direction,
    facing_right: bool,
    ran_ticks: u32,
}


#[derive(Debug)]
struct SpriteFinder {
    max_sprite_ids: [u16;2],
    mult: [f32;2],
}
impl SpriteFinder {
    pub fn new(sheet_size: [u16;2], sprite_dims: [u16;2]) -> Self {
        Self {
            max_sprite_ids: [
                sheet_size[0] / sprite_dims[0],
                sheet_size[1] / sprite_dims[1],
            ],
            mult: [
                sprite_dims[0] as f32 / sheet_size[0] as f32,
                sprite_dims[1] as f32 / sheet_size[1] as f32,
            ],
        }
    }
    pub fn find(&self, sprite_coord: [u16;2]) -> Result<Rect,()> {
        if sprite_coord[0] <= self.max_sprite_ids[0]
        && sprite_coord[1] <= self.max_sprite_ids[1] {
            Ok(Rect {
                x: self.mult[0] * sprite_coord[0] as f32,
                y: self.mult[1] * sprite_coord[1] as f32,
                w: self.mult[0],
                h: self.mult[1],
            })
        } else {
            Err(())
        }
    }
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
    	let mut img = graphics::Image::new(ctx, "/adventurer.png")?;
        img.set_filter(graphics::FilterMode::Nearest);


        let sprite_finder = SpriteFinder::new([img.width() as u16, img.height() as u16], [50,37]);
        println!("finder: {:?}", &sprite_finder);


        let mut drawer = Drawer::new();
        let image_key = drawer.add_image(img);


        let param = DrawParam {
            src: sprite_finder.find([0,0]).unwrap(),
            scale: Point2::new(-1.0, 1.0),
            offset: Point2::new(0.5, 0.8),
            dest: Point2::new(200.0, 300.0),
            ..Default::default()
        };
        let protag = Entity {
            param,
            depth: 0.5,
            image_key,
            movement: Direction::None,
            facing_right: true,
            ran_ticks: 0,
        };
        
        let s = MainState {
            drawer,
            sprite_finder,
            entities: hashmap!{ Eid(0) => protag },
            protag_eid: Eid(0),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.drawer.clear();
        let start = Instant::now();
        for (_eid, e) in self.entities.iter_mut() {
            self.drawer.add(e.image_key, e.param, e.depth);
            match e.movement {
                Direction::None => e.ran_ticks = 0,
                Direction::Left => {
                    e.ran_ticks += 1;
                    let sprite_coord = [
                        1 + (e.ran_ticks/20) as u16,
                        1,
                    ];
                    e.param.src = self.sprite_finder.find(sprite_coord).unwrap();
                }
                _ => unreachable!(),
            }
        }
        println!("update took {:?} ", start.elapsed());
        // self.drawer.stat();
        Ok(())
    }

    fn key_down_event(
        &mut self, 
        _ctx: &mut Context, 
        keycode: Keycode, 
        _keymod: Mod, 
        _repeat: bool,
    ) {
        match keycode {
            Keycode::A => {
                let e = &mut self.entities.get_mut(&self.protag_eid).unwrap();
                e.movement = Direction::Left;
                e.facing_right = false;
            },
            _ => {},
        }
    }

    fn key_up_event(
        &mut self, 
        _ctx: &mut Context, 
        keycode: Keycode, 
        _keymod: Mod, 
        _repeat: bool,
    ) {
        match keycode {
            Keycode::A => self.entities.get_mut(&self.protag_eid).unwrap().movement = Direction::None,
            _ => {},
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        let x = Instant::now();
        self.drawer.draw(ctx);
        println!("draw took {:?}", x.elapsed());
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
    
}