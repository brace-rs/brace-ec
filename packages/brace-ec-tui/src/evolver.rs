use std::time::Duration;

use brace_ec::generation::Generation;
use brace_ec::operator::evolver::Evolver;
use crossterm::event::{self, Event, KeyCode};
use thiserror::Error;

use crate::renderer::Renderer;

pub struct Terminal<E, R> {
    evolver: E,
    renderer: R,
}

impl<E, R> Terminal<E, R> {
    pub fn new(evolver: E, renderer: R) -> Self {
        Self { evolver, renderer }
    }
}

impl<G, E, R> Evolver<G> for Terminal<E, R>
where
    G: Generation,
    E: Evolver<G>,
    R: Renderer<G>,
{
    type Error = TerminalError<E::Error>;

    fn evolve<Rng>(&self, mut generation: G, rng: &mut Rng) -> Result<G, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let mut terminal = ratatui::init();
        let mut paused = false;

        loop {
            terminal
                .draw(|frame| {
                    self.renderer.render(&generation, frame);
                })
                .map_err(TerminalError::Io)?;

            if event::poll(Duration::from_secs(0)).map_err(TerminalError::Io)? {
                let event = event::read().map_err(TerminalError::Io)?;

                if let Event::Key(key) = event {
                    if key.code == KeyCode::Char('p') {
                        paused = !paused;

                        continue;
                    }

                    if key.code == KeyCode::Esc {
                        break;
                    }
                }
            }

            if !paused {
                generation = self
                    .evolver
                    .evolve(generation, rng)
                    .map_err(TerminalError::Evolve)?;
            }
        }

        ratatui::restore();

        Ok(generation)
    }
}

#[derive(Debug, Error)]
pub enum TerminalError<E> {
    #[error(transparent)]
    Evolve(E),
    #[error(transparent)]
    Io(std::io::Error),
}
