use std::rc::Rc;

use super::H3;
use crate::{H, HiD};


impl<T:'static> H3<H<T>> {

    pub fn ovals(&self) -> H3<T> {
        self.as_ref().map(|v| v.oval())
    }
    pub fn get_ids(&self) -> H3<HiD> {
        self.as_ref().map(|v| v.get_id())
    }
    pub fn as_mems(&self) -> H3<Rc<H<T>>> {
        self.as_ref().map(|v| v.as_mem())
    }
    pub fn as_rchs(&self) -> H3<Rc<H<T>>> {
        self.as_ref().map(|v| v.as_rch())
    }
    pub fn rcs(&self) -> H3<Rc<H<T>>> {
        self.as_ref().map(|v| v.rc())
    }


    pub fn assigns<F>(&mut self, func: F)
    where F: Fn( H3<T> ) -> H3<T> + 'static,
    {

        let mem_slf = self.as_mems();
        let driver = move|| -> H3<T> { mem_slf.ovals()};
        let mut h = H::new_from_fn(driver);
        h.assign(func);
        let (hx,hy,hz) = (h.rc(),h.rc(),h.rc());

        // we ignore the 'total' input and return corresponding (coord) output from 'h'
        // this may work differently if the "assign" allows FnMut 
        // the 'oval' for 'h' will be called 3 times ( by every coordinate ) 
        let func_x = move |_: T| -> T { let H3{ x,..} = hx.oval(); x };
        let func_y = move |_: T| -> T { let H3{ y,..} = hy.oval(); y };
        let func_z = move |_: T| -> T { let H3{ z,..} = hz.oval(); z };
        self.x.assign(func_x);
        self.y.assign(func_y);
        self.z.assign(func_z);

        // additional isolate 
        let _ = self.as_mems();
    }

    // TODO : Find a way to "mem", "as_mem", ~`promise` 
}

impl<T:'static> H3<Rc<H<T>>> {


    pub fn to_owns(self) -> H3<H<T>> {

        H3 {
            x: self.x.to_own(),
            y: self.y.to_own(),
            z: self.z.to_own(),
        }
    }

    pub fn ovals(&self) -> H3<T> {
        self.as_ref().map(|v| v.oval())
    }
}
