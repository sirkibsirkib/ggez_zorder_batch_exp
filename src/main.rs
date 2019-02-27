

use ggez::Context;
use ggez::error::GameResult;

use ggez::graphics::DrawParam;
use ggez::graphics::Rect;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::{Point2};
use ggez::{graphics, event, conf};
use std::time::Instant;


mod drawer;

use drawer::{Drawer, ImageKey};

struct MainState {
    batches: Vec<SpriteBatch>,
    drawer: Drawer,
}


impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
    	let mut img = graphics::Image::new(ctx, "/sheet.png")?;
        img.set_filter(graphics::FilterMode::Nearest);

        let start = Instant::now();
        let batches = [5, 10, 20, 100, 1000, 30, 20, 5, 5, 5, 5000, 2, 2, 40, 50, 100].into_iter().map(|&num| {
            let mut batch = SpriteBatch::new(img.clone());
            for i in 0..num {
                batch.add(DrawParam {
                    src: Rect {
                        x: (i%4) as f32 * 0.25,
                        y: ((num/7)%4) as f32 * 0.25,
                        w: 0.25,
                        h: 0.25,
                    },
                    scale: Point2::new(0.3, 0.3),
                    dest: Point2::new(700.0 * i as f32 / num as f32, 500.0 * i as f32 / num as f32),
                    ..DrawParam::default()
                });
            }
            batch
        }).collect();
        println!("setup took {:?} ", start.elapsed());
        let mut drawer = Drawer::new();
        
        let s = MainState {
        	batches,
            drawer,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        self.drawer.draw(ctx);
        let x = Instant::now();
        self.drawer.clear();
        // for batch in self.batches.iter() {
        //     graphics::draw(ctx, batch, Point2::new(50., 50.), 0.).unwrap();
        // }
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