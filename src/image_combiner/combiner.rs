use core::panic;

use crate::ImageDataErrors;

fn get_nth_arg(n: usize) -> String {
    match std::env::args().nth(n) {
        Some(e) => e,
        None => panic!("Please fill the right paths"),
    }
}

pub struct Args {
    pub image_1: String,
    pub image_2: String,
    pub output: String,
}

impl Args {
    pub fn new() -> Self {
        Args {
            image_1: get_nth_arg(1),
            image_2: get_nth_arg(2),
            output: get_nth_arg(3),
        }
    }
}

pub struct FloatingImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub name: String,
}

impl FloatingImage {
    pub fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = height * width * 4;
        let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());
        FloatingImage {
            width,
            height,
            data: buffer,
            name,
        }
    }
    pub fn set_data<'a>(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors<'a>> {
        if (data.len() > self.data.capacity()) {
            return Err(ImageDataErrors::BufferTooSmall);
        } else {
            self.data = data;
            Ok(())
        }
    }
}
