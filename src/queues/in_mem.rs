use std::collections::VecDeque;

use super::Queue;

/*
In Memory Foot Note Queue

The In Memory Foot Note Queue is in charge of storing our current foot notes
which are yet to be appended to the document. It does this in memory. If the
foot notes in this struct become large, one could write a different Queue
which stores them in a temp file instead of in memory, freeing up memory space.
*/
pub struct InMemFootNoteQ {
    foot_notes: VecDeque<(Vec<u8>, usize)>,
    queue_number: usize,
}
impl InMemFootNoteQ {
    pub fn new() -> Self {
        InMemFootNoteQ {
            foot_notes: VecDeque::new(),
            queue_number: 0,
        }
    }
}
impl Queue for InMemFootNoteQ {
    fn add(&mut self, foot_note: Option<Vec<u8>>) -> Option<usize> {
        if let None = foot_note {
            return None;
        }
        self.queue_number += 1;
        self.foot_notes
            .push_back((foot_note.unwrap(), self.queue_number));
        return Some(self.queue_number);
    }
    fn pop_front(&mut self) -> Option<(Vec<u8>, usize)> {
        return self.foot_notes.pop_front();
    }
}
