use brace_ec::generation::Generation;
use brace_ec::individual::Individual;
use brace_ec::operator::selector::best::Best;
use brace_ec::population::Population;
use brace_ec_tui::renderer::Renderer;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::Color;
use ratatui::text::Text;
use ratatui::widgets::Widget;
use ratatui::Frame;

use super::{Ind, Pop};

pub struct HiffRenderer;

impl Renderer<(u64, Pop)> for HiffRenderer {
    fn render(&self, generation: &(u64, Pop), frame: &mut Frame) {
        let [best] = generation.population().select(Best).unwrap();

        let help = Text::from("P = Pause, Esc = Exit").left_aligned();
        let title = Text::from("Best Individual").centered();
        let info = Text::from(format!(
            "Fitness = {}, Generation = {}",
            best.fitness().total(),
            generation.id()
        ))
        .right_aligned();

        frame.render_widget(help, frame.area());
        frame.render_widget(title, frame.area());
        frame.render_widget(info, frame.area());
        frame.render_widget(HiffWidget(best), frame.area());
    }
}

pub struct HiffWidget(Ind);

impl Widget for HiffWidget {
    fn render(self, _: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        for (i, val) in self.0.genome().iter().take(buf.content.len()).enumerate() {
            let (x, y) = buf.pos_of(i);

            let color = match val {
                true => Color::White,
                false => Color::Black,
            };

            buf[Position::new(x, y + 1)].set_char('█').set_fg(color);
        }
    }
}
