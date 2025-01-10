use ratatui::Frame;

pub trait Renderer {
    type Generation;

    fn render(&self, generation: &Self::Generation, frame: &mut Frame);
}
