use brace_ec::core::generation::Generation;
use brace_ec::core::individual::scored::Scored;
use brace_ec::core::individual::Individual;
use brace_ec_tui::renderer::Renderer;
use image::GrayImage;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Position, Rect};
use ratatui::style::Color;
use ratatui::text::Text;
use ratatui::widgets::{Block, Widget};
use ratatui::Frame;

use crate::individual::Image;

pub struct ImageRenderer {
    image: Image,
}

impl ImageRenderer {
    pub fn new(image: Image) -> Self {
        Self { image }
    }
}

impl Renderer<(u64, Vec<Scored<Image, u64>>)> for ImageRenderer {
    fn render(&self, generation: &(u64, Vec<Scored<Image, u64>>), frame: &mut Frame) {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Fill(1)])
            .split(frame.area());

        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 2); 2])
            .split(vertical[1]);

        let help = Text::from("P = Pause, Esc = Exit").centered();

        frame.render_widget(help, vertical[0]);

        let source = Block::bordered().title(format!("Generation {}", generation.id()));
        let target = Block::bordered().title("Target");

        frame.render_widget(source, horizontal[0]);
        frame.render_widget(target, horizontal[1]);

        let best = generation.1.first().unwrap();

        frame.render_widget(ImageWidget(best.genome()), horizontal[0]);
        frame.render_widget(ImageWidget(self.image.genome()), horizontal[1]);
    }
}

pub struct ImageWidget<'a>(&'a GrayImage);

impl Widget for ImageWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let mut area = area.intersection(buf.area);

        if area.is_empty() {
            return;
        }

        area.x += 1;
        area.y += 1;
        area.height -= 2;
        area.width -= 2;

        for (x, y, pixel) in self.0.enumerate_pixels() {
            let x = area.x as u32 + (x * 2);
            let y = area.y as u32 + y;

            if !area.contains(Position::new(x as u16, y as u16)) {
                continue;
            }

            buf[Position::new(x as u16, y as u16)]
                .set_char('█')
                .set_fg(Color::Rgb(pixel[0], pixel[0], pixel[0]));

            if !area.contains(Position::new(x as u16 + 1, y as u16)) {
                continue;
            }

            buf[Position::new(x as u16 + 1, y as u16)]
                .set_char('█')
                .set_fg(Color::Rgb(pixel[0], pixel[0], pixel[0]));
        }
    }
}
