use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buf)?;
    let values: Vec<i32> = buf
        .lines()
        .map(|s| s.parse().expect("invalid input"))
        .collect();

    // part 1

    let increases = values
        .iter()
        .zip(values.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count();

    println!("{}", increases);

    // part 2

    let values_sliding_window: Vec<i32> = values
        .iter()
        .zip(values.iter().skip(1))
        .zip(values.iter().skip(2))
        .map(|((a, b), c)| a + b + c)
        .collect();

    let sliding_window_increases = values_sliding_window
        .iter()
        .zip(values_sliding_window.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count();

    println!("{}", sliding_window_increases);

    Ok(())
}
