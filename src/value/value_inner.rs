
use std::{cell::Cell, rc::Rc };
use super::H;
use crate::HiD;


enum InitState<T> {
    CallBack(Box<dyn FnMut () -> T>),
    Branch(Vec<H<T>>)
}

impl<T> InitState<T> 
where T: 'static,
{

    pub fn new_call_back<F>(func: F) -> Self
    where
        F: FnMut () -> T + 'static,
    {   
        InitState::CallBack(Box::new(func))
    }

    fn push(&mut self, h_mem: H<T>){
        if let Self::Branch(v) = self {
            v.push(h_mem);
        } else {
            panic!("InitState::push: Expected Branch variant.") 
        }
    }

    fn get_last_mem(&self) -> Option<Rc<H<T>>> {
        match self{
            Self::CallBack(_) => None,
            Self::Branch(v) => {
                if let Some( last ) = v.last() {
                    // these H's will never mutated their value
                    Some(last.rc())
                } else {
                    panic!("InitState::get_last_mem: Unexpected empty branch container.");
                }
            }
        }
    }
    fn get_branch(&self) -> Option<&Vec<H<T>>> {
        if let Self::Branch(v) = self {
            Some(v)
        } else { None }
    }

    fn get_branch_mut(&mut self) -> Option<&mut Vec<H<T>>> {
        if let Self::Branch(v) = self {
            Some(v)
        } else { None }
    }

    fn contains(&self, name: &String ) -> bool {
        if let Some(branch) = self.get_branch(){
            for mem in branch {
                if let Some(mem_name) = mem.get_name(){
                    if mem_name.eq(name){
                        return true;
                    }
                }
            }
        } 
        false
    }

    fn get_mem_by_id(&self, hid: HiD) -> Option<Rc<H<T>>> {
        if let Some(branch) = self.get_branch(){
            if let Some(h) = branch.iter().find(|&x| x.get_id() == hid){
                return Some(h.rc());
            }
        }
        None 
    }

    fn get_mem_by_name(&self, name: &String) -> Option<Rc<H<T>>> {
        if let Some(branch) = self.get_branch(){
            if let Some(h) = branch.iter().find(|&x| { 
                if let Some(mem_name) = x.get_name(){
                    mem_name.eq(name)
                } else { false }
            }){ return Some(h.rc());}
        } 
        None 
    }
}


pub (super) struct ViH<T> {
    id:  HiD, 
    name: Option<String>,
    eval: Cell<bool>, 
    init: InitState<T>,
    assigns: Vec<Box<dyn Fn(T) -> T>>,
    h: Option<Rc<H<T>>>,
}

impl<T:'static> ViH<T>{


    pub fn new_driver<F>( func: F, hid: Option<HiD>, name: Option<String>) -> Self
    where
        F: FnMut() -> T + 'static,
    {
        let id = if let Some(id) = hid { id } else { Self::new_id() };
        Self{ 
            id,
            name, 
            eval: Cell::new(false), 
            init: InitState::new_call_back(func), 
            assigns: Vec::new(),
            h: None,
        }
    }

    pub fn set_ival <F>(&mut self, func: F)
    where
        F: FnMut() -> T + 'static,
    {
        if let Some(branch) = self.init.get_branch_mut(){
            branch[0].ival_fn(func);
        } else {
            let new_init = InitState::new_call_back(Box::new(func));
            self.init = new_init;
        }
    }

    pub(super) fn set_h(&mut self, h: Rc<H<T>> ){
        if self.h.is_none() {
            self.h = Some(h);
        }
    }

    pub fn next_mem(&mut self, name: Option<String>) -> Rc<H<T>>{
        let mut id =  self.id.new_formulae();

        let eval = Cell::new(false);
    
        if let Some(name) = name.as_ref(){
            if self.init.contains(name){
                let msg = format!("ViH::next_mem: mem name `{name}` - not unique");
                panic!("{msg}");
            }
        }

        let init = if let Some(last_mem) = self.init.get_last_mem(){
            if self.assigns.len() == 0 { 
                if name.is_none(){
                    return last_mem; 
                }
            }

            id = last_mem.get_id().new_formulae();
            InitState::new_call_back(move || -> T { last_mem.oval() })
        } else {
            std::mem::replace(&mut self.init,  InitState::Branch(Vec::new()))
        };

        let assigns = std::mem::take(&mut self.assigns);
    
        // these mems (Hs) will never be mutated
        let h_mem = H::from_inner(Self {id, name, eval, init, assigns, h: self.h.clone()}); 
    
        self.init.push(h_mem);

        self.init.get_last_mem().unwrap()

    }

    pub fn assign<F>(&mut self, func: F )
    where
        F: Fn(T) -> T + 'static,
    {   
        self.assigns.push( Box::new(func) );
    }

    pub fn get_value(&mut self) -> T {
        use InitState::*;

        if self.eval.get() {
            panic!("H::value -> HInner::get_value:💥 cyclic dependency detected — a H's formula depends on itself.");
        }
        self.eval.set(true);

        let mut total = match &mut self.init {
            CallBack(func)=> (*func)(),
            Branch(branch) => {
                if branch.is_empty() { panic!("Unexpected empty branch!")} 
                if let Some(last_fork) = branch.last(){
                    last_fork.oval()
                } else {
                    panic!("ViH::get_value: Unexpected fork branch is empty.")
                }
            }
        };

        for func in &self.assigns {
            total = (*func)(total)
        }
        self.eval.set(false);
        total
    }



    pub fn get_id(&self) -> HiD { self.id }

    pub fn h(&self) -> Rc<H<T>> {
        if let Some( h) = self.h.as_ref() {
            Rc::clone(h)
        } else {
            panic!("ViH::h: unexpected None `h`");
        }
    }

    pub fn get_name(&self) -> Option<String> { self.name.clone() }
    
    pub fn new_id() -> HiD { HiD::new::<T>() }
}


#[poly_type::in_impl]
impl<T:'static> ViH<T> { 

    #[poly_type::arg(type MemIndex{ &'a str | &'a String | String | HiD })]
    pub fn get_mem<'a>(&self, index: _ ) -> Result<Rc<H<T>>, &'static str> {
 

        let name = match index {
            index::< &'a str >    => { index.to_string() },
            index::< &'a String > => { index.to_owned() },
            index::< String >     => { index },
            index::< HiD >        => { 

                if !self.get_id().eq_inst(&index){ return Err("unknown instace");}

                return if let Some(v) = self.init.get_mem_by_id(index){
                    Ok(v.rc())
                } else {
                    Err("mem not found")
                };
            },
        };

        if name.is_empty() { return Err("indexing by empty string"); }

        if let Some(v) = self.init.get_mem_by_name(&name){
            Ok(v.rc())
        } else {
            Err("mem not found")
        }

    }
}