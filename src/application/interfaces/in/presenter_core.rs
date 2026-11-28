pub trait Presenter<T> {
    fn present(&mut self, dto: T);
}
