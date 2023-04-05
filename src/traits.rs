pub trait ToF32 {
	fn to_f32(&self) -> f32;
}
impl ToF32 for usize {
	fn to_f32(&self) -> f32 {
		*self as f32
	}
}
impl ToF32 for isize {
	fn to_f32(&self) -> f32 {
		*self as f32
	}
}
impl ToF32 for f32 {
	fn to_f32(&self) -> f32 {
		*self
	}
}
impl ToF32 for i32 {
	fn to_f32(&self) -> f32 {
		*self as f32
	}
}

pub trait ToUsize {
	fn to_usize(&self) -> usize;
}
impl ToUsize for usize {
	fn to_usize(&self) -> usize {
		*self
	}
}
impl ToUsize for isize {
	fn to_usize(&self) -> usize {
		*self as usize
	}
}
impl ToUsize for i32 {
	fn to_usize(&self) -> usize {
		*self as usize
	}
}
