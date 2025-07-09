pub enum TextAction {
    Add { text: String, position: usize },
    Delete { text: String, position: usize },
}