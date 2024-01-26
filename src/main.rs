mod chip8;
fn main() {
    let f = vec![34, 5, 643, 234];

    f.iter()
        .enumerate()
        .for_each(|(index, val)| println!("{index}: {val}"));
}
