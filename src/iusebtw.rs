use whoami::distro;
fn main() {
    println!("I use {} btw!", &distro()[..4].to_lowercase());
}
