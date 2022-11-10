use momoden_password::BoundedU8;

pub(crate) trait BoolExt {
    fn toggle(&mut self);
}

impl BoolExt for bool {
    fn toggle(&mut self) {
        *self = !*self;
    }
}

pub(crate) trait VecExt {
    unsafe fn pop_unchecked(&mut self);
}

impl<T> VecExt for Vec<T> {
    unsafe fn pop_unchecked(&mut self) {
        self.set_len(self.len() - 1);
    }
}

pub(crate) trait NewClampExt<T> {
    /// x を自身の型の値域で clamp したものを返す。
    fn new_clamp(x: T) -> Self;
}

macro_rules! impl_clamp_ext_primitive {
    (for $ty:ty, $($ty_arg:ty)*) => {
        $(
            impl NewClampExt<$ty_arg> for $ty {
                fn new_clamp(x: $ty_arg) -> Self {
                    if x < <$ty_arg>::from(Self::MIN) {
                        Self::MIN
                    } else if x > <$ty_arg>::from(Self::MAX) {
                        Self::MAX
                    } else {
                        x as Self
                    }
                }
            }
        )*
    };
}

impl_clamp_ext_primitive!(for u8, i16 i32 i64 i128 isize u16 u32 u64 u128 usize);
impl_clamp_ext_primitive!(for u16, i32 i64 i128 u32 u64 u128 usize);

macro_rules! impl_clamp_ext_bounded_u8 {
    ($($ty_arg:ty)*) => {
        $(
            impl<const MIN: u8, const MAX: u8> NewClampExt<$ty_arg> for BoundedU8<MIN, MAX> {
                fn new_clamp(x: $ty_arg) -> Self {
                    if x < <$ty_arg>::from(Self::MIN) {
                        Self::MIN
                    } else if x > <$ty_arg>::from(Self::MAX) {
                        Self::MAX
                    } else {
                        unsafe { Self::new_unchecked(x as u8) }
                    }
                }
            }
        )*
    };
}

impl_clamp_ext_bounded_u8!(i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
