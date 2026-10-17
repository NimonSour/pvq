mod value_inner;
mod value_impl;
mod value_impl_prim;

use std::{ cell::RefCell, rc::{Rc, Weak}};
use crate::HiD;
use value_inner::ViH;


/*
                                   
[input]-------+----+----* H
              |    |     \   
              |   mem2    \
             mem1         ref 
*/
pub struct HpR<T>(pub H<Rc<H<T>>>);
pub struct HdpR(pub H<HiD>);

enum Pointer<T>{
    Weak(Weak<T>),
    Strong(Rc<T>),
}

impl<T> Pointer <T> {

    pub fn rc(&self) -> Rc<T> {
        use Pointer::*;
        match self {
            Weak(wk) => {wk.upgrade().unwrap()},
            Strong(rc) => {Rc::clone(rc)},
        }
    }
}


// this Object is ever clone 
pub struct H<T> {
    pointer: Pointer<Self>,
    inner: Rc<RefCell<ViH<T>>>,
}
