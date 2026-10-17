

use super::{H3,Dimension};


impl<T> H3<T> 
{   
    
    pub fn cast<A>(self) -> H3<A> 
        where T: Into<A>
    {   
        let x: A = self.x.into();
        let y: A = self.y.into();
        let z: A = self.z.into();

        H3{x,y,z}
    }

    pub fn to_array<const I0: char, const I1: char, const I2: char,V>(self) -> [V; 3] 
    where T: Into<V>
    {
        assert!(I0 != I1 && I1 != I2 && I2 != I0, "Expected characters are 'X' | 'Y' | 'Z', in required order.");
        use super::Dimension::*;
        let Self{x,y,z} = self;

        match (Dimension::from(I0),Dimension::from(I1),Dimension::from(I2)){
            (X,Y,Z) => [x.into(),y.into(),z.into()],
            (X,Z,Y) => [x.into(),z.into(),y.into()],

            (Y,X,Z) => [y.into(),x.into(),z.into()],
            (Y,Z,X) => [y.into(),z.into(),x.into()],

            (Z,X,Y) => [z.into(),x.into(),y.into()],
            (Z,Y,X) => [z.into(),y.into(),x.into()],
            _ => unreachable!()
        }
    }

    pub fn map<U, F>(self, mut f: F) -> H3<U>
    where
        F: FnMut(T) -> U,
    {
        H3 {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }

    pub fn as_ref(&self) -> H3<&T> {
        H3 {
            x: &self.x,
            y: &self.y,
            z: &self.z,
        }
    }

    pub fn as_mut(&mut self) -> H3<&mut T> {
        H3 {
            x: &mut self.x,
            y: &mut self.y,
            z: &mut self.z,
        }
    }

}


impl<T> H3<&mut T> {
    pub fn apply<F>(self, mut f: F)
    where F: FnMut(&mut T),
    {
        f(self.x); 
        f(self.y);
        f(self.z);
    }
}


impl<T:Default> Default for H3<T> 
{
    fn default() -> Self {
        H3 {
            x: T::default(),
            y: T::default(),
            z: T::default()
        }
    }
}


impl<T> AsRef<H3<T>> for H3<T> {
    fn as_ref(&self) -> &H3<T> { self }
}

impl<T> AsMut<H3<T>> for H3<T> {
    fn as_mut(&mut self) -> &mut H3<T> { self }
}


