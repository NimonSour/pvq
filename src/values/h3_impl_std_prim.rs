use std::ops::{ Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign };


use super::H3;


macro_rules! impl_binop {
    ($trait:ident, $method:ident) => {
        impl<T, Rhs, Out> $trait<H3<Rhs>> for H3<T>
        where
            T: $trait<Rhs, Output = Out>,
        {
            type Output = H3<Out>;
            fn $method(self, rhs: H3<Rhs>) -> H3<Out> {
                H3 {
                    x: self.x.$method(rhs.x),
                    y: self.y.$method(rhs.y),
                    z: self.z.$method(rhs.z),
                }
            }
        }
    };
}


impl_binop!(Add, add);
impl_binop!(Sub, sub);
impl_binop!(Mul, mul);
impl_binop!(Div, div);



macro_rules! impl_assign_op {
    ($trait:ident, $method:ident) => {
        impl<T, Rhs> $trait<H3<Rhs>> for H3<T>
        where
            T: $trait<Rhs>,
        {
            fn $method(&mut self, rhs: H3<Rhs>) {
                self.x.$method(rhs.x);
                self.y.$method(rhs.y);
                self.z.$method(rhs.z);
            }
        }
    };
}

impl_assign_op!(AddAssign, add_assign);
impl_assign_op!(SubAssign, sub_assign);
impl_assign_op!(MulAssign, mul_assign);
impl_assign_op!(DivAssign, div_assign);





macro_rules! impl_scalar_for_h3 {
    ($t:ty, $trait:ident, $method:ident) => {
        impl $trait<$t> for H3<$t> {
            type Output = H3<$t>;
            fn $method(self, rhs: $t) -> H3<$t> {
                H3 {
                    x: self.x.$method(rhs),
                    y: self.y.$method(rhs),
                    z: self.z.$method(rhs),
                }
            }
        }
    };
}

macro_rules! impl_h3_scalar_ops_for_types {
    ($($t:ty),* $(,)?) => {
        $(
            impl_scalar_for_h3!($t, Add, add);
            impl_scalar_for_h3!($t, Sub, sub);
            impl_scalar_for_h3!($t, Mul, mul);
            impl_scalar_for_h3!($t, Div, div);
        )*
    };
}
impl_h3_scalar_ops_for_types!(u8, u16, u32, u64, u128, usize,i8, i16, i32, i64, i128, isize,f32, f64 );




macro_rules! impl_assign_scalar_for_h3 {
    ($t:ty, $trait:ident, $method:ident) => {
        impl $trait<$t> for H3<$t> {
            fn $method(&mut self, rhs: $t) {
                self.x.$method(rhs);
                self.y.$method(rhs);
                self.z.$method(rhs);
            }
        }
    };
}
 
macro_rules! impl_h3_scalar_assign_for_types {
    ($($t:ty),* $(,)?) => {
        $(
            impl_assign_scalar_for_h3!($t, AddAssign, add_assign);
            impl_assign_scalar_for_h3!($t, SubAssign, sub_assign);
            impl_assign_scalar_for_h3!($t, MulAssign, mul_assign);
            impl_assign_scalar_for_h3!($t, DivAssign, div_assign);
        )*
    };
}
 
impl_h3_scalar_assign_for_types!(u8, u16, u32, u64, u128, usize,i8, i16, i32, i64, i128, isize,f32, f64);
 


