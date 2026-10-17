
use std::{ cell::RefCell, rc::{Weak,Rc} };
use super::{ HpR, HdpR, ViH, H, value_inner::MemIndex };
use crate::{HiD, value::Pointer};



impl<T> H <T>
where T: Clone  + 'static,
{   
    pub fn new( value: T ) -> Self {
        Self::new_with_name(value,"")
    }

    pub fn new_with_name<S:ToString>( value: T , name: S) -> Self {
        let name = Self::util_check_name(name);
        let func = move || -> T { value.clone() };
        Self::from_inner(ViH::<T>::new_driver(func,None,name) )
    }

    pub fn ival(&mut self, value: T ){
        let func = move || -> T { value.clone() };
        self.inner.borrow_mut().set_ival(func)
    }
}

impl<T:'static> H <T>{   

    pub fn new_from_fn<F>( func: F) -> Self 
    where  
        F: FnMut() -> T + 'static,
    {   
        Self::new_from_fn_with_name(func,"")
    }

    pub fn new_from_fn_with_name<F,S:ToString>( func: F, name: S) -> Self 
    where  
        F: FnMut() -> T + 'static,
    {   
        let name = Self::util_check_name(name);
        Self::from_inner(ViH::new_driver(func, None, name ))
    }

    pub (super) fn from_inner(value_inner: ViH<T>) -> Self 
    {
        let inner = Rc::new(RefCell::new(value_inner));
        let own_inner = Rc::clone(&inner);

        let rc = Rc::new_cyclic(|wk| {
            H { pointer: Pointer::Weak(Weak::clone(wk)), inner }
        });

        own_inner.borrow_mut().set_h(Rc::clone(&rc));
        Self{ pointer: Pointer::Strong(rc), inner: own_inner }
    }

    pub fn ival_fn<F>(&mut self, func: F)
    where  
        F: FnMut() -> T + 'static,
    {
        self.inner.borrow_mut().set_ival(func)
    }

    fn util_check_name<S:ToString>(s:S) -> Option<String> {
        let s = s.to_string();
        if s.is_empty(){ None } else { Some(s)}
    }

    pub fn to_own(self: Rc<Self>) -> Self {
        Self::new_from_fn(move || { self.oval() })
    }

    /// Returns a reference counted pointer to H
    pub fn as_rch(&self) -> Rc<Self> {
        // mem's return H rc
        if self.is_mem(){
            self.inner.borrow().h()
        } else {
            self.rc()
        }
    }

    pub fn rc(&self) -> Rc<Self> {
        self.pointer.rc()
    }

    pub fn oval(&self) -> T {
        self.inner.borrow_mut().get_value()
    }

    pub fn get_id(&self) -> HiD {
        self.inner.borrow().get_id()
    }

    // keep this method '&self' so the refs could use it , it's not mutating the value just reshaping value container
    pub fn as_mem(&self) -> Rc<Self> {
        if self.is_mem(){
            self.as_rch().as_mem()
        } else {
            self.inner.borrow_mut().next_mem(None)
        }
    }
   
    pub fn get_name(&self) -> Option<String> {
        self.inner.borrow().get_name().clone()
    }

    pub fn is_mem(&self) -> bool {
        self.get_id().formulae() > 0
    }

    pub fn heq<A:AsRef<Self>>(&self, other: A) -> bool{
        self.get_id().eq_inst(&other.as_ref().get_id())
    }

    pub fn assign<F>(&mut self, func: F )
    where
        F: Fn(T) -> T + 'static,
    {   
        self.inner.borrow_mut().assign(func);
    }
    
}



#[poly_type::in_impl]
impl<T:'static> H <T>{   

    #[poly_type::arg(type MemInput { () | &'a str | String | HpR<T> | 
                                     HdpR | Vec<HpR<T>> | Vec<HdpR> | (Vec<HpR<T>>, Vec<HdpR>) | (Vec<HdpR>, Vec<HpR<T>>) |
                                     (&'a str,HpR<T>) | (&'a str,HdpR)| (String,HpR<T>)| (String,HdpR) | 
                                     (&'a str,Vec<HpR<T>>) | (&'a str,Vec<HdpR>) | (&'a str, Vec<HpR<T>>, Vec<HdpR>) | (&'a str, Vec<HdpR>, Vec<HpR<T>>) | 
                                     (String,Vec<HpR<T>>) | (String,Vec<HdpR>) | (String, Vec<HpR<T>>, Vec<HdpR>) | (String, Vec<HdpR>, Vec<HpR<T>>) 

                                    }
    )]

    pub fn mem<'a>(&self, args: _ ) -> Rc<H<T>> {

        if self.is_mem(){
            return self.as_rch().mem(args);
        }

        let mut name: Option<String> = None;
        let mut props: Vec<HpR<T>> = Vec::new();
        let mut hid_props: Vec<HdpR> = Vec::new();

        match args {
            _args::<()> => (),
            args::<&'a str> => { name = Some(args.to_string()); },
            args::<String> => { name = Some(args); },
            args::<HpR<T>> => { props = vec![args]; },
            args::<HdpR> => { hid_props = vec![args]; },

            args::<Vec<HpR<T>>> => { props = args; },
            args::<Vec<HdpR>> => { hid_props = args; },
            args::<(Vec<HpR<T>>, Vec<HdpR>)> => { let (pr,h_pr) = args; props = pr; hid_props = h_pr; },
            args::<(Vec<HdpR>, Vec<HpR<T>>)> => { let (h_pr, pr) = args; props = pr; hid_props = h_pr; },

            args::<(&'a str,HpR<T>)> => { let (s,p) = args; name = Some(s.to_string()); props = vec![p]; },
            args::<(&'a str,HdpR)> => { let (s,p) = args; name = Some(s.to_string()); hid_props = vec![p]; },
            args::<(String,HpR<T>)> => { let (s,p) = args; name = Some(s); props = vec![p]; },
            args::<(String,HdpR)> => { let (s,p) = args; name = Some(s); hid_props = vec![p]; },

            args::<(&'a str,Vec<HpR<T>>)> => { let (s,pr) = args; name = Some(s.to_string()); props = pr; },
            args::<(&'a str,Vec<HdpR>)> => { let (s,pr) = args; name = Some(s.to_string()); hid_props = pr; },
            args::<(&'a str, Vec<HpR<T>>, Vec<HdpR>)> => { let (s,pr,h_pr) = args; name = Some(s.to_string()); props = pr; hid_props = h_pr; },
            args::<(&'a str, Vec<HdpR>, Vec<HpR<T>>)> => { let (s,h_pr,pr) = args; name = Some(s.to_string()); props = pr; hid_props = h_pr; },

            args::<(String,Vec<HpR<T>>)> => { let (s,pr) = args; name = Some(s); props = pr; },
            args::<(String,Vec<HdpR>)> => { let (s,pr) = args; name = Some(s); hid_props = pr; },
            args::<(String, Vec<HpR<T>>, Vec<HdpR>)> => { let (s,pr,h_pr) = args; name = Some(s); props = pr; hid_props = h_pr; },
            args::<(String, Vec<HdpR>, Vec<HpR<T>>)> => { let (s,h_pr,pr) = args; name = Some(s); props = pr; hid_props = h_pr; },
        
        }
        
        let h_mem = self.inner.borrow_mut().next_mem(name);

        let mem_id = h_mem.get_id();

        for h_pr in hid_props{
            let HdpR(mut h) = h_pr;
            // check 
            if !h.oval().eq_inst(&mem_id) {
                panic!("HdpR failed type and instance equality");
            }

            h.ival(mem_id.clone());
        }

        for pr in props {
            let HpR(mut pr) = pr;
            pr.ival(h_mem.clone());
        }

        h_mem
    }

    pub fn get_mem<'a>(&self, index: impl Into<MemIndex<'a>> ) -> Rc<H<T>> { 
        if self.is_mem(){
            self.as_rch().get_mem(index)
        } else {
            match self.inner.borrow().get_mem(index){
                Ok(mem) => mem,
                Err(msg) => panic!("{msg}"),
            }
        }
    }

    pub fn from_mem_promise<A:AsRef<H<T>>>(h: A, mem_name: &'static str ) -> Self {
        Self::from_mem_promise_with_name(h,mem_name,"")
    }

    pub fn from_mem_promise_with_name<S:ToString, A:AsRef<H<T>>>( h: A, mem_name: &'static str, name: S) -> Self {
        let h = h.as_ref().as_rch();

        let mut mem = None;
        let func = move || -> T {
            match &mem {
                None => { 
                    let m = h.get_mem(mem_name);
                    let val = m.oval();
                    mem = Some(m);
                    val
                },
                Some( mem ) => {mem.oval()}
            }
        };

        Self::new_from_fn_with_name(func,name)

    }
}


impl<T> H<T> 
where T: Default + Clone + 'static
{
    pub fn new_from_promise() -> (Self,HpR<T>) {
        Self::new_from_promise_with_name("")
    }

    pub fn new_from_promise_with_name<S: ToString>(name:S) -> (Self,HpR<T>){

        let slf = H::new(T::default());

        let hpr = H::new(slf.as_rch());
        let prom = hpr.as_rch();
        let mut mem  = None;

        let func = move || -> T {
            match &mem {
                None => { 
                    let m = prom.oval();
                    let val = m.oval();
                    mem = Some(m);
                    val
                },
                Some( mem ) => {mem.oval()}
            }
        };

        let name  = Self::util_check_name(name);
        let inner = ViH::new_driver(func,None, name);
        let _ = slf.inner.replace(inner);
        (slf,HpR(hpr))
    }

    pub fn new_from_hid_promise<A:AsRef<H<T>>>( h: A ) -> (Self, HdpR){
        Self::new_from_hid_promise_with_name(h,"")
    }

    pub fn new_from_hid_promise_with_name<A:AsRef<H<T>>,S:ToString>( h: A, name: S ) -> (Self, HdpR){

        let source = h.as_ref().as_rch();

        let prom = H::<HiD>::new(source.get_id());
        let prom_rch = prom.as_rch();

        let mut source_mem = None;
        let func = move || -> T {
            match &source_mem {
                None => {
                    let mem_hid = prom_rch.oval();
                    let mem = source.get_mem(mem_hid);
                    let val = mem.oval();
                    source_mem = Some(mem);
                    val
                },
                Some(src_mem) => { 
                    src_mem.oval()
                },
            }
        };
        
        let name  = Self::util_check_name(name);
        let inner = ViH::new_driver(func,None, name);

        ( Self::from_inner(inner) ,HdpR(prom))
    }

}



impl<T> Default for H<T> 
where T: Default + 'static, 
{
    fn default() -> Self {
        Self::new_from_fn(|| { T::default() })
    }
}

impl<T> AsRef<H<T>> for H<T> {
    fn as_ref(&self) -> &H<T> { self }
}

impl<T> AsMut<H<T>> for H<T> {
    fn as_mut(&mut self) -> &mut H<T> { self }
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_as_mem() {

        let mut h = H::new(3u32);

        h += 1;

        let mem_1 = h.as_mem();

        h += 1;

        let mem_2 = h.as_rch().as_mem();

        h += 1;

        // mem H via a mem instance 
        let mem_3 = mem_2.as_mem();

        h += 1;

        assert_eq!(4,mem_1.oval());
        assert_eq!(5,mem_2.oval());
        assert_eq!(6,mem_3.oval());
        assert_eq!(7,h.oval());

    }

    #[test]
    fn test_mem() {

        let mut h = H::new(3u32);
        let first = "first";
        let second = "second";
        let third = "third";

        h += 1;

        let mem_1 = h.mem(first);
        
        h += 1;

        let mem_2 = h.mem(second);

        h += 1;

        // mem H via a mem instance 
        let mem_3 = mem_2.mem(third);

        h += 1;

        assert_eq!(4,mem_1.oval());
        assert_eq!(5,mem_2.oval());
        assert_eq!(6,mem_3.oval());
        assert_eq!(7,h.oval());

        // get mem from H
        let eme_1 = h.get_mem(first);
        assert!( mem_1.get_id().eq(&eme_1.get_id()));

        // get mem from Rc<H>
        let eme_2 = h.as_rch().get_mem(second);
        assert!( mem_2.get_id().eq(&eme_2.get_id()));

        // get mem from Rc<H>(mem)
        let eme_3 = mem_3.get_mem(third);
        assert!( mem_3.get_id().eq(&eme_3.get_id()));

    }

    #[test]
    fn test_from_promise() {
        let first = "first";

        let mut h = H::new(3u32);
        let mut prom_h = H::from_mem_promise(&h,first);

        h += 1;

        let _ = h.mem(first);

        h += 1;
        prom_h -=2;

        assert_eq!(2,prom_h.oval());
        assert_eq!(5,h.oval());

    }

    #[test]
    fn test_new_from_promise() {

        let mut h = H::new(3u32);
        let (mut q,prom) = H::<u32>::new_from_promise();

        h += 1;

        let _ = h.mem(prom);

        q += 1;

        assert_eq!(5,q.oval());
        assert_eq!(4,h.oval());
    }


    #[test]
    fn test_new_from_hid_promise() {

        let mut h = H::new(3u32);
        let (mut q,prom) = H::new_from_hid_promise(&h);

        h += 1;

        let _ = h.mem(prom);

        q += 1;

        assert_eq!(5,q.oval());
        assert_eq!(4,h.oval());
    }

}

