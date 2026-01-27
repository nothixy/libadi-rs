use crate::types;

#[derive(Debug)]
pub struct DeviceDebugAttr {
    name: String,
    filename: String,
}

impl types::traits::Attr for DeviceDebugAttr {
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

impl DeviceDebugAttr {
    pub fn new(name: String) -> DeviceDebugAttr {
        let mut device_debug_attr = DeviceDebugAttr {
            name: name.clone(),
            filename: "".to_owned(),
        };
        types::traits::Attr::init(&mut device_debug_attr, name, None);
        device_debug_attr
    }

    fn read(&self, device: &iio::IIODevice) -> Result<&str, ()> {
        let res = device.debug_attr_read(self.name.as_str(), 1024)?;
        Ok(res.0)
    }

    fn write(&self, device: &iio::IIODevice, value: &str) -> isize {
        device.debug_attr_write(self.name.as_str(), value)
    }

    fn get_value(&self, device: &iio::IIODevice) -> Result<&str, ()> {
        self.read(device)
    }

    fn set_value(&mut self, device: &iio::IIODevice, value: &str) -> isize {
        self.write(device, value)
    }
}
