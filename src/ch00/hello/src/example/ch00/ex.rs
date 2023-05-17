type Integers = (i8, i16, i32, i64, i128, isize);

pub fn get_integers() -> Integers {
  (1, 2, 3, 4, 5, 6)
}

type UnsignedIntegers = (u8, u16, u32, u64, u128, usize);

pub fn get_unsigned_integers() -> UnsignedIntegers {
  (7, 8, 9, 10, 11, 12)
}
