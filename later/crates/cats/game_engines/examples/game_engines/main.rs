mod bevy;
mod fyrox;
mod ggez;
mod macroquad;

fn main() {
    bevy::run();
    #[cfg(feature = "fyrox")]
    fyrox::run();
    #[cfg(feature = "ggez")]
    ggez::run();
    macroquad::run();
}
