#[cfg(feature = "lua")]
mod mlua;
#[cfg(feature = "lua")]
mod mlua2;

fn main() {
    mlua::run();
    mlua2::run();
}
