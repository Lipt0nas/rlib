use std::cell::Cell;
use std::rc::Rc;

extern crate log;

pub(crate) struct NativeBuffer {
    handle: u32,
    size: Cell<isize>,
    buffer_type: u32,
    usage: u32,
}

#[derive(Clone)]
pub struct Buffer {
    pub(crate) handle: Rc<NativeBuffer>,
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
                handle: Rc::new(NativeBuffer {
                    handle,
                    size: Cell::new(0),
                    buffer_type,
                    usage,
                }),
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
                handle: Rc::new(NativeBuffer {
                    handle,
                    size: Cell::new(capacity as isize),
                    buffer_type,
                    usage,
                }),
            })
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.handle.buffer_type, self.handle.handle);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.handle.buffer_type, 0);
        }
    }

    pub fn set_data(&mut self, data: &[f32]) {
        let byte_count = (data.len() * std::mem::size_of::<f32>()) as isize;
        let ptr = data.as_ptr() as *const std::os::raw::c_void;

        self.handle.size.set(byte_count);

        self.bind();
        unsafe {
            gl::BufferData(
                self.handle.buffer_type,
                byte_count,
                std::ptr::null(),
                self.handle.usage,
            );
            gl::BufferData(self.handle.buffer_type, byte_count, ptr, self.handle.usage);
        }
    }

    pub fn set_data_u32(&mut self, data: &[u32]) {
        let byte_count = (data.len() * std::mem::size_of::<u32>()) as isize;
        let ptr = data.as_ptr() as *const std::os::raw::c_void;

        self.handle.size.set(byte_count);

        self.bind();
        unsafe {
            gl::BufferData(
                self.handle.buffer_type,
                byte_count,
                std::ptr::null(),
                self.handle.usage,
            );
            gl::BufferData(self.handle.buffer_type, byte_count, ptr, self.handle.usage);
        }
    }

    pub fn copy_data(&self, data: &[f32], offset: isize) {
        let mut byte_count = (data.len() * std::mem::size_of::<f32>()) as isize;
        let ptr = data.as_ptr() as *const std::os::raw::c_void;

        if (offset + byte_count) > self.handle.size.get() {
            byte_count = self.handle.size.get() - offset;
        }

        self.bind();
        unsafe {
            gl::BufferSubData(self.handle.buffer_type, offset, byte_count, ptr);
        }
    }

    pub fn copy_data_part(&self, data: &[f32], src_size: isize, dst_offset: isize) {
        let mut byte_count = (src_size as usize * std::mem::size_of::<f32>()) as isize;
        let ptr = data.as_ptr() as *const std::os::raw::c_void;

        if (dst_offset + byte_count) > self.handle.size.get() {
            byte_count = self.handle.size.get() - dst_offset;
        }

        self.bind();
        unsafe {
            gl::BufferSubData(self.handle.buffer_type, dst_offset, byte_count, ptr);
        }
    }

    pub fn get_size(&self) -> isize {
        self.handle.size.get()
    }

    pub fn get_type(&self) -> u32 {
        self.handle.buffer_type
    }

    pub fn get_usage(&self) -> u32 {
        self.handle.usage
    }

    pub fn get_handle(&self) -> u32 {
        self.handle.handle
    }
}
