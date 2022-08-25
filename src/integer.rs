use num::Num;

pub(crate) mod i12 {
    pub const MAX: i16 = 2047;
    pub const MIN: i16 = -2048;
}

pub(crate) trait AsUnsigned<N: Num + PartialOrd> {
    fn as_unsigned(self) -> N;
}

pub trait AsIndex {
    fn as_index(&self) -> usize;
}

impl AsIndex for u32 {
    /// Convert this u32 to a usize
    ///
    /// Panics if your u32 is bigger than a usize, e.g if you are on a 16-bit machine and ask
    /// for a number > 2^16.
    fn as_index(&self) -> usize {
        usize::try_from(*self).expect("u32 is bigger than usize!")
    }
}

macro_rules! impl_unsigned {
    ($signed:ty, $unsigned:ty) => {
        impl AsUnsigned<$unsigned> for $signed {
            fn as_unsigned(self) -> $unsigned {
                self as $unsigned
            }
        }
        impl AsUnsigned<$unsigned> for $unsigned {
            fn as_unsigned(self) -> $unsigned {
                self as $unsigned
            }
        }
    };
}

impl_unsigned!(i8, u8);
impl_unsigned!(i16, u16);
impl_unsigned!(i32, u32);
impl_unsigned!(i64, u64);
impl_unsigned!(i128, u128);
