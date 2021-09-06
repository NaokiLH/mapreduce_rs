use std::error::Error;
pub trait Maper {
    fn map(&self) -> Result<(), Box<dyn Error>>;
    fn recv(&self) -> Result<(), Box<dyn Error>>;
    fn send(&self, id: u32, maped_file: Vec<(&str, i32)>) -> Result<(), Box<dyn Error>>;
}

pub trait Reducer {
    fn reduce(&self) -> Result<(), Box<dyn Error>>;
    fn recv(&self) -> Result<(), Box<dyn Error>>;
}
