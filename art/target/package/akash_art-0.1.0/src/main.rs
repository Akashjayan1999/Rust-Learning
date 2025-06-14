use akash_art::{PrimaryColor, SecondaryColor};
use akash_art::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let orange = mix(red, yellow);
    
    match orange {
        SecondaryColor::Orange => println!("The color is orange!"),
        _ => println!("The color is not orange!"),
    }
}