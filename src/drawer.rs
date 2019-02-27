
use std::cmp::Ordering;


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
	data: Vec<DrawUnit>
}

impl Drawer {


	pub fn new() -> Self {
		Drawer {
			data: vec![],
		}
	}

	fn find_insertion_for(&self, image_key: ImageKey, z: f32) -> (usize, bool) {
		for (i, d) in self.data.iter().enumerate() {
			match d.order(z) {
				Equal => return (i, true),
				Less => return (i, false),
				Greater => {},
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

	pub fn draw(&self) {
		//TODO
	}
	pub fn clear(&self) {
		self.data.clear();
	}
}