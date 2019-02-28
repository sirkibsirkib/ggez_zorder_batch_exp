
use ggez::Context;
use std::collections::HashMap;
use ggez::graphics::Image;
use ggez::graphics::spritebatch::SpriteBatch;
use std::cmp::Ordering;
use ggez::graphics::{self, Point2};
use std::time::{Instant, Duration};

use ggez::graphics::DrawParam;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
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
		let comparator = |x: &(DrawParam, f32)| x.1.partial_cmp(&depth).unwrap_or(Ordering::Equal);
		let index = match self.instances.binary_search_by(comparator) {
			Ok(i) => i+1,
			Err(i) => if i == self.instances.len() {i} else {i+1},
		};
		// println!("inserting at {}", index);
		self.instances.insert(index, (param, depth));
	}
	fn new_singleton(image_key: ImageKey, param: DrawParam, depth: f32) -> Self {
		Self {
			image_key,
			instances: vec![(param, depth)]
		}
	}
	fn split_off(&mut self, depth: f32) -> Self { // returns latter half
		let comparator = |x: &(DrawParam, f32)| x.1.partial_cmp(&depth).unwrap_or(Ordering::Equal);
		let index = self.instances.binary_search_by(comparator).unwrap_or_else(|x| x);
		// println!("split index is {}", index);
		DrawUnit {
			image_key: self.image_key,
			instances: self.instances.split_off(index),
		}
	}
	fn order(&self, depth: f32) -> Ordering {
		let (min, max) = self.bounds();
		if depth < min {
			Ordering::Greater
		} else if depth > max {
			Ordering::Less
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
		panic!("NO MORE KEYS");
	} 

	pub fn add(&mut self, image_key: ImageKey, param: DrawParam, z: f32) {
		// println!("~~~ inserting with z={:?}", z);
		let mut inside_mismatch = None;
		let mut runout_index = self.data.len(); //where it would go if we insert at END
		for i in 0..self.data.len() {
			// let d = &mut self.data[i];
			match self.data[i].order(z) {
				Ordering::Equal => {
					if image_key == self.data[i].image_key {
						// perfect fit!
						// println!(">> perfect fit! {}", i);
						self.data[i].insert(z, param); 
						return;
					} else {
						// maybe split here? keep searching
						// println!(">> mismatch fit at! {}", i);
						inside_mismatch = Some(i);
					}
				}
				Ordering::Less => {
					// need to insert here
					// println!(">> less! {}", i);
					if image_key == self.data[i].image_key {
						if self.data.len() <= i+1 || self.data[i+1].bounds().0 >= z {
							// println!(">> ... but putting in existing");
							self.data[i].insert(z, param); 
							return;
						} else {
							// println!("unfortunately the neighbour would overlap");
						}
					}
				}
				Ordering::Greater => {
					// println!(">> greater at! {}", i);
					if image_key == self.data[i].image_key {
						// println!(">> ... but putting in existing");
						self.data[i].insert(z, param);
						return;
					} else {
						// println!(">> ... putting in new");
						runout_index = i;
						break;
					}
				},
			} 
		}
		if let Some(index) = inside_mismatch {
			// split this object
			// println!(">> split at! {}", index);
			let latter: DrawUnit = self.data[index].split_off(z);
			self.data.insert(index+1, DrawUnit::new_singleton(image_key, param, z));
			self.data.insert(index+2, latter);

		} else {
			// put it at the end
			// println!(">> end at! {}", runout_index);
			self.data.insert(runout_index, DrawUnit::new_singleton(image_key, param, z));
		}
	} 

	pub fn stat(&self) {
		println!("STAT. LEN={}", self.data.len());
		for (i,d) in self.data.iter().enumerate() {
			println!("{} (img={:?}, bounds={:?}, len={:?}", i, d.image_key, d.bounds(), d.instances.len());
		}
	}

	pub fn draw(&mut self, ctx: &mut Context) {
		const DEBUG: bool = false;

		let mut prep = Duration::from_millis(0);
		let mut gfx = Duration::from_millis(0);
		for d in self.data.iter() {
			let image_key = d.image_key;
			let image = self.images.get(&image_key).expect("BAD KEY").clone();

			if d.instances.len() == 1 {
				graphics::draw_ex(ctx, &image, d.instances[0].0).unwrap();
			} else {
				match &mut self.batch {
					None => self.batch = Some(SpriteBatch::new(image)),
					Some(b) => { let _ = b.set_image(image); b.clear(); } ,
				}
				if let Some(ref mut b) = &mut self.batch {

					let t1 = Instant::now();
					for (param, _depth) in d.instances.iter() {
						b.add(param.clone());
					}
					let t2 = Instant::now();
					prep += t2-t1;
					let t2 = Instant::now();
					graphics::draw(ctx, b, Point2::new(0., 0.), 0.).unwrap();
					let t3 = Instant::now();
					gfx += t3-t2;
				} else {
					panic!("BATCH NOT NULL FOR SURE MY GUY");
				}
			}
		}
		if DEBUG {
			println!("prep {:?}, gfx {:?}", prep, gfx);
		}
	}
	pub fn clear(&mut self) {
		self.data.clear();
	}
}

