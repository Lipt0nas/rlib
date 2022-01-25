extern crate log;

pub struct Buffer {
    handle: u32,
    size: isize,
    buffer_type: u32,
    usage: u32,
}

impl Buffer {
    pub fn new(buffer_type: u32, usage: u32) -> Option<Buffer> {
        let mut handle: u32 = 0;

        unsafe {
            gl::GenBuffers(1, &mut handle);
        }

        if handle == 0 {
            None
        } else {
            Some(Buffer {
                handle,
                size: 0,
                buffer_type,
                usage,
            })
        }
    }

    pub fn new_with_capacity(buffer_type: u32, usage: u32, capacity: u32) -> Option<Buffer> {
        let mut handle: u32 = 0;

        unsafe {
            gl::GenBuffers(1, &mut handle);
            gl::BindBuffer(buffer_type, handle);
            gl::BufferData(buffer_type, capacity as isize, std::ptr::null(), usage);
        }

        if handle == 0 {
            None
        } else {
            Some(Buffer {
                handle,
                size: capacity as isize,
                buffer_type,
                usage,
            })
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.buffer_type, self.handle);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.buffer_type, 0);
        }
    }

    pub fn set_data(&mut self, data: &[f32]) {
        let byte_count = (data.len() * std::mem::size_of::<f32>()) as isize;
        let ptr = data.as_ptr() as *const std::os::raw::c_void;

        self.size = byte_count;

        self.bind();
        unsafe {
            gl::BufferData(self.buffer_type, byte_count, std::ptr::null(), self.usage);
            gl::BufferData(self.buffer_type, byte_count, ptr, self.usage);
        }
    }

    pub fn copy_data(&self, data: &[f32], offset: isize) {
        let mut byte_count = (data.len() * std::mem::size_of::<f32>()) as isize;
        let ptr = data.as_ptr() as *const std::os::raw::c_void;

        if (offset + byte_count) > self.size {
            byte_count = self.size - offset;
        }

        self.bind();
        unsafe {
            gl::BufferSubData(self.buffer_type, offset, byte_count, ptr);
        }
    }

    pub fn copy_data_part(&self, data: &[f32], src_size: isize, dst_offset: isize) {
        let mut byte_count = (src_size as usize * std::mem::size_of::<f32>()) as isize;
        let ptr = data.as_ptr() as *const std::os::raw::c_void;

        if (dst_offset + byte_count) > self.size {
            byte_count = self.size - dst_offset;
        }

        info!("{}", byte_count);

        self.bind();
        unsafe {
            gl::BufferSubData(self.buffer_type, dst_offset, byte_count, ptr);
        }
    }

    pub fn get_size(&self) -> isize {
        self.size
    }

    pub fn get_type(&self) -> u32 {
        self.buffer_type
    }

    pub fn get_usage(&self) -> u32 {
        self.usage
    }

    pub fn get_handle(&self) -> u32 {
        self.handle
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        info!("Dropping buffer");
        if self.handle != 0 {
            unsafe {
                gl::DeleteBuffers(1, &self.handle);
            }
        }
    }
}
