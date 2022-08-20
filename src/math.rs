use crate::traits::ToF32;

#[test]
fn test_map() {
	assert_eq!(2.5f32, 0f32.map(-1isize, 1, 2, 3));
	assert_eq!(9.5f32, (-4f32).map(-5f32, 2f32, 9f32, 12.5));
}
pub trait Math {
	fn map<T: ToF32>(self, r_min: T, r_max: T, m_min: T, m_max: T) -> f32;
	fn round_i(self) -> usize;
}
impl Math for f32 {
	fn map<T: ToF32>(self, range_min: T, range_max: T, mapped_min: T, mapped_max: T) -> f32 {
		((self - range_min.to_f32()) / (range_max.to_f32() - range_min.to_f32()))
			* (mapped_max.to_f32() - mapped_min.to_f32())
			+ mapped_min.to_f32()
	}
	fn round_i(self) -> usize {
		if (self - (self as usize).to_f32()) > 0.5f32 {
			return self as usize + 1;
		}
		self as usize
	}
}
