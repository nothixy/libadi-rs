use std::io::Write;

use crate::datatypes;
use crate::fir;
use crate::traits;
use crate::traits::DDS;
use crate::traits::RxCore;
use crate::traits::TxCore;
use crate::types;

#[derive(Debug)]
pub struct Pluto<'a> {
    // Base Pluto fields
    device_name: Option<String>,

    // Inherited from RxTxCommon trait
    complex_data: bool,

    // Inherited from ad9364 trait
    rx_channel_names: Option<Vec<String>>,
    tx_channel_names: Option<Vec<String>>,
    control_device_name: String,
    rx_data_device_name: String,
    tx_data_device_name: String,

    // Inherited from TxCore trait
    tx_complex_data: Option<bool>,
    tx_data_type: Option<datatypes::SdrDataType>,
    tx_cyclic_buffer: bool,
    tx_enabled_channels: Vec<u32>,
    tx_output_byte_filename: String,
    txbuf: Option<types::buffer::Buffer>,
    tx_push_to_file: bool,
    num_tx_channels: u32,
    tx_buffer_size: usize,

    // Inherited from RxCore trait
    rx_complex_data: Option<bool>,
    rx_data_type: datatypes::SdrDataType,
    rx_data_si_type: datatypes::SdrDataType,
    rx_shift: u32,
    rx_buffer_size: usize,
    rx_enabled_channels: Vec<u32>,
    rx_output_type: String,
    rxbuf: Option<types::buffer::Buffer>,
    rx_unbuffered_data: bool,
    rx_annotated: bool,
    rx_stack_interleaved: bool,
    num_rx_channels: u32,

    // Inherited from TxDef trait
    txdac: Option<Box<types::device_trigger::DeviceTrigger>>,

    // Inherited from RxDef trait
    rxadc: Option<Box<types::device_trigger::DeviceTrigger>>,

    // Inherited from ContextManager trait
    context: Box<types::context::Context<'a>>,

    // Inherited from SharedDef trait
    ctrl: Box<types::device_trigger::DeviceTrigger>,

    // Inherited from DDS trait
    split_cores: bool,
}

fn implicit_convert(data: &[u8], be: bool, signed: bool, length: u32) -> Result<i128, ()> {
    let res = match be {
        true => match signed {
            true => match length {
                8 => data[0] as i128,
                16 => i16::from_be_bytes([data[0], data[1]]) as i128,
                32 => i32::from_be_bytes([data[0], data[1], data[2], data[3]]) as i128,
                64 => i64::from_be_bytes([
                    data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
                ]) as i128,
                _ => Err(())?,
            },
            false => match length {
                8 => data[0] as i128,
                16 => u16::from_be_bytes([data[0], data[1]]) as i128,
                32 => u32::from_be_bytes([data[0], data[1], data[2], data[3]]) as i128,
                64 => u64::from_be_bytes([
                    data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
                ]) as i128,
                _ => Err(())?,
            },
        },
        false => match signed {
            true => match length {
                8 => data[0] as i128,
                16 => i16::from_le_bytes([data[0], data[1]]) as i128,
                32 => i32::from_le_bytes([data[0], data[1], data[2], data[3]]) as i128,
                64 => i64::from_le_bytes([
                    data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
                ]) as i128,
                _ => Err(())?,
            },
            false => match length {
                8 => data[0] as i128,
                16 => u16::from_le_bytes([data[0], data[1]]) as i128,
                32 => u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as i128,
                64 => u64::from_le_bytes([
                    data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
                ]) as i128,
                _ => Err(())?,
            },
        },
    };

    Ok(res)
}

// En fait ce qu'il faut c'est des impl<'a, T> traits::...<'a> for T where T: traits::...<'a>
// Le problème c'est qu'il faut des getters dans tous les sens et j'ai une flemme monstrueuse de faire ça maintenant

impl<'a> Pluto<'a> {
    pub fn new(uri: Option<String>) -> Result<Pluto<'a>, ()> {
        let uri_string = uri.unwrap_or("192.168.2.1".to_owned());

        let device_name = Some("PlutoSDR".to_owned());

        let buffer_size = None;

        let (
            complex_data_opt,
            rx_channel_names,
            tx_channel_names,
            control_device_name,
            rx_data_device_name,
            tx_data_device_name,
            // ) = <Pluto<'a> as traits::AD9364>::init();
        ) = <Pluto<'a> as traits::AD9364<'a>>::init();

        let (context, ctrl) = <Pluto<'a> as traits::SharedDef<'a>>::init(
            Some(uri_string),
            device_name.clone(),
            rx_data_device_name.as_str(),
            Some(control_device_name.as_str()),
        )?;

        // Already initialized by SharedDef trait
        // let context = <Pluto<'a> as traits::ContextManager>::init(Some(uri_string), Some("PlutoSDR"))?;

        // Unused by ad9364
        // let (run_rx_post_init, run_tx_post_init) = <Pluto<'a> as traits::RxTxDef>::init();

        let split_cores = <Pluto<'a> as traits::DDS<'a>>::init();

        let (rxadc, rx_channel_names) = <Pluto<'a> as traits::RxDef<'a>>::init(
            &context,
            Some(&rx_data_device_name),
            rx_channel_names,
        )?;

        let (txdac, tx_channel_names) = <Pluto<'a> as traits::TxDef<'a>>::init(
            &context,
            Some(&tx_data_device_name),
            tx_channel_names,
        )?;

        let complex_data = <Pluto<'a> as traits::RxTxCommon<'a>>::init(complex_data_opt);

        let (
            rx_complex_data,
            rx_data_type,
            rx_data_si_type,
            rx_shift,
            rx_buffer_size,
            rx_enabled_channels,
            rx_output_type,
            rxbuf,
            rx_unbuffered_data,
            rx_annotated,
            rx_stack_interleaved,
            num_rx_channels,
        ) = <Pluto<'a> as traits::RxCore<'a>>::init(buffer_size, complex_data, &rx_channel_names)?;

        let (
            tx_complex_data,
            tx_data_type,
            tx_cyclic_buffer,
            tx_enabled_channels,
            tx_output_byte_filename,
            txbuf,
            tx_push_to_file,
            num_tx_channels,
            tx_buffer_size,
        ) = <Pluto<'a> as traits::TxCore<'a>>::init(None, complex_data, &tx_channel_names)?;

        Ok(Pluto {
            device_name,

            complex_data,
            rx_channel_names,
            tx_channel_names,
            control_device_name,
            rx_data_device_name,
            tx_data_device_name,

            rx_complex_data,
            rx_data_type,
            rx_data_si_type,
            rx_shift,
            rx_buffer_size,
            rx_enabled_channels,
            rx_output_type,
            rxbuf,
            rx_unbuffered_data,
            rx_annotated,
            rx_stack_interleaved,
            num_rx_channels,

            tx_complex_data,
            tx_data_type,
            tx_cyclic_buffer,
            tx_enabled_channels,
            tx_output_byte_filename,
            txbuf,
            tx_push_to_file,
            num_tx_channels,
            tx_buffer_size,

            txdac,
            rxadc,

            context,

            ctrl,

            split_cores,
        })
    }
}

impl<'a> traits::Attribute<'a> for Pluto<'a> {
    fn set_iio_attr_str(
        &'a self,
        channel_name: &str,
        attr_name: &'a str,
        output: Option<bool>,
        value: &'a str,
        ctrl: Option<&'a types::device_trigger::DeviceTrigger>,
    ) -> Result<(), ()> {
        let control = ctrl.unwrap_or(&self.ctrl);
        let box_context = self.context.as_ref();
        let iio_context = box_context.get_iio_context();
        let iio_device = iio_context.find_device("ad9361-phy")?;
        let iio_channel = iio_device.find_channel(channel_name, output.unwrap_or(false))?;
        let mut channel = control.find_channel(iio_device, channel_name, output)?;
        let attrs = channel.get_attrs();
        let entry = attrs.get_mut(attr_name).ok_or(())?;
        let result = entry.set_value(iio_channel, value);
        if result <= 0 { Err(()) } else { Ok(()) }
    }

    fn get_iio_attr_str(
        &'a self,
        channel_name: &'a str,
        attr_name: &'a str,
        output: Option<bool>,
        ctrl: Option<&'a types::device_trigger::DeviceTrigger>,
    ) -> Result<String, ()> {
        let control = ctrl.unwrap_or(&self.ctrl);
        let iio_context = self.context.get_iio_context();
        let iio_device = iio_context.find_device("ad9361-phy")?;
        let iio_channel = iio_device.find_channel(channel_name, output.unwrap_or(false))?;
        let mut channel = control.find_channel(iio_device, channel_name, output)?;
        let attrs = channel.get_attrs();
        let attr = attrs.get(attr_name).ok_or(())?;
        Ok(attr.get_value(iio_channel)?.to_owned())
    }

    fn set_iio_dev_attr_str(
        &mut self,
        attr_name: &str,
        value: &str,
        ctrl: Option<&mut types::device_trigger::DeviceTrigger>,
    ) -> Result<(), ()> {
        let control = ctrl.unwrap_or(&mut self.ctrl);
        let attrs = control.get_attrs();
        let box_context = self.context.as_ref();
        let iio_context = box_context.get_iio_context();
        let iio_device = iio_context.find_device("ad9361-phy")?;
        let entry = attrs.get_mut(attr_name).ok_or(())?;
        let result = entry.set_value(iio_device, value);
        if result <= 0 { Err(()) } else { Ok(()) }
    }

    fn get_iio_dev_attr_str(
        &mut self,
        attr_name: &str,
        ctrl: Option<&mut types::device_trigger::DeviceTrigger>,
    ) -> Result<String, ()> {
        let control = ctrl.unwrap_or(&mut self.ctrl);
        let attrs = control.get_attrs();
        let iio_context = self.context.get_iio_context();
        let iio_device = iio_context.find_device("ad9361-phy")?;
        let attr = attrs.get(attr_name).ok_or(())?;
        Ok(attr.get_value(iio_device)?.to_owned())
    }

    fn set_iio_debug_attr_str(
        &self,
        _attr_name: &str,
        _value: &str,
        _ctrl: Option<&types::device_trigger::DeviceTrigger>,
    ) -> Result<(), ()> {
        todo!()
    }

    fn get_iio_debug_attr_str(
        &self,
        _attr_name: &str,
        _ctrl: Option<&types::device_trigger::DeviceTrigger>,
    ) -> Result<String, ()> {
        todo!()
    }

    fn set_iio_attr_int<T: Into<i128>>(
        &'a self,
        channel_name: &str,
        attr_name: &'a str,
        output: Option<bool>,
        value: T,
        ctrl: Option<&'a types::device_trigger::DeviceTrigger>,
    ) -> Result<(), ()> {
        let value_string = format!("{:}", value.into());
        self.set_iio_attr_str(channel_name, attr_name, output, value_string.as_str(), ctrl)
    }

    fn get_iio_attr_int(
        &'a self,
        channel_name: &'a str,
        attr_name: &'a str,
        output: Option<bool>,
        ctrl: Option<&'a types::device_trigger::DeviceTrigger>,
    ) -> Result<i128, ()> {
        let res = self.get_iio_attr_str(channel_name, attr_name, output, ctrl)?;
        let bytes_with_dot = res
            .as_bytes()
            .iter()
            .take_while(|f| f.is_ascii_digit())
            .copied()
            .collect::<Vec<u8>>();
        let bytes_string = String::from_utf8_lossy(bytes_with_dot.as_slice()).to_string();
        bytes_string.parse::<i128>().map_err(|_| ())
    }

    fn set_iio_attr_float<T: Into<f64>>(
        &'a self,
        channel_name: &str,
        attr_name: &'a str,
        output: Option<bool>,
        value: T,
        ctrl: Option<&'a types::device_trigger::DeviceTrigger>,
    ) -> Result<(), ()> {
        let value_string = format!("{:.1}", value.into());
        self.set_iio_attr_str(channel_name, attr_name, output, value_string.as_str(), ctrl)
    }

    fn get_iio_attr_float(
        &'a self,
        channel_name: &'a str,
        attr_name: &'a str,
        output: Option<bool>,
        ctrl: Option<&'a types::device_trigger::DeviceTrigger>,
    ) -> Result<f64, ()> {
        let res = self.get_iio_attr_str(channel_name, attr_name, output, ctrl)?;
        let bytes_with_dot = res
            .as_bytes()
            .iter()
            .take_while(|f| f.is_ascii_digit() || **f == b'.')
            .copied()
            .collect::<Vec<u8>>();
        let bytes_string = String::from_utf8_lossy(bytes_with_dot.as_slice()).to_string();
        bytes_string.parse::<f64>().map_err(|_| ())
    }
}

impl<'a> traits::ContextManager<'a> for Pluto<'a> {
    fn get_ctx(&self) -> &'a types::device_trigger::DeviceTrigger {
        todo!()
    }

    fn init(
        uri: Option<String>,
        device_name: Option<&str>,
    ) -> Result<Box<types::context::Context<'a>>, ()> {
        let version = iio::get_version();
        println!("Using IIO library version {}", version.get_tag());

        if let Some(uri_str) = uri {
            let context = types::context::Context::new_from_string(uri_str)?;
            let final_context = Box::new(context);
            return Ok(final_context);
        }

        if let Some(device_name_str) = device_name {
            let contexts = types::context_manager::scan_contexts()?;
            for context in contexts {
                if context.1.contains(device_name_str) {
                    let context_0 = context.0;
                    let context_res = types::context::Context::new_from_string(context_0.to_owned())?;
                    let final_context = Box::new(context_res);
                    return Ok(final_context);
                }
            }
            Err(())
        } else {
            Err(())
        }
    }
}

impl<'a> traits::SharedDef<'a> for Pluto<'a> {
    fn get_complex_data(&self) {
        todo!()
    }

    fn get_control_device_name(&self) {
        todo!()
    }

    fn init(
        uri_opt: Option<String>,
        device_name: Option<String>,
        rx_data_device_name: &str,
        control_device_name_opt: Option<&str>,
    ) -> Result<
        (
            Box<types::context::Context<'a>>,
            Box<types::device_trigger::DeviceTrigger>,
        ),
        (),
    > {
        let device_name_opt = device_name.ok_or(())?;
        let control_device_name = control_device_name_opt.ok_or(())?;

        let context = if let Some(uri) = uri_opt {
            let context = <Pluto<'a> as traits::ContextManager<'a>>::init(
                Some(uri),
                Some(device_name_opt.as_str()),
            )?;
            Ok(context)
        } else {
            let required_devices = [rx_data_device_name, control_device_name];
            let contexts = types::context_manager::scan_contexts()?;
            let mut context_ret = None;
            for context in contexts {
                let inner_context = types::context::Context::new_from_string(context.0.to_owned())?;
                let devs_devices = inner_context.get_devices().map_err(|_| ())?;
                let mut devs = devs_devices.iter().map(|f| f.get_name());
                if devs.all(|f| required_devices.contains(&f)) {
                    context_ret = Some(types::context::Context::new_from_string(context.0.to_owned())?);
                }
            }

            if let Some(context) = context_ret {
                Ok(Box::new(context))
            } else {
                Err(())
            }
        }?;

        let control = if let Some(control_device_name) = control_device_name_opt {
            Some(context.find_device(control_device_name)?)
        } else {
            None
        }
        .ok_or(())?;

        Ok((context, control))
    }

    fn post_init(&self) {
        todo!()
    }
}

impl<'a> traits::RxTxCommon<'a> for Pluto<'a> {
    fn init(complex_data: Option<bool>) -> bool {
        complex_data.unwrap_or(false)
    }

    fn annotate(&self, _data: Vec<f32>, _channel_names: Vec<&str>, _enabled_channels: Vec<&str>) {
        todo!()
    }
}

impl<'a> types::traits::Crx for Pluto<'a> {
    fn rx_init_channels(&mut self) -> Result<(), ()> {
        let rx_channel_names = self.rx_channel_names.clone().ok_or(())?;

        let rxadc = self.rxadc.as_ref().ok_or(())?;
        let iio_context = self.context.as_ref().get_iio_context();
        let iio_rxadc = iio_context.find_device(&self.rx_data_device_name)?;

        for channel_name in rx_channel_names.iter() {
            let channel = rxadc.find_channel(iio_rxadc, channel_name, None)?;
            let iio_channel = iio_rxadc.find_channel(channel_name, false)?;
            channel.set_enabled(iio_channel, true);
        }

        if <Pluto<'a> as RxCore<'a>>::get_rx_complex_data(self) {
            for channel_idx in self.rx_enabled_channels.iter() {
                let channel1 = rx_channel_names[(*channel_idx * 2) as usize].as_str();
                let channel2 = rx_channel_names[(*channel_idx * 2 + 1) as usize].as_str();
                for channel_name in [channel1, channel2] {
                    let channel = rxadc.find_channel(iio_rxadc, channel_name, None)?;
                    let iio_channel = iio_rxadc.find_channel(channel_name, false)?;
                    channel.set_enabled(iio_channel, true);
                }
            }
        } else {
            for channel_idx in self.rx_enabled_channels.iter() {
                let channel_name = rx_channel_names[*channel_idx as usize].as_str();
                let channel = rxadc.find_channel(iio_rxadc, channel_name, None)?;
                let iio_channel = iio_rxadc.find_channel(channel_name, false)?;
                channel.set_enabled(iio_channel, true);
            }
        }

        self.rxbuf = Some(types::buffer::Buffer::new(
            iio_rxadc,
            self.rx_buffer_size as usize,
            Some(false),
        )?);

        Ok(())
    }

    fn rx_buffered_data(&mut self) -> Result<Vec<Vec<i128>>, ()> {
        let complex_data = <Pluto<'a> as RxCore<'a>>::get_rx_complex_data(self);
        if self.rxbuf.is_none() {
            <Pluto<'a> as types::traits::Crx>::rx_init_channels(self)?;
        }
        let rxbuf = self.rxbuf.as_mut().ok_or(())?;
        rxbuf.refill();

        let rx_channel_names = self.rx_channel_names.clone().ok_or(())?;
        let rxadc = self.rxadc.as_ref().ok_or(())?;
        let iio_context = self.context.as_ref().get_iio_context();
        let iio_rxadc = iio_context.find_device(&self.rx_data_device_name)?;

        let mut data_channel_interleaved = vec![];
        let mut ecn = vec![];

        if complex_data {
            for channel_idx in self.rx_enabled_channels.iter() {
                let channel1 = rx_channel_names[(*channel_idx * 2) as usize].as_str();
                let channel2 = rx_channel_names[(*channel_idx * 2 + 1) as usize].as_str();
                ecn.push(channel1);
                ecn.push(channel2);
            }
        } else {
            for channel_idx in self.rx_enabled_channels.iter() {
                let channel = rx_channel_names[*channel_idx as usize].as_str();
                ecn.push(channel);
            }
        };

        for name in ecn {
            let channel = rxadc.find_channel(iio_rxadc, name, None)?;
            let iio_channel = iio_rxadc.find_channel(name, false)?;
            let data = channel.read(iio_channel, rxbuf, None)?;
            let data_format = types::channel::Channel::get_data_format(iio_channel).ok_or(())?;
            let data_formatted = data
                .chunks_exact((data_format.length / 8) as usize)
                .map(|d| {
                    implicit_convert(
                        d,
                        data_format.is_be,
                        data_format.is_signed,
                        data_format.length,
                    )
                })
                .collect::<Result<Vec<i128>, ()>>()?;
            data_channel_interleaved.push(data_formatted);
        }

        Ok(data_channel_interleaved)
    }
}

impl<'a> traits::DDS<'a> for Pluto<'a> {
    fn init() -> bool {
        true
    }

    fn update_dds(&self, attr: &str, value: Vec<types::traits::DdsValue>) -> Result<(), ()> {
        let _tx_channel_names = self.tx_channel_names.clone().ok_or(())?;
        let txdac = self.txdac.as_ref().ok_or(())?;
        let iio_context = self.context.as_ref().get_iio_context();
        let iio_txdac = iio_context.find_device(&self.tx_data_device_name)?;
        let channels = txdac.get_channels(iio_txdac)?;
        let mut _split_cores_indx = 0;
        for index in 0..channels.len() {
            let channel_name_string = "altvoltage".to_owned() + index.to_string().as_str();
            let channel_name = channel_name_string.as_str();
            let channel = txdac.find_channel(iio_txdac, channel_name, Some(true));
            if channel.is_err() {
                // TODO: This should be implemented
                return Ok(());
            }
            let mut final_channel = channel?;
            if index >= value.len() {
                return Ok(());
            }
            let iio_channel = iio_txdac.find_channel(channel_name, true)?;
            let attrs = final_channel.get_attrs();
            let entry = attrs.get_mut(attr).ok_or(())?;
            let result = entry.set_value(iio_channel, value[index].get_string().as_str());
            if result <= 0
            {
                return Err(());
            }
        }

        Ok(())
    }

    fn read_dds(&self, _attr: &str) -> &str {
        todo!()
    }

    fn disable_dds(&self) -> Result<(), ()> {
        let value = vec![false; (self.num_tx_channels * 2) as usize];
        <Pluto<'a> as DDS<'a>>::set_dds_enabled(self, value)
    }

    fn get_dds_frequencies(&self) -> &str {
        todo!()
    }

    fn set_dds_frequencies(&self, _value: &str) {
        todo!()
    }

    fn get_dds_scales(&self) -> &str {
        todo!()
    }

    fn set_dds_scales(&self, _value: &str) {
        todo!()
    }

    fn get_dds_phases(&self) -> &str {
        todo!()
    }

    fn set_dds_phases(&self, _value: &str) {
        todo!()
    }

    fn get_dds_enabled(&self) -> &str {
        todo!()
    }

    fn set_dds_enabled(&self, value: Vec<bool>) -> Result<(), ()> {
        <Pluto<'a> as DDS<'a>>::update_dds(self, "raw", value.iter().map(|f| f.into()).collect())
    }

    fn dds_single_tone(&self, _frequency: i32, _scale: f32, _channel: i32) {
        todo!()
    }

    fn dds_dual_tone(
        &self,
        _frequency1: i32,
        _scale1: f32,
        _frequncy2: i32,
        _scale2: f32,
        _channel: i32,
    ) {
        todo!()
    }
}

impl<'a> traits::RxCore<'a> for Pluto<'a> {
    fn init(
        in_rx_buffer_size: Option<usize>,
        complex_data: bool,
        in_rx_channel_names_opt: &Option<Vec<String>>,
    ) -> traits::RxCoreInitResult {
        let in_rx_channel_names = in_rx_channel_names_opt.as_ref().ok_or(())?;
        let rx_complex_data = None;
        let rx_data_type = datatypes::SdrDataType::Int16;
        let rx_data_si_type = datatypes::SdrDataType::Int16;
        let rx_shift = 0;
        let rx_buffer_size = in_rx_buffer_size.unwrap_or(1024);
        let rx_output_type = "raw".to_owned();
        let rxbuf = None;
        let rx_unbuffered_data = false;
        let rx_annotated = false;
        let rx_stack_interleaved = false;
        let n = if complex_data { 2u32 } else { 1u32 };
        let num_rx_channels = in_rx_channel_names.len() as u32;
        let rx_enabled_channels_range = num_rx_channels / n;
        let rx_enabled_channels = (0..rx_enabled_channels_range).collect::<Vec<u32>>();
        Ok((
            rx_complex_data,
            rx_data_type,
            rx_data_si_type,
            rx_shift,
            rx_buffer_size,
            rx_enabled_channels,
            rx_output_type,
            rxbuf,
            rx_unbuffered_data,
            rx_annotated,
            rx_stack_interleaved,
            num_rx_channels,
        ))
    }

    fn get_rx_complex_data(&self) -> bool {
        self.rx_complex_data.unwrap_or(self.complex_data)
    }

    fn get_rx_channel_names(&self) -> Vec<&str> {
        todo!()
    }

    fn get_rx_annotated(&self) -> bool {
        self.rx_annotated
    }

    fn set_rx_annotated(&mut self, value: bool) {
        self.rx_annotated = value;
    }

    fn get_rx_output_type(&self) -> &str {
        self.rx_output_type.as_str()
    }

    fn set_rx_output_type(&mut self, value: &str) {
        self.rx_output_type = value.to_owned();
    }

    fn get_rx_buffer_size(&self) -> usize {
        self.rx_buffer_size
    }

    fn set_rx_buffer_size(&mut self, value: usize) {
        self.rx_buffer_size = value;
    }

    fn get_rx_enabled_channels(&self) -> Vec<i32> {
        todo!()
    }

    fn set_rx_enabled_channels(&self, _value: Vec<i32>) {
        todo!()
    }

    fn get_num_rx_channels_enabled(&self) -> u32 {
        self.num_rx_channels
    }

    fn rx_destroy_buffer(&mut self) {
        todo!()
    }

    fn get_rx_channel_scales(&self) -> Vec<f32> {
        todo!()
    }

    fn get_rx_channel_offsets(&self) -> Vec<f32> {
        todo!()
    }

    fn rx_unbuffered_data(&self) -> Vec<Vec<f32>> {
        todo!()
    }

    fn rx_complex(&mut self) -> Result<Vec<Vec<datatypes::PlutoComplex>>, ()>
    {
        let mut out = vec![];
        let data = <Pluto<'a> as RxCore<'a>>::rx_buffered_data(self)?;
        let data_len = data.len();
        if !data_len.is_multiple_of(2) {
            return Err(());
        }
        for i in (0..data_len).step_by(2) {
            let zipped =
                std::iter::zip(data[i].clone(), data[i + 1].clone()).map(|f| datatypes::PlutoComplex::new(f.0 as f32, f.1 as f32)).collect::<Vec<datatypes::PlutoComplex>>();
            out.push(zipped);
        }

        Ok(out)
    }

    fn rx_non_complex(&self) -> Vec<Vec<f32>> {
        todo!()
    }

    fn rx(&mut self) -> Vec<Vec<f32>> {
        todo!()
        // let data = if self.rx_unbuffered_data
        // {
        //     self.rx_unbuffered_data()
        // }
        // else
        // {
        //     if self.get_rx_complex_data()
        //     {
        //         self.rx_complex()
        //     }
        //     else
        //     {
        //         self.rx_non_complex()
        //     }
        // };
    }

    fn rx_init_channels(&mut self) -> Result<(), ()> {
        <Pluto<'a> as types::traits::Crx>::rx_init_channels(self)
    }

    fn rx_buffered_data(&mut self) -> Result<Vec<Vec<i128>>, ()> {
        <Pluto<'a> as types::traits::Crx>::rx_buffered_data(self)
    }
}

impl<'a> traits::RxDef<'a> for Pluto<'a> {
    fn init(
        context: &types::context::Context,
        rx_data_device_name: Option<&String>,
        rx_channel_names: Option<Vec<String>>,
    ) -> traits::TxOrRxDefInitResult {
        let rxadc_opt = if let Some(device_name) = rx_data_device_name {
            Some(context.find_device(device_name.as_str())?)
        } else {
            None
        };

        let channel_names = if let Some(rxadc) = &rxadc_opt
            && rx_channel_names.is_none()
        {
            let iio_context = context.get_iio_context();
            let iio_device =
                iio_context.find_device(rx_data_device_name.as_ref().ok_or(())?.as_str())?;
            let tx_channels = rxadc.get_channels(iio_device)?;
            Some(
                tx_channels
                    .iter()
                    .filter(|f| f.get_is_scan_element())
                    .map(|f| f.get_id().to_owned())
                    .collect::<Vec<String>>(),
            )
        } else {
            rx_channel_names
        };

        Ok((rxadc_opt, channel_names))
    }

    fn get_tx_data_device_name(&self) -> &str {
        self.tx_data_device_name.as_str()
    }
}

impl<'a> types::traits::Ctx for Pluto<'a> {
    fn tx_init_channels(&mut self) -> Result<(), ()> {
        let tx_channel_names = self.tx_channel_names.clone().ok_or(())?;

        let txdac = self.txdac.as_ref().ok_or(())?;
        let iio_context = self.context.as_ref().get_iio_context();
        let iio_txdac = iio_context.find_device(&self.tx_data_device_name)?;

        if <Pluto<'a> as TxCore<'a>>::get_tx_complex_data(self) {
            for channel_idx in self.tx_enabled_channels.iter() {
                let channel1 = tx_channel_names[(*channel_idx * 2) as usize].as_str();
                let channel2 = tx_channel_names[(*channel_idx * 2 + 1) as usize].as_str();
                for channel_name in [channel1, channel2] {
                    let channel = txdac.find_channel(iio_txdac, channel_name, Some(true))?;
                    let iio_channel = iio_txdac.find_channel(channel_name, true)?;
                    channel.set_enabled(iio_channel, true);
                }
            }
        } else {
            for channel_idx in self.tx_enabled_channels.iter() {
                let channel_name = tx_channel_names[*channel_idx as usize].as_str();
                let channel = txdac.find_channel(iio_txdac, channel_name, Some(true))?;
                let iio_channel = iio_txdac.find_channel(channel_name, true)?;
                channel.set_enabled(iio_channel, true);
            }
        }

        self.txbuf = Some(types::buffer::Buffer::new(
            iio_txdac,
            self.tx_buffer_size,
            Some(false),
        )?);

        Ok(())
    }

    fn tx_buffer_push(&mut self, data: Vec<u8>) -> Result<(), ()> {
        let txbuf = self.txbuf.as_mut().ok_or(())?;
        txbuf.write(data);
        txbuf.push(None);
        Ok(())
    }
}

impl<'a> traits::TxCore<'a> for Pluto<'a> {
    fn init(
        in_tx_cyclic_buffer: Option<bool>,
        complex_data: bool,
        in_tx_channel_names_opt: &Option<Vec<String>>,
    ) -> traits::TxCoreInitResult {
        let in_tx_channel_names = in_tx_channel_names_opt.as_ref().ok_or(())?;
        let tx_complex_data = None;
        let tx_data_type = Some(datatypes::SdrDataType::Int16);
        let tx_cyclic_buffer = in_tx_cyclic_buffer.unwrap_or(false);
        let tx_output_byte_filename = "out.bin".to_owned();
        let txbuf = None;
        let tx_push_to_file = false;
        let n = if complex_data { 2u32 } else { 1u32 };
        let num_tx_channels = in_tx_channel_names.len() as u32;
        let tx_enabled_channels_range = num_tx_channels / n;
        let tx_enabled_channels = (0..tx_enabled_channels_range).collect::<Vec<u32>>();
        let tx_buffer_size = 1024;
        Ok((
            tx_complex_data,
            tx_data_type,
            tx_cyclic_buffer,
            tx_enabled_channels,
            tx_output_byte_filename,
            txbuf,
            tx_push_to_file,
            num_tx_channels,
            tx_buffer_size,
        ))
    }
    fn get_tx_complex_data(&self) -> bool {
        self.tx_complex_data.unwrap_or(self.complex_data)
    }

    fn get_tx_cyclic_buffer(&self) -> bool {
        todo!()
    }

    fn set_tx_cyclic_buffer(&self, _value: bool) {
        todo!()
    }

    fn get_num_tx_channels_enabled(&self) -> usize {
        self.tx_enabled_channels.len()
    }

    fn get_tx_channel_names(&self) -> Vec<&str> {
        todo!()
    }

    fn get_tx_enabled_channels(&self) -> Vec<i32> {
        todo!()
    }

    fn set_tx_enabled_channels(&self, _value: Vec<i32>) {
        todo!()
    }

    fn tx_destroy_buffer(&mut self) {
        todo!()
    }

    fn tx(&mut self, data_opt: Option<Vec<Vec<datatypes::PlutoComplex>>>) -> Result<(), ()>
    {
        let txdac = self.txdac.as_ref().ok_or(())?;
        let iio_context = self.context.as_ref().get_iio_context();
        let iio_txdac = iio_context.find_device(&self.tx_data_device_name)?;
        let channels = txdac.get_channels(iio_txdac)?;

        if self.tx_enabled_channels.is_empty() && data_opt.is_none() {
            return Err(());
        };

        if self.tx_enabled_channels.is_empty() {
            for mut channel in channels {
                if channel.get_is_output() {
                    let channel_name = channel.get_name().ok_or(())?.to_owned();
                    let attrs = channel.get_attrs();
                    let iio_channel = iio_txdac.find_channel(channel_name.as_str(), true)?;
                    let entry = attrs.get_mut("raw").ok_or(())?;
                    let result = entry.set_value(iio_channel, "0");
                    return if result <= 0 { Err(()) } else { Ok(()) }
                }
            }

            return Err(());
        };

        if self.tx_data_type.is_none() {
            let channel_name = self.tx_channel_names.as_ref().ok_or(())?
                [self.tx_enabled_channels[0] as usize]
                .as_str();
            let iio_channel = iio_txdac.find_channel(channel_name, true)?;
            let _channel_format = iio_channel.get_data_format().ok_or(())?;
            // TODO: Handle this logic and add data types

            self.tx_data_type = Some(datatypes::SdrDataType::Int16);
        };

        // This used to be a .unwrap(), may it be remembered as the last one in this program
        let _tx_data_type = self.tx_data_type.as_ref().ok_or(())?;

        let data = data_opt.ok_or(())?;
        let mut _index = 0;
        let num_tx_channels_enabled = <Pluto<'a> as TxCore<'a>>::get_num_tx_channels_enabled(self);

        if data.len() != num_tx_channels_enabled {
            return Err(());
        }

        if self.txbuf.is_some() && self.tx_cyclic_buffer {
            return Err(());
        };

        // TODO: C'est pas fini hein petit bâtard

        let (stride, out_data): (usize, Vec<u8>) =
            if <Pluto<'a> as TxCore<'a>>::get_tx_complex_data(self) {
                let stride = num_tx_channels_enabled * 2;
                let mut out_data = vec![0; stride * data[0].len() * 2];
                for (channel_index, channel_data) in data.iter().enumerate() {
                    let real_values = channel_data.iter().map(|f| (f.re as i128) << 14);
                    let imaginary_values = channel_data.iter().map(|f| (f.im as i128) << 14);
                    for (index, real) in real_values.enumerate() {
                        let real_bytes = (real as u16).to_le_bytes();
                        out_data[((channel_index * 2 + index) * 2) * 2] = real_bytes[0];
                        out_data[((channel_index * 2 + index) * 2) * 2 + 1] = real_bytes[1];
                    }
                    for (index, imaginary) in imaginary_values.enumerate() {
                        let imaginary_bytes = (imaginary as u16).to_le_bytes();
                        out_data[((channel_index * 2 + index) * 2 + 1) * 2] = imaginary_bytes[0];
                        out_data[((channel_index * 2 + index) * 2 + 1) * 2 + 1] =
                            imaginary_bytes[1];
                    }
                }

                (stride, out_data)
            } else {
                let _stride = num_tx_channels_enabled;
                todo!()
            };

        if self.txbuf.is_none() {
            <Pluto<'a> as DDS<'a>>::disable_dds(self)?;
            self.tx_buffer_size = out_data.len() / stride / 2;
            <Pluto<'a> as TxCore<'a>>::tx_init_channels(self)?;
        };

        // println!("TX buffer size = {}", self.tx_buffer_size);

        if out_data.len() / stride / 2 != self.tx_buffer_size {
            return Err(());
        };

        // println!("Data = {:?}, {}", out_data, out_data.len());

        if self.tx_push_to_file {
            // println!("HELO {}", self.tx_output_byte_filename.clone());
            let mut f =
                std::fs::File::create(self.tx_output_byte_filename.clone()).map_err(|_| ())?;
            f.write(out_data.as_slice()).map_err(|_| ())?;
        } else {
            <Pluto<'a> as TxCore<'a>>::tx_buffer_push(self, out_data)?;
        };

        Ok(())
    }

    fn tx_buffer_push(&mut self, data: Vec<u8>) -> Result<(), ()> {
        <Pluto<'a> as types::traits::Ctx>::tx_buffer_push(self, data)
    }

    fn tx_init_channels(&mut self) -> Result<(), ()> {
        <Pluto<'a> as types::traits::Ctx>::tx_init_channels(self)
    }
}

impl<'a> traits::TxDef<'a> for Pluto<'a> {
    fn init(
        context: &types::context::Context,
        tx_data_device_name: Option<&String>,
        tx_channel_names: Option<Vec<String>>,
    ) -> traits::TxOrRxDefInitResult {
        let txdac_opt = if let Some(device_name) = tx_data_device_name {
            Some(context.find_device(device_name.as_str())?)
        } else {
            None
        };

        let channel_names = if let Some(txdac) = &txdac_opt
            && tx_channel_names.is_none()
        {
            let iio_context = context.get_iio_context();
            let iio_device =
                iio_context.find_device(tx_data_device_name.as_ref().ok_or(())?.as_str())?;
            let tx_channels = txdac.get_channels(iio_device)?;
            Some(
                tx_channels
                    .iter()
                    .filter(|f| f.get_is_scan_element())
                    .map(|f| f.get_id().to_owned())
                    .collect::<Vec<String>>(),
            )
        } else {
            tx_channel_names
        };

        Ok((txdac_opt, channel_names))
    }

    fn get_tx_data_device_name(&self) {
        todo!()
    }
}

impl<'a> traits::RxTxDef<'a> for Pluto<'a> {
    fn init() -> (bool, bool) {
        (false, false)
    }
}

impl<'a> traits::AD9364<'a> for Pluto<'a> {
    fn get_filter(&self) -> Vec<i32> {
        todo!()
    }

    fn set_filter(&self, _filename: &str) {
        todo!()
    }

    fn get_loopback(&self) -> Vec<i32> {
        todo!()
    }

    fn set_loopback(&self, _value: datatypes::Loopback) {
        todo!()
    }

    fn get_gain_control_mode_chan0(&'a self) -> Result<String, ()> {
        traits::Attribute::get_iio_attr_str(
            self,
            "voltage0",
            "gain_control_mode",
            Some(false),
            None,
        )
    }

    fn set_gain_control_mode_chan0(&'a self, value: &'a str) -> Result<(), ()> {
        traits::Attribute::set_iio_attr_str(
            self,
            "voltage0",
            "gain_control_mode",
            Some(false),
            value,
            None,
        )
    }

    fn get_rx_hardwaregain_chan0(&self) -> Result<f32, ()> {
        traits::Attribute::get_iio_attr_float(self, "voltage0", "hardwaregain", Some(false), None)
            .map(|f| f as f32)
    }

    fn set_rx_hardwaregain_chan0(&self, value: f32) -> Result<(), ()> {
        traits::Attribute::set_iio_attr_float(
            self,
            "voltage0",
            "hardwaregain",
            Some(false),
            value,
            None,
        )
    }

    fn get_tx_hardwaregain_chan0(&self) -> Result<f32, ()> {
        traits::Attribute::get_iio_attr_float(self, "voltage0", "hardwaregain", Some(true), None)
            .map(|f| f as f32)
    }

    fn set_tx_hardwaregain_chan0(&self, value: f32) -> Result<(), ()> {
        traits::Attribute::set_iio_attr_float(
            self,
            "voltage0",
            "hardwaregain",
            Some(true),
            value,
            None,
        )
    }

    fn get_rx_rf_bandwidth(&self) -> Result<u32, ()> {
        traits::Attribute::get_iio_attr_int(self, "voltage0", "rf_bandwidth", Some(false), None)
            .map(|f| f as u32)
    }

    fn set_rx_rf_bandwidth(&self, value: u32) -> Result<(), ()> {
        traits::Attribute::set_iio_attr_int(
            self,
            "voltage0",
            "rf_bandwidth",
            Some(false),
            value,
            None,
        )
    }

    fn get_tx_rf_bandwidth(&self) -> Result<u32, ()> {
        traits::Attribute::get_iio_attr_int(self, "voltage0", "rf_bandwidth", Some(true), None)
            .map(|f| f as u32)
    }

    fn set_tx_rf_bandwidth(&self, value: u32) -> Result<(), ()> {
        traits::Attribute::set_iio_attr_int(
            self,
            "voltage0",
            "rf_bandwidth",
            Some(true),
            value,
            None,
        )
    }

    fn get_sample_rate(&self) -> Result<u32, ()> {
        traits::Attribute::get_iio_attr_int(
            self,
            "voltage0",
            "sampling_frequency",
            Some(false),
            None,
        )
        .map(|f| f as u32)
    }

    fn set_sample_rate(&mut self, value: u32) -> Result<(), ()> {
        static CURRENT_SAMPLING_THRESH: u32 = 2083333;
        if value < 521_000 {
            return Err(());
        }
        let (dec, fir) = match value {
            x if x <= 20_000_000 => (4, fir::FIR_1.as_slice()),
            x if x <= 40_000_000 => (2, fir::FIR_2.as_slice()),
            x if x <= 53_333_333 => (2, fir::FIR_3.as_slice()),
            _ => (2, fir::FIR_4.as_slice()),
        };

        let current_sampling_freq = self.get_sample_rate()?;
        let fir_old = traits::Attribute::get_iio_attr_str(
            self,
            "out",
            "voltage_filter_fir_en",
            Some(false),
            None,
        )?;
        let fir_number = fir_old.parse::<u32>().map_err(|_| ())?;

        if fir_number != 0 {
            if current_sampling_freq <= CURRENT_SAMPLING_THRESH {
                let sampling_frequency_fixed = 3_000_000;
                traits::Attribute::set_iio_attr_int(
                    self,
                    "voltage0",
                    "sampling_frequency",
                    Some(false),
                    sampling_frequency_fixed,
                    None,
                )?;
            }
            let fir0 = 0;
            traits::Attribute::set_iio_attr_int(
                self,
                "out",
                "voltage_filter_fir_en",
                Some(false),
                fir0,
                None,
            )?;
        }

        let mut fir_config_str = format!("RX 3 GAIN -6 DEC {}\nTX 3 GAIN 0 INT {}\n", dec, dec);
        for attr in fir {
            fir_config_str = format!("{}{},{}\n", fir_config_str, attr, attr);
        }
        fir_config_str = format!("{}\n", fir_config_str);
        traits::Attribute::set_iio_dev_attr_str(
            self,
            "filter_fir_config",
            fir_config_str.as_str(),
            None,
        )?;

        if value <= CURRENT_SAMPLING_THRESH {
            let readbuf = traits::Attribute::get_iio_dev_attr_str(self, "tx_path_rates", None)?;
            let dacrate = readbuf.split(" ").collect::<Vec<&str>>()[1]
                .split(":")
                .collect::<Vec<&str>>()[1];
            let txrate = readbuf.split(" ").collect::<Vec<&str>>()[5]
                .split(":")
                .collect::<Vec<&str>>()[1];

            let max_rate = ((dacrate.parse::<u32>().map_err(|_| ())?
                / txrate.parse::<u32>().map_err(|_| ())?)
                * 16) as usize;
            if max_rate < fir.len() {
                let sampling_frequency_fixed = 3_000_000;
                traits::Attribute::set_iio_attr_int(
                    self,
                    "voltage0",
                    "sampling_frequency",
                    Some(false),
                    sampling_frequency_fixed,
                    None,
                )?;
            }
            traits::Attribute::set_iio_attr_int(
                self,
                "out",
                "voltage_filter_fir_en",
                Some(false),
                1,
                None,
            )?;
            traits::Attribute::set_iio_attr_int(
                self,
                "voltage0",
                "sampling_frequency",
                Some(false),
                value,
                None,
            )?;
        } else {
            traits::Attribute::set_iio_attr_int(
                self,
                "voltage0",
                "sampling_frequency",
                Some(false),
                value,
                None,
            )?;
            traits::Attribute::set_iio_attr_int(
                self,
                "out",
                "voltage_filter_fir_en",
                Some(false),
                1,
                None,
            )?;
        }

        Ok(())
    }

    fn get_rx_lo(&self) -> Result<u64, ()> {
        traits::Attribute::get_iio_attr_int(self, "altvoltage0", "frequency", Some(true), None)
            .map(|f| f as u64)
    }

    fn set_rx_lo(&self, value: u64) -> Result<(), ()> {
        traits::Attribute::set_iio_attr_int(
            self,
            "altvoltage0",
            "frequency",
            Some(true),
            value,
            None,
        )
    }

    fn get_tx_lo(&self) -> Result<u64, ()> {
        traits::Attribute::get_iio_attr_int(self, "altvoltage1", "frequency", Some(true), None)
            .map(|f| f as u64)
    }

    fn set_tx_lo(&self, value: u64) -> Result<(), ()> {
        traits::Attribute::set_iio_attr_int(
            self,
            "altvoltage1",
            "frequency",
            Some(true),
            value,
            None,
        )
    }

    fn init() -> traits::Ad9364InitResult {
        let complex_data = Some(true);
        let rx_channel_names = Some(["voltage0".to_owned(), "voltage1".to_owned()].to_vec());
        let tx_channel_names = Some(["voltage0".to_owned(), "voltage1".to_owned()].to_vec());
        let control_device_name = "ad9361-phy".to_owned();
        let rx_data_device_name = "cf-ad9361-lpc".to_owned();
        let tx_data_device_name = "cf-ad9361-dds-core-lpc".to_owned();
        (
            complex_data,
            rx_channel_names,
            tx_channel_names,
            control_device_name,
            rx_data_device_name,
            tx_data_device_name,
        )
    }
}

impl<'a> traits::DecIntFPGAFilter for Pluto<'a> {
    fn get_rates(
        &self,
        dev: &types::device_trigger::DeviceTrigger,
        output: bool,
    ) -> Result<Vec<u32>, ()> {
        let sfa = traits::Attribute::get_iio_attr_str(
            self,
            "voltage0",
            "sampling_frequency_available",
            Some(output),
            Some(dev),
        )?;
        let rates_replaced = sfa.trim().replace("[", "").replace("]", "");
        let rates = rates_replaced.split(" ");
        let mut my_vec = vec![];
        for rate in rates {
            my_vec.push(rate.parse::<u32>().map_err(|_| ())?);
        }
        Ok(my_vec)
    }

    fn get_rx_dec8_filter_en(&self) -> Result<bool, ()> {
        let rxadc = self.rxadc.as_ref().ok_or(())?;
        let rates = self.get_rates(rxadc, false)?;
        let sf_string = traits::Attribute::get_iio_attr_str(
            self,
            "voltage0",
            "sampling_frequency",
            Some(false),
            Some(rxadc),
        )?;
        let bytes_with_dot = sf_string
            .as_bytes()
            .iter()
            .take_while(|f| f.is_ascii_digit())
            .copied()
            .collect::<Vec<u8>>();
        let bytes_string = String::from_utf8_lossy(bytes_with_dot.as_slice()).to_string();
        let sf = bytes_string.parse::<u32>().map_err(|_| ())?;
        let mut min_rate = u32::MAX;
        for rate in rates {
            if rate < min_rate {
                min_rate = rate;
            }
        }
        Ok(min_rate == sf)
    }

    fn set_rx_dec8_filter_en(&self, value: bool) -> Result<(), ()> {
        let rxadc = self.rxadc.as_ref().ok_or(())?;
        let rates = self.get_rates(rxadc, false)?;
        if rates.len() < 2 {
            return Err(());
        }
        let sr = if value { rates[1] } else { rates[0] };
        traits::Attribute::set_iio_attr_str(
            self,
            "voltage0",
            "sampling_frequency",
            Some(false),
            sr.to_string().as_str(),
            Some(rxadc),
        )
    }

    fn get_tx_int8_filter_en(&self) -> Result<bool, ()> {
        let txdac = self.txdac.as_ref().ok_or(())?;
        let rates = self.get_rates(txdac, true)?;
        let sf_string = traits::Attribute::get_iio_attr_str(
            self,
            "voltage0",
            "sampling_frequency",
            Some(true),
            Some(txdac),
        )?;
        let bytes_with_dot = sf_string
            .as_bytes()
            .iter()
            .take_while(|f| f.is_ascii_digit())
            .copied()
            .collect::<Vec<u8>>();
        let bytes_string = String::from_utf8_lossy(bytes_with_dot.as_slice()).to_string();
        let sf = bytes_string.parse::<u32>().map_err(|_| ())?;
        let mut min_rate = u32::MAX;
        for rate in rates {
            if rate < min_rate {
                min_rate = rate;
            }
        }
        Ok(min_rate == sf)
    }

    fn set_tx_int8_filter_en(&self, value: bool) -> Result<(), ()> {
        let txdac = self.txdac.as_ref().ok_or(())?;
        let rates = self.get_rates(txdac, false)?;
        if rates.len() < 2 {
            return Err(());
        }
        let sr = if value { rates[1] } else { rates[0] };
        traits::Attribute::set_iio_attr_str(
            self,
            "voltage0",
            "sampling_frequency",
            Some(true),
            sr.to_string().as_str(),
            Some(txdac),
        )
    }
}
