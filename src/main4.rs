// Add the whoami lib for learn the name of my user and my hostname
use whoami::*;
fn main() {
    println!("Hi, {}. I use {} btw.", realname(), distro().split_whitespace().next().unwrap_or(""));
}
