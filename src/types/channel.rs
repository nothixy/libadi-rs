use crate::types;

#[derive(Debug)]
pub struct Channel {
    attributes: std::collections::HashMap<String, types::channel_attr::ChannelAttr>,
    id: String,
    name: Option<String>,
    is_output: bool,
    is_scan_element: bool,
}

impl Channel {
    pub fn new(channel: &iio::IIOChannel) -> Result<Channel, ()> {
        let mut attributes = std::collections::HashMap::new();
        let id = channel.get_id();
        let name = channel.get_name();
        let is_output = channel.is_output();
        let is_scan_element = channel.is_scan_element();
        for index in 0..channel.get_attrs_count() {
            let attr = channel.get_attr(index);
            let channel_attr = types::channel_attr::ChannelAttr::new(attr.to_owned());
            attributes.insert(attr, channel_attr);
        }
        Ok(Channel {
            attributes,
            id,
            name,
            is_output,
            is_scan_element,
        })
    }

    pub fn read(
        &self,
        channel: &iio::IIOChannel,
        buf: &mut types::buffer::Buffer,
        raw_opt: Option<bool>,
    ) -> Result<Vec<u8>, ()> {
        let raw = raw_opt.unwrap_or(false);
        let buflen = buf.len();
        if raw {
            channel.read_raw(buf.get_buffer(), buflen, raw)
        } else {
            channel.read(buf.get_buffer(), buflen, raw)
        }
    }

    pub fn write(
        &self,
        channel: &iio::IIOChannel,
        buf: &mut types::buffer::Buffer,
        array: Vec<u8>,
        raw_opt: Option<bool>,
    ) -> usize {
        let raw = raw_opt.unwrap_or(false);
        if raw {
            channel.write_raw(buf.get_buffer(), array, raw)
        } else {
            channel.write(buf.get_buffer(), array, raw)
        }
    }

    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn get_attrs(
        &mut self,
    ) -> &mut std::collections::HashMap<String, types::channel_attr::ChannelAttr> {
        &mut self.attributes
    }

    pub fn get_is_output(&self) -> bool {
        self.is_output
    }

    pub fn get_is_scan_element(&self) -> bool {
        self.is_scan_element
    }

    pub fn get_enabled(&self, channel: &iio::IIOChannel) -> bool {
        channel.is_enabled()
    }

    pub fn set_enabled(&self, channel: &iio::IIOChannel, enabled: bool) {
        if enabled {
            channel.enable();
        } else {
            channel.disable();
        }
    }

    pub fn get_index(channel: &iio::IIOChannel) -> i64 {
        channel.get_index()
    }

    pub fn get_data_format(channel: &iio::IIOChannel) -> Option<&iio::IIODataFormat> {
        channel.get_data_format()
    }

    pub fn get_modifier(channel: &iio::IIOChannel) -> iio::IIOModifier {
        channel.get_modifier()
    }

    pub fn get_type(channel: &iio::IIOChannel) -> iio::IIOChanType {
        channel.get_type()
    }
}
