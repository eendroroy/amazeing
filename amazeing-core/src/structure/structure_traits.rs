pub trait DataStorage<T> {
    fn push(&mut self, data: T);
    fn pop(&mut self) -> Option<T>;
}
