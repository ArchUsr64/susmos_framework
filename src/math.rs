use crate::traits::ToF32;

#[test]
fn test_map() {
	assert_eq!(2.5f32, 0f32.map(-1isize, 1, 2, 3));
	assert_eq!(9.5f32, (-4f32).map(-5f32, 2f32, 9f32, 12.5));
}
#[test]
fn test_round_i() {
	assert_eq!(9, 9.3.round_i());
	assert_eq!(9, 9.5.round_i());
	assert_eq!(10, 9.6.round_i());
}
#[test]
fn test_mantissa() {
	assert_eq!(9, 9.312f32.mantissa());
	assert_eq!(9, 0.95.mantissa());
	assert_eq!(-9, (-0.95).mantissa());
	assert_eq!(-8, (-0.085).mantissa());
	assert_eq!(9, 96f32.mantissa());
	assert_eq!(0, 0f32.mantissa());
}
#[test]
fn test_exponent() {
	assert_eq!(2, 110.093f32.exponent());
	assert_eq!(-2, 0.093f32.exponent());
	assert_eq!(-2, (-0.01f32).exponent());
	assert_eq!(0, 0f32.exponent());
}
pub trait Math {
	fn map<T: ToF32>(self, r_min: T, r_max: T, m_min: T, m_max: T) -> f32;
	fn round_i(self) -> isize;
	fn mantissa(self) -> isize;
	fn fractional_part(self) -> f32;
	fn exponent(self) -> isize;
}
impl Math for f32 {
	fn map<T: ToF32>(self, range_min: T, range_max: T, mapped_min: T, mapped_max: T) -> f32 {
		((self - range_min.to_f32()) / (range_max.to_f32() - range_min.to_f32()))
			* (mapped_max.to_f32() - mapped_min.to_f32())
			+ mapped_min.to_f32()
	}
	fn round_i(self) -> isize {
		if (self - (self as isize).to_f32()) > 0.5f32 {
			return self as isize + 1;
		}
		self as isize
	}
	fn fractional_part(self) -> f32 {
		self - (self as usize).to_f32()
	}
	fn mantissa(self) -> isize {
		(self * 10f32.powi(((-self.abs().log10()).ceil()) as i32)) as isize % 10
	}
	fn exponent(self) -> isize {
		if self == 0f32 {
			return 0isize;
		}
		self.abs().log10().floor() as isize
	}
}
