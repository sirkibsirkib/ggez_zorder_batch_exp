
use ggez::Context;
use std::collections::HashMap;
use ggez::graphics::Image;
use ggez::graphics::spritebatch::SpriteBatch;
use std::cmp::Ordering;
use ggez::graphics::{self, Point2};


use ggez::graphics::DrawParam;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ImageKey(u32);



pub struct DrawUnit {
	image_key: ImageKey,
	instances: Vec<(DrawParam, f32)>,
}
impl DrawUnit {
	fn bounds(&self) -> (f32,f32) {
		assert!(!self.instances.is_empty());
		(
			self.instances.iter().next().unwrap().1,
			self.instances.iter().last().unwrap().1,
		)
	}
	fn insert(&mut self, depth: f32, param: DrawParam) {
		let comparator = |x: &(DrawParam, f32)| depth.partial_cmp(&x.1).unwrap_or(Ordering::Equal);
		let i = self.instances.binary_search_by(comparator).unwrap_or_else(|x| x);
		self.instances.insert(i, (param, depth));
	}
	fn order(&self, depth: f32) -> Ordering {
		let (min, max) = self.bounds();
		if depth < min {
			Ordering::Less
		} else if depth > max {
			Ordering::Greater
		} else {
			Ordering::Equal
		}
	}
}

pub struct Drawer {
	batch: Option<SpriteBatch>,
	data: Vec<DrawUnit>,
	images: HashMap<ImageKey, Image>,
}

impl Drawer {


	pub fn new() -> Self {
		Drawer {
			data: vec![],
			batch: None,
			images: HashMap::new(),
		}
	}

	pub fn add_image(&mut self, image: Image) -> ImageKey {
		for x in 0.. {
			let key = ImageKey(x);
			if !self.images.contains_key(&key) {
				self.images.insert(key, image);
				return key;
			}
		}
		panic!("NIO MORE KEYS");
	} 

	fn find_insertion_for(&self, image_key: ImageKey, z: f32) -> (usize, bool) {
		for (i, d) in self.data.iter().enumerate() {
			match d.order(z) {
				Ordering::Equal => return (i, true), // TODO split case if image_key mismatches
				Ordering::Less => return (i, false),
				Ordering::Greater => {},
			} 
		}
		(self.data.len()-1, false)
	} 

	pub fn add(&mut self, image_key: ImageKey, param: DrawParam, z: f32) {
		let (index, existing) = self.find_insertion_for(image_key, z);
		if !existing {
			let u = DrawUnit {
				image_key,
				instances: vec![(param,z)]
			};
			self.data.insert(index, u);
		}
		self.data[index].insert(z, param);
	}

	pub fn draw(&mut self, ctx: &mut Context) {
		for d in self.data.iter() {
			let image_key = d.image_key;
			let image = self.images.get(&image_key).expect("BAD KEY").clone();
			match &mut self.batch {
				None => self.batch = Some(SpriteBatch::new(image)),
				Some(b) => { let _ = b.set_image(image); b.clear(); } ,
			}
			if let Some(ref mut b) = &mut self.batch {
				for (param, _depth) in d.instances.iter() {
					b.add(param.clone());
				}
				graphics::draw(ctx, b, Point2::new(0., 0.), 0.).unwrap();
			} else {
				panic!("BATCH NOT NULL FOR SURE MY GUY");
			}
		}
	}
	pub fn clear(&mut self) {
		self.data.clear();
	}
}