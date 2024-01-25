mod chip8;
fn main() {
    let x: u8 = 0b10101101;
    let y: u8 = 0b01101100;

    println!("{:#b}", x);
    println!("{:#b}", y);

    let a = (x & 0b1000_0000) >> 7;
    let b = (x & 0b0000_0001);

    let c = (y & 0b1000_0000) >> 7;
    let d = (y & 0b0000_0001);
    println!("{:#b}", a);
    println!("{:#b}", b);

    println!("{:#b}", c);
    println!("{:#b}", d);

    let k: u8 = 0b1101_0110;
    println!("{}", k << 1);
    let usdufsdf = (k & 0b1000_0000) >> 7;
    println!("{}", usdufsdf);
}
