use art::PrimaryColor;
use art::mix;

fn main() {
    let red: PrimaryColor = PrimaryColor::Red;
    let yellow: PrimaryColor = PrimaryColor::Yellow;

    mix(red, yellow);
}
