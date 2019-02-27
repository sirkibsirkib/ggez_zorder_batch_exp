

use ggez::Context;
use ggez::error::GameResult;

use ggez::graphics::DrawParam;
use ggez::graphics::Rect;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::{Point2};
use ggez::{graphics, event, conf};
use std::time::{Instant, Duration};


mod drawer;

use drawer::{Drawer, ImageKey};

struct MainState {
    // batches: Vec<SpriteBatch>,
    drawer: Drawer,
}


impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
    	let mut img = graphics::Image::new(ctx, "/sheet.png")?;
        img.set_filter(graphics::FilterMode::Nearest);

        let mut drawer = Drawer::new();

        let image_key = drawer.add_image(img.clone());
        let image_key2 = drawer.add_image(img.clone());
        let param = || {
            DrawParam {
                src: Rect {
                    x: (rand::random::<u8>()%4) as f32 * 0.25,
                    y: (rand::random::<u8>()%4) as f32 * 0.25,
                    w: 0.25,
                    h: 0.25,
                },
                scale: Point2::new(0.3, 0.3),
                dest: Point2::new(rand::random::<f32>() * 500.0, rand::random::<f32>() * 480.0),
                ..DrawParam::default()
            }
        };
        
        let start = Instant::now();
        for _ in 0..1000 {
            drawer.add(image_key, param(), rand::random::<f32>());
            drawer.add(image_key2, param(), rand::random::<f32>());
        }
        println!("setup took {:?} ", start.elapsed());
        drawer.stat();
        
        let s = MainState {
        	// batches,
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
        let x = Instant::now();
        // self.drawer.clear();
        self.drawer.draw(ctx);
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