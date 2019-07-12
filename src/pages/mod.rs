
pub mod commentpage;
pub mod homepage;

pub trait Page {
    fn render(&self);

    fn fetch_data(&mut self);

    fn input(&mut self, key: i32);
}
