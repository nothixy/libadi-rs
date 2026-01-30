use crate::datatypes;
use crate::types;

pub type RxCoreInitResult = Result<
    (
        Option<bool>,
        datatypes::SdrDataType,
        datatypes::SdrDataType,
        u32,
        usize,
        Vec<u32>,
        String,
        Option<types::buffer::Buffer>,
        bool,
        bool,
        bool,
        u32,
    ),
    (),
>;

pub type TxCoreInitResult = Result<
    (
        Option<bool>,
        Option<datatypes::SdrDataType>,
        bool,
        Vec<u32>,
        String,
        Option<types::buffer::Buffer>,
        bool,
        u32,
        usize,
    ),
    (),
>;

pub type TxOrRxDefInitResult = Result<
    (
        Option<Box<types::device_trigger::DeviceTrigger>>,
        Option<Vec<String>>,
    ),
    (),
>;

pub type Ad9364InitResult = (
    Option<bool>,
    Option<Vec<String>>,
    Option<Vec<String>>,
    String,
    String,
    String,
);

pub trait ContextManager<'a> {
    fn get_ctx(&self) -> &'a types::device_trigger::DeviceTrigger;
    fn init(
        uri: Option<String>,
        device_name: Option<&str>,
    ) -> Result<Box<types::context::Context<'a>>, ()>;
}

pub trait DecIntFPGAFilter {
    fn get_rates(
        &self,
        dev: &types::device_trigger::DeviceTrigger,
        output: bool,
    ) -> Result<Vec<u32>, ()>;
    fn get_rx_dec8_filter_en(&self) -> Result<bool, ()>;
    fn set_rx_dec8_filter_en(&self, value: bool) -> Result<(), ()>;
    fn get_tx_int8_filter_en(&self) -> Result<bool, ()>;
    fn set_tx_int8_filter_en(&self, value: bool) -> Result<(), ()>;
}

pub trait SharedDef<'a> {
    fn get_complex_data(&self);
    fn get_control_device_name(&self);
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
    >;
    fn post_init(&self);
}

pub trait Attribute<'a> {
    fn set_iio_attr_str(
        &'a self,
        channel_name: &str,
        attr_name: &'a str,
        output: Option<bool>,
        value: &'a str,
        ctrl: Option<&'a types::device_trigger::DeviceTrigger>,
    ) -> Result<(), ()>;
    fn get_iio_attr_str(
        &'a self,
        channel_name: &'a str,
        attr_name: &'a str,
        output: Option<bool>,
        ctrl: Option<&'a types::device_trigger::DeviceTrigger>,
    ) -> Result<String, ()>;
    fn set_iio_dev_attr_str(
        &mut self,
        attr_name: &str,
        value: &str,
        ctrl: Option<&mut types::device_trigger::DeviceTrigger>,
    ) -> Result<(), ()>;
    fn get_iio_dev_attr_str(
        &mut self,
        attr_name: &str,
        ctrl: Option<&mut types::device_trigger::DeviceTrigger>,
    ) -> Result<String, ()>;
    fn set_iio_debug_attr_str(
        &self,
        attr_name: &str,
        value: &str,
        ctrl: Option<&types::device_trigger::DeviceTrigger>,
    ) -> Result<(), ()>;
    fn get_iio_debug_attr_str(
        &self,
        attr_name: &str,
        ctrl: Option<&types::device_trigger::DeviceTrigger>,
    ) -> Result<String, ()>;
    fn set_iio_attr_int<T: Into<i128>>(
        &'a self,
        channel_name: &str,
        attr_name: &'a str,
        output: Option<bool>,
        value: T,
        ctrl: Option<&'a types::device_trigger::DeviceTrigger>,
    ) -> Result<(), ()>;
    fn get_iio_attr_int(
        &'a self,
        channel_name: &'a str,
        attr_name: &'a str,
        output: Option<bool>,
        ctrl: Option<&'a types::device_trigger::DeviceTrigger>,
    ) -> Result<i128, ()>;
    fn set_iio_attr_float<T: Into<f64>>(
        &'a self,
        channel_name: &str,
        attr_name: &'a str,
        output: Option<bool>,
        value: T,
        ctrl: Option<&'a types::device_trigger::DeviceTrigger>,
    ) -> Result<(), ()>;
    fn get_iio_attr_float(
        &'a self,
        channel_name: &'a str,
        attr_name: &'a str,
        output: Option<bool>,
        ctrl: Option<&'a types::device_trigger::DeviceTrigger>,
    ) -> Result<f64, ()>;
}

pub trait RxTxCommon<'a> {
    fn init(complex_data: Option<bool>) -> bool;
    fn annotate(&self, data: Vec<f32>, channel_names: Vec<&str>, enabled_channels: Vec<&str>);
}

pub trait DDS<'a> {
    fn init() -> bool;
    fn update_dds(&self, attr: &str, value: Vec<types::traits::DdsValue>) -> Result<(), ()>;
    fn read_dds(&self, attr: &str) -> &str;
    fn disable_dds(&self) -> Result<(), ()>;
    fn get_dds_frequencies(&self) -> &str;
    fn set_dds_frequencies(&self, value: &str);
    fn get_dds_scales(&self) -> &str;
    fn set_dds_scales(&self, value: &str);
    fn get_dds_phases(&self) -> &str;
    fn set_dds_phases(&self, value: &str);
    fn get_dds_enabled(&self) -> &str;
    fn set_dds_enabled(&self, value: Vec<bool>) -> Result<(), ()>;
    fn dds_single_tone(&self, frequency: i32, scale: f32, channel: i32);
    fn dds_dual_tone(
        &self,
        frequency1: i32,
        scale1: f32,
        frequncy2: i32,
        scale2: f32,
        channel: i32,
    );
}

pub trait TxCore<'a> {
    fn init(
        in_tx_cyclic_buffer: Option<bool>,
        complex_data: bool,
        in_tx_channel_names_opt: &Option<Vec<String>>,
    ) -> TxCoreInitResult;
    fn get_tx_complex_data(&self) -> bool;
    fn get_tx_cyclic_buffer(&self) -> bool;
    fn set_tx_cyclic_buffer(&self, value: bool);
    fn get_num_tx_channels_enabled(&self) -> usize;
    fn get_tx_channel_names(&self) -> Vec<&str>;
    fn get_tx_enabled_channels(&self) -> Vec<i32>;
    fn set_tx_enabled_channels(&self, value: Vec<i32>);
    fn tx_destroy_buffer(&mut self);
    fn tx(&mut self, data_opt: Option<Vec<Vec<datatypes::PlutoComplex>>>) -> Result<(), ()>;
    fn tx_buffer_push(&mut self, data: Vec<u8>) -> Result<(), ()>;
    fn tx_init_channels(&mut self) -> Result<(), ()>;
}

pub trait RxCore<'a> {
    fn init(
        in_rx_buffer_size: Option<usize>,
        complex_data: bool,
        in_rx_channel_names_opt: &Option<Vec<String>>,
    ) -> RxCoreInitResult;
    fn get_rx_complex_data(&self) -> bool;
    fn get_rx_channel_names(&self) -> Vec<&str>;
    fn get_rx_annotated(&self) -> bool;
    fn set_rx_annotated(&mut self, value: bool);
    fn get_rx_output_type(&self) -> &str;
    fn set_rx_output_type(&mut self, value: &str);
    fn get_rx_buffer_size(&self) -> usize;
    fn set_rx_buffer_size(&mut self, value: usize);
    fn get_rx_enabled_channels(&self) -> Vec<i32>;
    fn set_rx_enabled_channels(&self, value: Vec<i32>);
    fn get_num_rx_channels_enabled(&self) -> u32;
    fn rx_destroy_buffer(&mut self);
    fn get_rx_channel_scales(&self) -> Vec<f32>;
    fn get_rx_channel_offsets(&self) -> Vec<f32>;
    fn rx_unbuffered_data(&self) -> Vec<Vec<f32>>;
    fn rx_complex(&mut self) -> Result<Vec<Vec<datatypes::PlutoComplex>>, ()>;
    fn rx_non_complex(&self) -> Vec<Vec<f32>>;
    fn rx(&mut self) -> Vec<Vec<f32>>;
    fn rx_init_channels(&mut self) -> Result<(), ()>;
    fn rx_buffered_data(&mut self) -> Result<Vec<Vec<i128>>, ()>;
}

pub trait RxDef<'a> {
    fn init(
        context: &types::context::Context,
        rx_data_device_name: Option<&String>,
        rx_channel_names: Option<Vec<String>>,
    ) -> TxOrRxDefInitResult;
    fn get_tx_data_device_name(&self) -> &str;
}

pub trait TxDef<'a> {
    fn init(
        context: &types::context::Context,
        tx_data_device_name: Option<&String>,
        tx_channel_names: Option<Vec<String>>,
    ) -> TxOrRxDefInitResult;
    fn get_tx_data_device_name(&self);
}

pub trait RxTxDef<'a> {
    fn init() -> (bool, bool);
}

pub trait AD9364<'a> {
    fn init() -> Ad9364InitResult;
    fn get_filter(&self) -> Vec<i32>;
    fn set_filter(&self, filename: &str);
    fn get_loopback(&self) -> Vec<i32>;
    fn set_loopback(&self, value: datatypes::Loopback);
    fn get_gain_control_mode_chan0(&'a self) -> Result<String, ()>;
    fn set_gain_control_mode_chan0(&'a self, value: &'a str) -> Result<(), ()>;
    fn get_rx_hardwaregain_chan0(&self) -> Result<f32, ()>;
    fn set_rx_hardwaregain_chan0(&self, value: f32) -> Result<(), ()>;
    fn get_tx_hardwaregain_chan0(&self) -> Result<f32, ()>;
    fn set_tx_hardwaregain_chan0(&self, value: f32) -> Result<(), ()>;
    fn get_rx_rf_bandwidth(&self) -> Result<u32, ()>;
    fn set_rx_rf_bandwidth(&self, value: u32) -> Result<(), ()>;
    fn get_tx_rf_bandwidth(&self) -> Result<u32, ()>;
    fn set_tx_rf_bandwidth(&self, value: u32) -> Result<(), ()>;
    fn get_sample_rate(&self) -> Result<u32, ()>;
    fn set_sample_rate(&mut self, value: u32) -> Result<(), ()>;
    fn get_rx_lo(&self) -> Result<u64, ()>;
    fn set_rx_lo(&self, value: u64) -> Result<(), ()>;
    fn get_tx_lo(&self) -> Result<u64, ()>;
    fn set_tx_lo(&self, value: u64) -> Result<(), ()>;
}
