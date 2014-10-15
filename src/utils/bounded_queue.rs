use std::vec::Vec;
use std::result;

pub struct BoundedQueue<T> {
    data: Vec<Option<T>>,
    size: uint,
    read_index: uint,
    write_index: uint
}

pub enum State {
    Full,
    Empty
}

pub type QueueResult<T> = result::Result<T, State>;

impl<T> BoundedQueue<T> {
     pub fn new(size: uint) -> BoundedQueue<T> {
         let real_size = size + 1;
         BoundedQueue {
             data: Vec::from_fn(real_size, |_| { None }),
             size: real_size,
             read_index: 0,
             write_index: 0
         }

     }

     pub fn push(&mut self, value: T) -> QueueResult<uint> {
         if self.is_full() {
             return Err(Full);
         }

         let index = self.write_index;
         *self.data.get_mut(index) = Some(value);
         self.write_index = (self.write_index + 1) % self.size;

         Ok(index)
     }

     pub fn pop(&mut self) -> QueueResult<T> {
         if self.is_empty() {
             return Err(Empty);
         }

         let value = self.data.get_mut(self.read_index).take().unwrap();

         self.read_index = (self.read_index + 1) % self.size;

         Ok(value)

     }
     
     pub fn is_full(&self) -> bool {
         (self.write_index + 1) % self.size == self.read_index
     }

     pub fn is_empty(&self) -> bool {
         self.write_index == self.read_index
     }

}

#[cfg(test)]
mod tests {
    use super::{BoundedQueue, State, QueueResult};
    use std::iter;

    #[test]
    fn basic_tests() {
        let mut queue = BoundedQueue::<int>::new(8);
        for to_push in iter::count(0, 1).take(8) {
            assert_eq!(queue.push(to_push).is_ok(), true);
        }
        assert_eq!(queue.push(9).is_err(), true);

    }
}
