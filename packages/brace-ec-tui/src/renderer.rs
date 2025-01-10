use ratatui::Frame;

pub trait Renderer<G> {
    fn render(&self, generation: &G, frame: &mut Frame);
}
