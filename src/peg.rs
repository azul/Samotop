extern crate peg;

fn main() {
    peg::cargo_build("src/grammar/smtp.rustpeg");
}
