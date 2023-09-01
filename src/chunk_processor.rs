use crate::{
    queues::{in_mem::InMemFootNoteQ, Queue},
    writers::{to_file::FileWriter, Writer},
};

/*
Special Char

This enum represents all possible characters which should be considered "special"
when encountered. Special characters are used to control program flow. For instance,
An open parentheses indicates the start of a foot note. A pound symbol indicates that
the program should dump all foot notes in memory at this location in the document.

We also implement the Into trait for this enum. This allows us to map ASCII character
codes to our enum, making it trivial to convert the bytes we are reading in.
*/

#[derive(Debug)]
enum SpecialChar {
    Open,
    Closed,
    Pound,
    NotSpecial,
}

impl Into<SpecialChar> for u8 {
    fn into(self) -> SpecialChar {
        match self {
            40 => SpecialChar::Open,
            41 => SpecialChar::Closed,
            35 => SpecialChar::Pound,
            _ => SpecialChar::NotSpecial,
        }
    }
}

/*
Chunk Processor

The Chunk Processor is the meat and potatoes of our application. It does a few things.
First it takes in a chunk of bytes from our FileProcessor. It will scan the chunk
for any special characters, handling any encountered. This handling is how we
separate footers from the main text. Once it is done processing a chunk, it will
write the data to the FileWriter who is in charge of outputting the result of the
program.
*/

pub struct ChunkProcessor {
    main_writer: FileWriter,
    brackets_deep: u64,
    foot_note_queue: InMemFootNoteQ,
    collected: Vec<u8>,
}
impl ChunkProcessor {
    pub fn new(main_writer: FileWriter) -> Self {
        let foot_note_queue = InMemFootNoteQ::new();
        return ChunkProcessor {
            main_writer,
            foot_note_queue,
            brackets_deep: 0,
            collected: Vec::new(),
        };
    }
    pub fn process_bytes(&mut self, data: Vec<u8>) {
        for char in data {
            let special_char: SpecialChar = char.into();
            match special_char {
                /*
                If we encounter an open bracket, flush the chars we have collected thus
                far into the main output, and start collecting the coming foot note.
                 */
                SpecialChar::Open => {
                    self.brackets_deep += 1;
                    if self.collected.len() > 0 {
                        self.main_writer.write(Some(self.collected.clone()));
                        self.collected = Vec::new();
                    }
                }
                /*
                If we encounter a closing bracket, add the currently collected foot note
                into the foot note queue for later appending to the output
                 */
                SpecialChar::Closed => {
                    self.brackets_deep -= 1;
                    if self.collected.len() > 0 {
                        let queue_number = self
                            .foot_note_queue
                            .add(Some(self.collected.clone()))
                            .unwrap();

                        self.main_writer
                            .write(Some(format!("^{}", queue_number).as_bytes().to_vec()));
                        self.collected = Vec::new();
                    }
                }
                /*
                If we encounter a pound symbol and are not within a foot note,
                dump all foot notes we have collected thus far into the output writer
                 */
                SpecialChar::Pound if (self.brackets_deep <= 0) => {
                    if self.collected.len() > 0 {
                        self.main_writer.write(Some(self.collected.clone()));
                        self.collected = Vec::new();
                    }
                    self.append_stored_footers();
                }
                /*
                If the char we are on is not a special symbol, or it is a pound
                but we are within a foot note, push the current char into our
                collected char vector.
                 */
                SpecialChar::NotSpecial | SpecialChar::Pound => {
                    self.collected.push(char);
                }
            }
        }
        /*
        Once we have iterated through, If we are not in a foot note, dump the
        currently collected chars into the output writer. This helps keep our
        collected vector small incase there is large gaps between foot notes.

        In that situation our collected vector could grow very large causing
        performance issues. currently this is still an issue if foot notes
        are gigantic. To fix this, one could instead of storing foot notes
        in memory, store them in a temp file.
         */
        if self.collected.len() > 0 {
            match self.brackets_deep {
                n if (n > 0) => {}
                _ => {
                    self.main_writer.write(Some(self.collected.clone()));
                    self.collected = Vec::new();
                }
            }
        }
    }

    /*
    Take the footers we have stored, and dump them to our output writer in a
    user readable format
     */
    pub fn append_stored_footers(&mut self) {
        self.main_writer.write(Some("\n".as_bytes().to_vec()));
        while let Some((mut foot_note, queue_number)) = self.foot_note_queue.pop_front() {
            let mut vec_to_write: Vec<u8> = Vec::new();
            vec_to_write.append(&mut format!("\nFN-{}:", queue_number).as_bytes().to_vec());
            vec_to_write.append(&mut foot_note);
            vec_to_write.append(&mut "\n\n".as_bytes().to_vec());

            self.main_writer.write(Some(vec_to_write));
        }
    }
}

/*
To interact with our chunk processor, we implement the writer trait so that
if need be we could swap it out for a different writer.
*/
impl Writer for ChunkProcessor {
    /*
    For the write function, all we do is check if the incoming option contains
    data or not. If it does, process the data. If it does not, there is no more
    text and we should dump the last of our foot notes.
     */
    fn write(&mut self, buf: Option<Vec<u8>>) {
        match buf {
            Some(data) => {
                self.process_bytes(data);
            }
            None => {
                self.append_stored_footers();
            }
        }
    }
}
