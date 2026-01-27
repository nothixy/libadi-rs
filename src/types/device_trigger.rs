use crate::types;

#[derive(Debug)]
pub struct DeviceTrigger {
    attrs: std::collections::HashMap<String, types::device_attr::DeviceAttr>,
    debug_attrs: std::collections::HashMap<String, types::device_debug_attr::DeviceDebugAttr>,
    buffer_attrs: std::collections::HashMap<String, types::device_buffer_attr::DeviceBufferAttr>,
    id: String,
    name: String,
    label: Option<String>,
}

impl DeviceTrigger {
    pub fn new(device: &iio::IIODevice) -> Result<Self, std::str::Utf8Error> {
        let mut attrs = std::collections::HashMap::new();
        let attr_count = device.get_attrs_count();
        for i in 0..attr_count {
            let attr = device.get_attr(i)?;
            let device_attr = types::device_attr::DeviceAttr::new(attr.to_owned());
            attrs.insert(attr.to_owned(), device_attr);
        }
        let mut debug_attrs = std::collections::HashMap::new();
        let debug_attr_count = device.get_debug_attrs_count();
        for i in 0..debug_attr_count {
            let debug_attr = device.get_debug_attr(i)?;
            let device_debug_attr =
                types::device_debug_attr::DeviceDebugAttr::new(debug_attr.to_owned());
            debug_attrs.insert(debug_attr.to_owned(), device_debug_attr);
        }
        let mut buffer_attrs = std::collections::HashMap::new();
        let buffer_attr_count = device.get_buffer_attrs_count();
        for i in 0..buffer_attr_count {
            let buffer_attr = device.get_buffer_attr(i)?;
            let device_buffer_attr =
                types::device_buffer_attr::DeviceBufferAttr::new(buffer_attr.to_owned());
            buffer_attrs.insert(buffer_attr.to_owned(), device_buffer_attr);
        }
        let id = device.get_id()?;
        let name = device.get_name()?;
        let label = device.get_label();

        Ok(DeviceTrigger {
            attrs,
            debug_attrs,
            buffer_attrs,
            id,
            name,
            label,
        })
    }

    pub fn reg_write(device: &mut iio::IIODevice, reg: u32, value: u32) -> Result<(), i32> {
        let res = device.reg_write(reg, value);
        if res < 0 { Err(res) } else { Ok(()) }
    }

    pub fn reg_read(device: &mut iio::IIODevice, reg: u32) -> Result<u32, i32> {
        device.reg_read(reg)
    }

    pub fn find_channel(
        &self,
        device: &iio::IIODevice,
        name_or_id: &str,
        output_opt: Option<bool>,
    ) -> Result<types::channel::Channel, ()> {
        let output = output_opt.unwrap_or(false);
        let channel = device.find_channel(name_or_id, output)?;
        types::channel::Channel::new(channel)
    }

    pub fn set_kernel_buffers_count(device: &mut iio::IIODevice, count: u32) -> Result<(), i32> {
        device.set_kernel_buffers_count(count)
    }

    pub fn get_sample_size(device: &iio::IIODevice) -> isize {
        device.get_sample_size()
    }

    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub fn get_attrs(
        &mut self,
    ) -> &mut std::collections::HashMap<String, types::device_attr::DeviceAttr> {
        &mut self.attrs
    }

    pub fn get_debug_attrs(
        &mut self,
    ) -> &mut std::collections::HashMap<String, types::device_debug_attr::DeviceDebugAttr> {
        &mut self.debug_attrs
    }

    pub fn get_buffer_attrs(
        &mut self,
    ) -> &mut std::collections::HashMap<String, types::device_buffer_attr::DeviceBufferAttr> {
        &mut self.buffer_attrs
    }

    pub fn get_channels(
        &self,
        device: &iio::IIODevice,
    ) -> Result<Vec<types::channel::Channel>, ()> {
        let channel_count = device.get_channels_count();
        let mut channels = vec![];
        for i in 0..channel_count {
            let iio_channel_opt = device.get_channel(i);
            if let Some(iio_channel) = iio_channel_opt {
                let channel = types::channel::Channel::new(iio_channel)?;
                channels.push(channel)
            }
        }
        Ok(channels)
    }
}
