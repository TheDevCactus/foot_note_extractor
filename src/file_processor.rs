use std::{
    fs::File,
    io::{Error, Read},
};

use crate::{
    chunk_processor::ChunkProcessor,
    writers::{to_file::FileWriter, Writer},
};

/*
File Processor

The file processors job is to handle the ingress of data from a provided input file.
It reads in BUFFER_SIZE bytes at a time, passing these chunks into our Text Router
which handles the next step.

The reason we only grab a chunk of text at a time is for memory utilization purposes.
Say you had a file whose size was larger than the memory of the machine executing the
program. Best case this will result in performance hits as the machine needs to do
swaps between memory and disk. Worst case this situation will crash the program.

*/

pub const BUFFER_SIZE: usize = 100; //bytes
pub struct FileProcessor {
    input: File,
    text_router: ChunkProcessor,
    buffer: [u8; BUFFER_SIZE],
}
impl FileProcessor {
    pub fn new(input_file: File, output_file: File) -> Self {
        let main_writer = FileWriter::new(output_file);
        let footer_collector = ChunkProcessor::new(main_writer);
        return FileProcessor {
            input: input_file,
            text_router: footer_collector,
            buffer: [0; BUFFER_SIZE],
        };
    }
    pub fn process(&mut self) -> Result<Option<()>, Error> {
        match self.input.read(&mut self.buffer)? {
            0 => {
                self.text_router.write(None);
                return Ok(None);
            }
            n if (n < self.buffer.len()) => {
                let final_bytes = &self.buffer[0..n];
                self.text_router.write(Some(final_bytes.to_vec()));
                return Ok(Some(()));
            }
            _ => {
                self.text_router.write(Some(self.buffer.to_vec()));
                return Ok(Some(()));
            }
        }
    }
}
