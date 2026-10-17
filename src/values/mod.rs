mod h3_impl;
mod h3_impl_h;
mod h3_impl_std_prim;


#[derive(Clone,Copy,Debug)]
pub enum Dimension{ X,Y,Z }


impl From<char> for Dimension {

    fn from(value: char) -> Self {
        match value {
            'X'|
            'x' => Self::X,
            'Y'|
            'y' => Self::Y,
            'Z'|
            'z' => Self::Z,
            _ => panic!("Dimension::from<char>: Unknown character.")
        }
    }
}

#[derive(Clone,Copy,Debug)]
pub struct H3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

