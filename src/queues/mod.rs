pub mod in_mem;

/*
The Queue Trait

The queue trait describes methods for interacting with a Queue data structure.
All we need is a way to add to the back (add) and take from the front (pop_front).

For return types, The add function returns the "queue number" of the item just added.
This isn't necessarily the index in the queue. Think of it more like the deli tickets
at a grocery store. Queue numbers will always be ascending but may not start at 0.
*/
pub trait Queue {
    fn add(&mut self, item: Option<Vec<u8>>) -> Option<usize>;
    fn pop_front(&mut self) -> Option<(Vec<u8>, usize)>;
}
