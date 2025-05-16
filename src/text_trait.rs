pub trait TextTrait {
    fn new(text: &str) -> Self;
    fn add_text(&mut self, text: &str, position: usize) -> Result<(), String>;
    fn get_text(&self) -> String;
}