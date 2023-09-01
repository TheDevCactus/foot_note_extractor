use std::{
    collections::VecDeque,
    env,
    fs::{self},
};

use file_processor::FileProcessor;

mod chunk_processor;
mod file_processor;
mod queues;
mod writers;

/*
Foot Guy

This program takes in a input file, and formats the foot notes in a
user friendly manner. Text wrapped in parenthesis will become a foot note.
A Pound symbol anywhere in the text will tell the program to append the
currently stored foot notes here in the text.
*/

fn main() {
    let io_files_res = get_io_files_from_args();
    if let Err(err) = io_files_res {
        match err {
            IOFileError::Input => {
                println!("Error while initializing input...");
            }
            IOFileError::Output => {
                println!("Error while initializing output...");
            }
        }
    }
    let (input, output) = io_files_res.ok().unwrap();

    let mut file_processor = FileProcessor::new(input, output);
    loop {
        match file_processor.process() {
            Err(err) => {
                println!("\n Error encountered while processing file \n {:?} \n", err);
                break;
            }
            Ok(None) => {
                break;
            }
            _ => {}
        }
    }

    println!("Foot notes extracted and formatted.")
}

/*
Errors specific to attempting to read the io file arguments
If an error is encountered trying to read the input file argument, or
read the file that argument points to, we will throw a IOFileError::Input.
You can deduce when IOFileError::Output would be thrown.
*/
#[derive(Clone, Copy)]
enum IOFileError {
    Input,
    Output,
}
/*
Read from the arguments called with this application.
The argument at index one is our input file path.
The argument at index two is our output file path.
the first index is the program itself which is unhelpful for us.
*/
fn get_io_files_from_args() -> Result<(fs::File, fs::File), IOFileError> {
    let mut args: VecDeque<String> = env::args().collect();
    // first arg is the program itself.
    args.pop_front();

    let input_file_path = args.pop_front().ok_or(IOFileError::Input)?;
    let output_file_path = args.pop_front().ok_or(IOFileError::Output)?;

    let input_reader = fs::OpenOptions::new()
        .read(true)
        .open(input_file_path)
        .map_err(|_| IOFileError::Input)?;

    let output_writer = fs::OpenOptions::new()
        .read(false)
        .append(true)
        .open(output_file_path)
        .map_err(|_| IOFileError::Output)?;

    return Ok((input_reader, output_writer));
}
