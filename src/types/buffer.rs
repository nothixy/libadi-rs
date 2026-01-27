use crate::types;

#[derive(Debug)]
pub struct Buffer {
    buffer: Box<iio::IIOBuffer>,
    length: usize,
    samples_count: usize,
}

impl Buffer {
    pub fn new(
        device: &iio::IIODevice,
        samples_count: usize,
        cyclic_opt: Option<bool>,
    ) -> Result<Buffer, ()> {
        let cyclic = cyclic_opt.unwrap_or(false);
        let length =
            samples_count * types::device_trigger::DeviceTrigger::get_sample_size(device) as usize;
        let buffer = device.create_buffer(samples_count, cyclic).ok_or(())?;
        Ok(Buffer {
            buffer,
            length,
            samples_count,
        })
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn refill(&mut self) -> isize {
        self.buffer.refill()
    }

    pub fn push(&mut self, samples_count_opt: Option<usize>) -> isize {
        let samples_count = samples_count_opt.unwrap_or(self.samples_count);
        self.buffer.push_partial(samples_count)
    }

    pub fn read(&self) -> Vec<u8> {
        let buffer_start = self.buffer.start();
        let buffer_end = self.buffer.end();
        let pointer_diff = unsafe { buffer_end.offset_from(buffer_start) } as usize;
        let slice = unsafe { std::slice::from_raw_parts(buffer_start, pointer_diff) };
        slice.to_vec()
    }

    pub fn write(&mut self, array: Vec<u8>) -> usize {
        let buffer_start = self.buffer.start();
        let buffer_end = self.buffer.end();
        let mut pointer_diff = unsafe { buffer_end.offset_from(buffer_start) } as usize;
        unsafe {
            std::ptr::write_bytes(buffer_start, 0, pointer_diff);
            if pointer_diff > array.len() {
                pointer_diff = array.len();
            }
            std::ptr::copy_nonoverlapping(array.as_ptr(), buffer_start, pointer_diff);
        }

        // for (i, item) in array.iter().enumerate().take(pointer_diff) {
        //     unsafe {
        //         *buffer_start.wrapping_add(i) = *item;
        //     };
        // }
        pointer_diff
    }

    pub fn cancel(&mut self) {
        self.buffer.cancel()
    }

    pub fn set_blocking_mode(&mut self, blocking: bool) -> Result<(), i32> {
        self.buffer.set_blocking_mode(blocking)
    }

    pub fn get_poll_fd(&mut self) -> i32 {
        self.buffer.get_poll_fd()
    }

    pub fn step(&mut self) -> i64 {
        self.buffer.step()
    }

    pub fn get_buffer(&mut self) -> &mut Box<iio::IIOBuffer> {
        &mut self.buffer
    }
}
