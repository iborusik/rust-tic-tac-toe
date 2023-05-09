use super::Game;

pub trait View {
    fn draw(&self, game: &Game);
}