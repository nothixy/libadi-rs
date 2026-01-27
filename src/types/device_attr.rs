use crate::types;

#[derive(Debug)]
pub struct DeviceAttr {
    name: String,
    filename: String,
}

impl types::traits::Attr for DeviceAttr {
    fn init(&mut self, name: String, filename_opt: Option<String>) {
        self.name = name;
        self.filename = if let Some(filename) = filename_opt {
            filename
        } else {
            self.name.clone()
        };
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_filename(&self) -> &str {
        self.filename.as_str()
    }
}

impl DeviceAttr {
    pub fn new(name: String) -> DeviceAttr {
        let mut device_attr = DeviceAttr {
            name: name.clone(),
            filename: "".to_owned(),
        };
        types::traits::Attr::init(&mut device_attr, name, None);
        device_attr
    }

    fn read(&self, device: &iio::IIODevice) -> Result<&str, ()> {
        let res = device.attr_read(self.name.as_str(), 1024)?;
        Ok(res.0)
    }

    fn write(&self, device: &iio::IIODevice, value: &str) -> isize {
        device.attr_write(self.name.as_str(), value)
    }

    pub fn get_value(&self, device: &iio::IIODevice) -> Result<&str, ()> {
        self.read(device)
    }

    pub fn set_value(&mut self, device: &iio::IIODevice, value: &str) -> isize {
        self.write(device, value)
    }
}
