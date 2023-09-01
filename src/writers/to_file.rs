use std::{fs::File, io::Write};

use super::Writer;

/*
File Writer

The File writer is simple enough. It takes in a char buffer through
the writer trait, and outputs the chunks to a given output file.
*/
pub struct FileWriter {
    output_file: File,
}
impl FileWriter {
    pub fn new(output_file: File) -> Self {
        return FileWriter { output_file };
    }
}
impl Writer for FileWriter {
    fn write(&mut self, char: Option<Vec<u8>>) {
        let write_res = self.output_file.write(&char.unwrap());
        if let Err(err) = write_res {
            println!("Error Occurred While Writing To Output: {:?}", err);
        }
    }
}
