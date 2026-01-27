pub trait Attr {
    fn init(&mut self, name: String, filename: Option<String>);
    fn get_name(&self) -> &str;
    fn get_filename(&self) -> &str;
}

pub trait Crx {
    fn rx_init_channels(&mut self) -> Result<(), ()>;
    fn rx_buffered_data(&mut self) -> Result<Vec<Vec<i128>>, ()>;
}

pub trait Ctx {
    fn tx_init_channels(&mut self) -> Result<(), ()>;
    fn tx_buffer_push(&mut self, data: Vec<u8>) -> Result<(), ()>;
}

pub enum DdsValue {
    Float(f64),
    Int(i128),
    Bool(bool)
}

impl From<&bool> for DdsValue {
    fn from(value: &bool) -> Self {
        DdsValue::Bool(*value)
    }
}

impl From<&i128> for DdsValue {
    fn from(value: &i128) -> Self {
        DdsValue::Int(*value)
    }
}

impl From<&f64> for DdsValue {
    fn from(value: &f64) -> Self {
        DdsValue::Float(*value)
    }
}

impl DdsValue {
    pub fn get_string(&self) -> String
    {
        match self {
            Self::Float(f) => format!("{:.1}", f),
            Self::Bool(b) => (if *b { "1" } else { "0" }).to_owned(),
            Self::Int(i) => format!("{:}", i)
        }
    }
}
