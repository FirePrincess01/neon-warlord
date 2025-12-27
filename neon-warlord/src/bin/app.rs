fn main() {
    println!("Hello, world!");

    pollster::block_on(neon_warlord::run());
}
