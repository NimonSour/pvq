

use std::{any::TypeId, cell::RefCell, collections::HashMap, fmt, sync::{Mutex, OnceLock}};

use std::marker::PhantomData;

/// H will have the same id for the entire lifetime
#[derive(Clone,Copy,Hash)]
pub struct HiD {

    type_id: TypeId,

    // is a key of instance HashMap<usize,Vec<HashSet<ValueInnerId>>
    instance: usize,
    

    // increments for each new "mem", 0 is main 
    formulae: usize,

    // force !Sync, !Send
    _mark: PhantomData<*const ()>
}


static TYPE_NAMES: OnceLock<Mutex<HashMap<TypeId, String>>> = OnceLock::new();



impl HiD {

    pub fn default<T:'static>() -> Self {
        HiD {
            type_id: TypeId::of::<T>(),
            instance: 0,
            formulae:0,
            _mark: PhantomData 
        }
    }

    pub fn new<T:'static>() -> Self {
        thread_local! {
            // Each thread gets its own completely separate, HashMap
            static SUB_TY_COUNT: RefCell<HashMap<TypeId, usize>> = RefCell::new(HashMap::new());
        }

        let type_id = TypeId::of::<T>();


        let instance = SUB_TY_COUNT.with_borrow_mut(|m|{
            let count = m.entry(type_id).or_insert(0);
            *count +=1;
            // instance "0" is kept for 'default'!
            count.clone()
        });
        Self::register_type_name::<T>();
        HiD { type_id, instance, formulae: 0, _mark: PhantomData}
    }

    pub fn register_type_name<T:'static>() {
        let mut map = TYPE_NAMES.get_or_init(|| Mutex::new(HashMap::new())).lock().unwrap();
        map.entry(TypeId::of::<T>())
            .or_insert_with(|| std::any::type_name::<T>().to_string());
    }

    fn get_type_name(&self) -> String {
        TYPE_NAMES.get_or_init(|| Mutex::new(HashMap::new()))
            .lock()
            .unwrap()
            .get(&self.type_id)
            .cloned()
            .unwrap_or_else(|| "?".to_string())
    }

    pub fn new_formulae(&self) -> Self {
        let Self{ type_id,instance,formulae,_mark} = *self;
        
        Self {
            type_id,
            instance,
            formulae: formulae +1,
            _mark
        }
       
    }

    pub fn type_id(&self) -> TypeId { self.type_id }
    pub fn instance(&self)-> usize { self.instance }   
    pub fn formulae(&self)-> usize { self.formulae }

    pub fn eq_inst(&self, other: &Self) -> bool {
        self.type_id == other.type_id &&
        self.instance == other.instance
    }

    pub fn eq_form(&self, other: &Self) -> bool {
        self.type_id == other.type_id &&
        self.instance == other.instance &&
        self.formulae == other.formulae
    }

    pub fn is_any_eq<T: AsRef<HiD>,I: IntoIterator<Item=T>>(&self, iter: I) -> bool {
        iter.into_iter().any(|x| self.eq(x.as_ref()))
    }

}


impl AsRef<HiD> for HiD {
    fn as_ref(&self) -> &HiD { self }
}

impl AsMut<HiD> for HiD {
    fn as_mut(&mut self) -> &mut HiD { self }
}

impl PartialEq for HiD {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id && 
        self.instance == other.instance &&
        self.formulae == other.formulae 
    }
}

impl Eq for HiD {}



fn box_name_string(path_name: String, width: usize, bord_char: char )->String {
    let width = width -2;
    let chunks: Vec<String> = path_name
        .chars()
        .collect::<Vec<char>>()
        .chunks(width)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect();

    let mut s = String::new();
    for chunk in chunks {
        s += &format!("{b} {:<width$} {b}\n",chunk, b = bord_char);
    }
    
    s
}

impl fmt::Display for HiD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const NAME_W: usize = 8;
        const VALUE_W: usize = 12;

        const CONTENT_W: usize = 1+NAME_W + 3 + VALUE_W+1; 

        let type_name = box_name_string(self.get_type_name(), CONTENT_W,'|');

        let border = format!("+{}+", "-".repeat(CONTENT_W));


        writeln!(f, "\n {:^CONTENT_W$}", "HiD")?;
        writeln!(f, "{border}")?;
        write!(f, "{type_name}")?; 
        writeln!(f, "{border}")?;
        writeln!(f, "| {:<NAME_W$} : {:>VALUE_W$} |", "instance", self.instance)?;
        writeln!(f, "{border}")?;
        writeln!(f, "| {:<NAME_W$} : {:>VALUE_W$} |", "formulae", self.formulae)?;
        write!(f, "{border}\n")
    }
}

