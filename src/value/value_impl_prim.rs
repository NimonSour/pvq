

use std::{ rc::Rc,
    ops::{ Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Neg }
};

use super::H;


macro_rules! impl_value_neg_numeric {
    ($($t:ty),*) => {
        $(
            impl Neg for H<$t> 
            {
                type Output = H<$t>;
                fn neg( mut self) -> Self {
                    let _ = -(&mut self);
                    self
                }
            }
        )*
    };
}

impl_value_neg_numeric!(f32, f64, i8, i16, i32, i64, i128, isize);


macro_rules! impl_value_neg_mut_ref_numeric {
    ($($t:ty),*) => {
        $(
            impl<'a> Neg for &'a mut H<$t> 
            {
                type Output = &'a mut H<$t>;
                fn neg( self) -> Self {
                    self.assign(| total: $t |{ -total });
                    self
                }
            }
        )*
    };
}

impl_value_neg_mut_ref_numeric!(f32, f64, i8, i16, i32, i64, i128, isize);



macro_rules! impl_value_neg_ref_numeric {
    ($($t:ty),*) => {
        $(
            impl Neg for & H<$t> 
            {
                type Output = H<$t>;
                fn neg( self) -> H<$t> {
                    let slf = self.as_ref().rc();
                    H::new_from_fn(move || { - slf.oval() } )
                }
            }
        )*
    };
}

impl_value_neg_ref_numeric!(f32, f64, i8, i16, i32, i64, i128, isize);



// Macro 1: one concrete type, one trait -> one impl
macro_rules! impl_oper_trait {
    ($t:ty, $trait:ident, $method:ident) => {

        // H and ~H  
        impl $trait<H<$t>> for H<$t> {
            type Output = H<$t>;

            fn $method(self, rhs: H<$t>) -> Self::Output {
                self.as_ref().$method(rhs.rc())
            }
        }

        impl $trait<&mut H<$t>> for H<$t> {
            type Output = H<$t>;

            fn $method(self, rhs: &mut H<$t>) -> Self::Output {
                self.as_ref().$method(rhs.rc())
            }
        }

        impl $trait<&H<$t>> for H<$t> {
            type Output = H<$t>;

            fn $method(self, rhs: &H<$t>) -> Self::Output {
                self.as_ref().$method(rhs.rc())
            }
        }     
        // Rc<H> 
        impl $trait<Rc<H<$t>>> for H<$t> {
            type Output = H<$t>;

            fn $method(self, rhs: Rc<H<$t>>) -> Self::Output {
                self.as_ref().$method(rhs)
            }
        }    
        // ---------------------------------------------------

        // &mut H and ~H  
        impl $trait<H<$t>> for &mut H<$t> {
            type Output = H<$t>;

            fn $method(self, rhs: H<$t>) -> Self::Output {
                self.as_ref().$method(rhs.rc())
            }
        }

        impl $trait<&mut H<$t>> for &mut H<$t> {
            type Output = H<$t>;

            fn $method(self, rhs: &mut H<$t>) -> Self::Output {
                self.as_ref().$method(rhs.rc())
            }
        }

        impl $trait<&H<$t>> for &mut H<$t> {
            type Output = H<$t>;

            fn $method(self, rhs: &H<$t>) -> Self::Output {
                self.as_ref().$method(rhs.rc())
            }
        }     
        // Rc<H> 
        impl $trait<Rc<H<$t>>> for &mut H<$t> {
            type Output = H<$t>;

            fn $method(self, rhs: Rc<H<$t>>) -> Self::Output {
                self.as_ref().$method(rhs)
            }
        }  
        // ---------------------------------------------------

        // &H  and ~H 
        impl $trait<H<$t>> for &H<$t> {
            type Output = H<$t>;

            fn $method(self, rhs: H<$t>) -> Self::Output {
                self.$method(rhs.rc())
            }
        }

        impl $trait<&mut H<$t>> for &H<$t> {
            type Output = H<$t>;

            fn $method(self, rhs: &mut H<$t>) -> Self::Output {
                self.$method(rhs.rc())
            }
        }

        impl $trait<&H<$t>> for &H<$t> {
            type Output = H<$t>;

            fn $method(self, rhs: &H<$t>) -> Self::Output {
                self.$method(rhs.rc())
            }
        }     
        // &H and Rc<H> 
        impl $trait<Rc<H<$t>>> for &H<$t> {
            type Output = H<$t>;

            fn $method(self, rhs: Rc<H<$t>>) -> Self::Output {
                let slf = self.rc();
                let func = move || {
                    slf.oval().$method(rhs.oval())
                };
                H::new_from_fn(func)
            }
        } 
        // ---------------------------------------------------

        // Rc<H> and ~H
        impl $trait<H<$t>> for Rc<H<$t>> {
            type Output = H<$t>;

            fn $method(self, rhs: H<$t>) -> Self::Output {
                self.$method(rhs.as_ref())
            }
        }

        impl $trait<&mut H<$t>> for Rc<H<$t>> {
            type Output = H<$t>;

            fn $method(self, rhs: &mut H<$t>) -> Self::Output {
                self.$method(rhs.as_ref())
            }
        }

        impl $trait<&H<$t>> for Rc<H<$t>> {
            type Output = H<$t>;

            fn $method(self, rhs: &H<$t>) -> Self::Output {
                let rhs = rhs.rc();
                let func = move || {
                    self.oval().$method(rhs.oval())
                };
                H::new_from_fn(func)
            }
        }     
        // ---------------------------------------------------

        //  H and prim 

        impl $trait<$t> for H<$t> {
            type Output = H<$t>;

            fn $method(self, rhs: $t) -> Self::Output {
                self.as_ref().$method(rhs)
            }
        }

        impl $trait<$t> for &mut H<$t> {
            type Output = H<$t>;

            fn $method(self, rhs: $t) -> Self::Output {
                self.as_ref().$method(rhs)
            }
        }

        impl $trait<$t> for &H<$t> {
            type Output = H<$t>;

            fn $method(self, rhs: $t) -> Self::Output {

                let slf = self.rc();
                let func = move || {
                    slf.oval().$method(rhs) 
                };
                H::new_from_fn(func)
            }
        }
        // ---------------------------------------------------

        // prim and ~H
        impl $trait<H<$t>> for $t {
            type Output = H<$t>;

            fn $method(self, rhs: H<$t>) -> Self::Output {
                self.$method(rhs.as_ref())
            }
        }

        impl $trait<&mut H<$t>> for $t {
            type Output = H<$t>;

            fn $method(self, rhs: &mut H<$t>) -> Self::Output {
                self.$method(rhs.as_ref())
            }
        }

        impl $trait<&H<$t>> for $t {
            type Output = H<$t>;

            fn $method(self, rhs: &H<$t>) -> Self::Output {

                let rhs = rhs.rc();
                let func = move || {
                    self.$method(rhs.oval()) 
                };
                H::new_from_fn(func)
            }
        }
    };
}



macro_rules! impl_oper_traits_for_type {
    ($($t:ty),* $(,)?) => {
        $(
            impl_oper_trait!($t, Add, add);
            impl_oper_trait!($t, Sub, sub);
            impl_oper_trait!($t, Mul, mul);
            impl_oper_trait!($t, Div, div);
        )*
    };
}

impl_oper_traits_for_type!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);




// ASSIGN

macro_rules! impl_oper_assign_trait {
    ($t:ty, $trait:ident, $method:ident) => {

        // H and= H 
        impl $trait< H<$t> > for H<$t> {
            fn $method(&mut self, rhs: H<$t>) {
                self.$method(rhs.rc())
            }
        }
        // H and= &mut H 
        impl $trait< &mut H<$t> > for H<$t> {
            fn $method(&mut self, rhs: &mut H<$t>) {
                self.$method(rhs.rc())
            }
        }

        // H and= &H 
        impl $trait< &H<$t> > for H<$t> {
            fn $method(&mut self, rhs: &H<$t>) {
                self.$method(rhs.rc())
            }
        }

        // H and= &Rc<H>  (additional)
        impl $trait< &Rc<H<$t>> > for H<$t> {
            fn $method(&mut self, rhs: &Rc<H<$t>>) {
                let rhs = Rc::clone(rhs);
                self.$method(rhs)
            }
        }

        // H and= Rc<H> 
        impl $trait< Rc<H<$t>> > for H<$t> {
            fn $method(&mut self, rhs: Rc<H<$t>>) {
                let func = move |mut total: $t| {
                    total.$method(rhs.oval());
                    total
                };
                self.assign(func);
            }
        }

        // H and= Prim
        impl $trait<$t> for H<$t> {
            fn $method(&mut self, rhs: $t) {

                let func = move |mut total: $t| {
                    total.$method(rhs);
                    total
                };
                self.assign(func);
            }
        }
    };
}

macro_rules! impl_oper_assign_traits_for_type {
    ($($t:ty),* $(,)?) => {
        $(
            impl_oper_assign_trait!($t, AddAssign, add_assign);
            impl_oper_assign_trait!($t, SubAssign, sub_assign);
            impl_oper_assign_trait!($t, MulAssign, mul_assign);
            impl_oper_assign_trait!($t, DivAssign, div_assign);
        )*
    };
}

impl_oper_assign_traits_for_type!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);



// INTO
macro_rules! impl_into_numeric {

    ($($t:ty),*) => {
        $(
            impl Into<$t> for H<$t> {
                fn into(self) -> $t {
                    self.oval()
                }
            }

            impl Into<$t> for &H<$t> {
                fn into(self) -> $t {
                    self.oval()
                }
            }

            impl Into<$t> for &mut H<$t> {
                fn into(self) -> $t {
                    self.oval()
                }
            }
        )*
    };
}
impl_into_numeric!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);




#[cfg(test)]
mod tests{
    use super::*;


    macro_rules! op_matrix_test {
        ($t:ty, $method:ident) => {

            let (x, y): ($t, $t) = (12 as $t, 4 as $t);
            let a = 2 as $t;
            let b = 6 as $t;
            let want = x.$method(y);
            let h = |v: $t| H::new(v);
            let rc = |v: $t| h(v).rc();

            let rc_mem = | a: $t | -> Rc<H<$t>> {

                let mut h =  H::new( a);
                h.assign(move |total:$t| total + a );

                let h_mem = h.as_mem();
                h.assign(move |total:$t| total + a );
                h_mem
            };

            assert_eq!(h(x).$method(h(y)).oval(), want);
            assert_eq!(h(x).$method(&mut h(y)).oval(), want);
            assert_eq!(h(x).$method(&h(y)).oval(), want);
            assert_eq!(h(x).$method(rc(y)).oval(), want);
            assert_eq!(h(x).$method(rc_mem(a)).oval(), want);
            assert_eq!(h(x).$method(y).oval(), want);

            assert_eq!((&mut h(x)).$method(h(y)).oval(), want);
            assert_eq!((&mut h(x)).$method(&mut h(y)).oval(), want);
            assert_eq!((&mut h(x)).$method(&h(y)).oval(), want);
            assert_eq!((&mut h(x)).$method(rc(y)).oval(), want);
            assert_eq!((&mut h(x)).$method(rc_mem(a)).oval(), want);
            assert_eq!((&mut h(x)).$method(y).oval(), want);

            assert_eq!((& h(x)).$method(h(y)).oval(), want);
            assert_eq!((& h(x)).$method(&mut h(y)).oval(), want);
            assert_eq!((& h(x)).$method(&h(y)).oval(), want);
            assert_eq!((& h(x)).$method(rc(y)).oval(), want);
            assert_eq!((& h(x)).$method(rc_mem(a)).oval(), want);
            assert_eq!((& h(x)).$method(y).oval(), want);

            assert_eq!(rc(x).$method(h(y)).oval(), want);
            assert_eq!(rc(x).$method(&mut h(y)).oval(), want);
            assert_eq!(rc(x).$method(&h(y)).oval(), want);
            // assert_eq!(rc(x).$method(rc(y)).oval(), want);
            // assert_eq!(rc(x).$method(y).oval(), want);

            /* mems */
            assert_eq!(rc_mem(b).$method(h(y)).oval(), want);
            assert_eq!(rc_mem(b).$method(&mut h(y)).oval(), want);
            assert_eq!(rc_mem(b).$method(&h(y)).oval(), want);

            assert_eq!(x.$method(h(y)).oval(), want);
            assert_eq!(x.$method(&mut h(y)).oval(), want);
            assert_eq!(x.$method(&h(y)).oval(), want);

        };
    }
    macro_rules! op_test {
        ($name:ident,$t:ty) => {

            #[test]
            fn $name() {
                use std::ops::*;
                op_matrix_test!($t, add);
                op_matrix_test!($t, sub);
                op_matrix_test!($t, mul);
                op_matrix_test!($t, div);
            }
        };
    }
op_test!(test_op_u8,u8);
 op_test!(test_op_u16,u16);
 op_test!(test_op_u32,u32);
 op_test!(test_op_u64,u64);
 op_test!(test_op_u128,u128);
 op_test!(test_op_usize,usize);
 op_test!(test_op_i8,i8);
 op_test!(test_op_i16,i16);
 op_test!(test_op_i32,i32);
 op_test!(test_op_i64,i64);
 op_test!(test_op_i128,i128);
 op_test!(test_op_isize,isize);
 op_test!(test_op_f32,f32);
 op_test!(test_op_f64,f64);





    macro_rules! op_assign_matrix_test {
        ($t:ty, $method:ident, $ass_method:ident) => {

            let (x, y): ($t, $t) = (12 as $t, 4 as $t);
            let a = 2 as $t;

            let want = x.$method(y);
            let h = |v: $t| H::new(v);
            let rc = |v: $t| h(v).rc();

            let rc_mem = | a: $t | -> Rc<H<$t>> {

                let mut h =  H::new( a);
                h.assign(move |total:$t| total + a );

                let h_mem = h.as_mem();
                h.assign(move |total:$t| total + a );
                h_mem
            };

            let mut val = h(x);
            val.$ass_method(h(y));
            assert_eq!(val.oval(), want);

            let mut val = h(x);
            val.$ass_method(&mut h(y));
            assert_eq!(val.oval(), want);

            let mut val = h(x);
            val.$ass_method(&h(y));
            assert_eq!(val.oval(), want);


            let mut val = h(x);
            val.$ass_method(rc(y));
            assert_eq!(val.oval(), want);

            // additional
            let mut val = h(x);
            val.$ass_method(&rc(y));
            assert_eq!(val.oval(), want);

            let mut val = h(x);
            val.$ass_method(rc_mem(a));
            assert_eq!(val.oval(), want);

            // additional
            let mut val = h(x);
            val.$ass_method(&rc_mem(a));
            assert_eq!(val.oval(), want);

            let mut val = h(x);
            val.$ass_method(y);
            assert_eq!(val.oval(), want);
        };
    }


    macro_rules! op_assign_test {
        ($name:ident,$t:ty) => {

            #[test]
            fn $name() {
                use std::ops::*;
                op_assign_matrix_test!($t, add,add_assign);
                op_assign_matrix_test!($t, sub,sub_assign);
                op_assign_matrix_test!($t, mul,mul_assign);
                op_assign_matrix_test!($t, div,div_assign);
            }
        };
    }

op_assign_test!(test_op_ass_u8,u8);
 op_assign_test!(test_op_ass_u16,u16);
 op_assign_test!(test_op_ass_u32,u32);
 op_assign_test!(test_op_ass_u64,u64);
 op_assign_test!(test_op_ass_u128,u128);
 op_assign_test!(test_op_ass_usize,usize);
 op_assign_test!(test_op_ass_i8,i8);
 op_assign_test!(test_op_ass_i16,i16);
 op_assign_test!(test_op_ass_i32,i32);
 op_assign_test!(test_op_ass_i64,i64);
 op_assign_test!(test_op_ass_i128,i128);
 op_assign_test!(test_op_ass_isize,isize);
 op_assign_test!(test_op_ass_f32,f32);
 op_assign_test!(test_op_ass_f64,f64);


    // #[test]
    // fn test_add(){
    //     let a = H::new(1);
    //     let b = H::new(1);

    //     let c = b.rc();
    //     let c1 = a.rc();

    //     // works
    //     // H + Rc
    //     // let d = a + c;  

    //     // Rc + H
    //     // let d = c + a; 

    //     // H + prim
    //     // let d = b + 3; 

    //     // prim + H
    //     // let d = 3 + b; 

    //     //----------------------------------------------------------------

    //     // doesn't work

    //     // prim + Rc
    //     // let d = 3 + c;

    //     // Rc + prim
    //     // let d = c + 3; 

    //     // Rc + Rc
    //     // let d = c + c1;
    //     // let d = c.to_owned() + c1; // works 
    //     // let d = c + c1.to_owned(); // works

    //     assert


    // }
}
