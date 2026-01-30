use crate::types;

#[derive(Debug)]
pub struct Context<'a> {
    context: Box<iio::IIOContext>,
    attrs: std::collections::HashMap<&'a str, &'a str>,
    name: &'a str,
    description: &'a str,
    xml: &'a str,
    version: iio::IIOVersion,
}

impl<'a> Context<'a> {
    pub fn get_iio_context(&self) -> &iio::IIOContext {
        &self.context
    }

    pub fn set_timeout(&self, timeout_ms: u32) -> Result<(), i32> {
        self.context.set_timeout(timeout_ms)
    }

    pub fn find_device(
        &self,
        name_or_id_or_label: &str,
    ) -> Result<Box<types::device_trigger::DeviceTrigger>, ()> {
        let device = self.context.find_device(name_or_id_or_label)?;
        let device_mut = types::device_trigger::DeviceTrigger::new(device).map_err(|_| ())?;
        Ok(Box::new(device_mut))
    }

    pub fn new_from_string(uri: String) -> Result<Context<'a>, ()> {
        let context_ptr = iio::IIOContext::create_from_uri(uri.as_str());
        if let Some(mut context) = context_ptr {
            let version = context.get_version().map_err(|_| ())?;
            let name = context.get_name().map_err(|_| ())?;
            let description = context.get_description().map_err(|_| ())?;
            let xml = context.get_xml().map_err(|_| ())?;
            let attr_count = context.get_attrs_count();
            let mut attrs_map = std::collections::HashMap::new();
            for index in 0..attr_count {
                let attr = context.get_attr(index)?;
                attrs_map.insert(attr.0, attr.1);
            }
            let attrs = attrs_map;
            Ok(Context {
                context,
                attrs,
                name,
                description,
                xml,
                version,
            })
        } else {
            let error = std::io::Error::last_os_error();
            eprintln!("Error: {}", error);
            Err(())
        }
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_description(&self) -> &str {
        self.description
    }

    pub fn get_xml(&self) -> &str {
        self.xml
    }

    pub fn get_version(&self) -> iio::IIOVersion {
        self.version.clone()
    }

    pub fn get_attrs(&self) -> &std::collections::HashMap<&str, &str> {
        &self.attrs
    }

    pub fn get_devices(
        &'a self,
    ) -> Result<Vec<types::device_trigger::DeviceTrigger>, std::str::Utf8Error> {
        let mut devices: Vec<types::device_trigger::DeviceTrigger> = vec![];
        let device_count = self.context.get_devices_count();
        for i in 0..device_count {
            let device_opt = self.context.get_device(i);
            if let Some(device) = device_opt {
                let device_mut = types::device_trigger::DeviceTrigger::new(device)?;
                devices.push(device_mut);
            }
        }
        Ok(devices)
    }
}
