use crate::types;

#[derive(Debug)]
pub struct DeviceBufferAttr {
    name: String,
    filename: String,
}

impl types::traits::Attr for DeviceBufferAttr {
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

impl DeviceBufferAttr {
    pub fn new(name: String) -> DeviceBufferAttr {
        let mut device_buffer_attr = DeviceBufferAttr {
            name: name.clone(),
            filename: "".to_owned(),
        };
        types::traits::Attr::init(&mut device_buffer_attr, name, None);
        device_buffer_attr
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
