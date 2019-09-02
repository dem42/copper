pub trait Shader {
    fn start(&mut self);
    fn stop(&mut self);
    fn init(&mut self);
}