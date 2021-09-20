pub mod fake_nom;
pub mod fake_std;

fn main() {
    fake_nom::many_m_n(10, 10);
    fake_nom::many0();
    fake_nom::many_m_n(10, usize::MAX);
    fake_nom::many_m_n(0, 9);
    fake_nom::many_m_n(1, 9);
    fake_nom::many_m_n(1, 10);
}
