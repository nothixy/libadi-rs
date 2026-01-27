use crate::types;

#[derive(Debug)]
pub struct ChannelAttr {
    name: String,
    filename: String,
    // channel: &'a iio::IIOChannel,
}

impl types::traits::Attr for ChannelAttr {
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

impl ChannelAttr {
    pub fn new(name: String) -> ChannelAttr {
        let mut channel_attr = ChannelAttr {
            name: name.clone(),
            filename: "".to_owned(),
            // channel,
        };
        types::traits::Attr::init(&mut channel_attr, name, None);
        channel_attr
    }

    fn read(&self, channel: &iio::IIOChannel) -> Result<String, ()> {
        let res = channel.attr_read(self.name.as_str(), 1024)?;
        Ok(res.0)
    }

    fn write(&self, channel: &iio::IIOChannel, value: &str) -> isize {
        channel.attr_write(self.name.as_str(), value)
    }

    pub fn get_value(&self, channel: &iio::IIOChannel) -> Result<String, ()> {
        self.read(channel)
    }

    pub fn set_value(&mut self, channel: &iio::IIOChannel, value: &str) -> isize {
        self.write(channel, value)
    }
}
