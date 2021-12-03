use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    // part 1

    let nums: Vec<Vec<u32>> = buf
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(2).unwrap()).collect())
        .collect();

    let (gamma_rate, epsilon_rate): (u32, u32) = nums
        .iter()
        .fold(vec![0u32; nums[0].len()], |sum, n| {
            sum.iter().zip(n).map(|(a, b)| a + b).collect()
        })
        .iter()
        .map(|n| if *n > (nums.len() as u32) / 2 { 1 } else { 0 })
        .rev()
        .zip(0..)
        .fold((0, 0), |(gamma_rate, epsilon_rate), (n, i)| {
            (
                gamma_rate + n * 2u32.pow(i),
                epsilon_rate + (1 - n) * 2u32.pow(i),
            )
        });

    println!("{}", gamma_rate * epsilon_rate);

    Ok(())
}
