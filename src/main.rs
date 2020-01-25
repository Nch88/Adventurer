mod game;
mod model;
mod view;

fn main() {
    let game = game::Game::new();
    game.start();
}
