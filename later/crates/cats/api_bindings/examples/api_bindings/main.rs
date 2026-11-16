// TODO Reviewed ??-??-2026.

mod gzip_round_trip;

fn main() -> std::io::Result<()> {
    gzip_round_trip::run()
}
