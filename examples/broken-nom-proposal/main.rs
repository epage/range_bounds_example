pub mod fake_nom;

fn main() {
    fake_nom::many(10);
    fake_nom::many(..);
    fake_nom::many(10..);
    fake_nom::many(..10);
    fake_nom::many(1..10);
    fake_nom::many(1..=10);
}
