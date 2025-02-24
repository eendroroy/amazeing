use crate::structure::structure_traits::DataStorage;
use std::collections::VecDeque;

pub struct Queue<T> {
    data: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            data: VecDeque::<T>::new(),
        }
    }
}

impl<T> DataStorage<T> for Queue<T> {
    fn push(&mut self, data: T) {
        self.data.push_back(data)
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop_front()
    }
}
