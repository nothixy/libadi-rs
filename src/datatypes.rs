pub enum Loopback {
    Disable,
    Digital,
    RF,
}

#[derive(Debug)]
pub enum SdrDataType {
    Int16,
}

impl SdrDataType {
    pub fn get_size(&self) -> usize {
        match self {
            Self::Int16 => 2,
        }
    }
}

pub type pluto_complex = num::complex::Complex<f32>;
