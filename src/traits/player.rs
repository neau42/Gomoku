pub trait Player {
    fn new() -> Box<Self> where Self: Sized;
    fn put_stone(&self);
}