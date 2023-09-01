pub mod to_file;

/*
The Writer trait describes a method of passing a buffer of chars from one
place to another.

For example, We have a "FileWriter" which passes these chunks into a output
file. But one could just as easily create a "StandardOutWriter" which would
output to stdout. All that would need to be done is implementing the Writer
trait for that "StandardOutWriter" so that the program would not need to change
to start utilizing this new writer.
*/
pub trait Writer {
    fn write(&mut self, char: Option<Vec<u8>>);
}
