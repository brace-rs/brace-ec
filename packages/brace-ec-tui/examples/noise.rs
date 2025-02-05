use brace_ec::generation::Generation;
use brace_ec::operator::evolver::select::Select;
use brace_ec::operator::evolver::Evolver;
use brace_ec::operator::mutator::noise::Noise;
use brace_ec::operator::mutator::Mutator;
use brace_ec::operator::selector::tournament::Tournament;
use brace_ec::operator::selector::Selector;
use brace_ec_tui::evolver::Terminal;
use brace_ec_tui::renderer::Renderer;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::Color;
use ratatui::text::Text;
use ratatui::widgets::Widget;
use ratatui::Frame;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let selector = Tournament::binary().mutate(Noise(1..5).rate(0.25));
    let evolver = Terminal::new(Select::fill(selector), NoiseRenderer);

    evolver.evolve((0, vec![0; 500]), &mut rand::rng())?;

    Ok(())
}

pub struct NoiseRenderer;

impl Renderer<(u64, Vec<u8>)> for NoiseRenderer {
    fn render(&self, generation: &(u64, Vec<u8>), frame: &mut Frame) {
        let help = Text::from("P = Pause, Esc = Exit").left_aligned();
        let generation_id = Text::from(generation.id().to_string()).right_aligned();

        frame.render_widget(help, frame.area());
        frame.render_widget(generation_id, frame.area());
        frame.render_widget(NoiseWidget(generation), frame.area());
    }
}

pub struct NoiseWidget<'a>(&'a (u64, Vec<u8>));

impl Widget for NoiseWidget<'_> {
    fn render(self, _: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        for (i, val) in self.0.population().iter().enumerate() {
            let (x, y) = buf.pos_of(i);

            buf[Position::new(x, y + 1)]
                .set_char('█')
                .set_fg(Color::Rgb(*val, *val, *val));
        }
    }
}
