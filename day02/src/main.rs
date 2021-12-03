use std::{
    io::{self, Read},
    str::FromStr,
};

#[derive(Debug, PartialEq)]
enum SubmarineCommand {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for SubmarineCommand {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let make_error = || {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "String is not a submarine command.",
            )
        };

        let (cmd, val) = s.split_once(" ").ok_or(make_error())?;
        let val: i32 = val.parse().map_err(|_| make_error())?;

        match cmd {
            "forward" => Ok(SubmarineCommand::Forward(val)),
            "down" => Ok(SubmarineCommand::Down(val)),
            "up" => Ok(SubmarineCommand::Up(val)),
            _ => Err(make_error()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn submarine_command_from_str_ok() {
        let submarine_command = "down 37".parse::<SubmarineCommand>().ok();
        assert_eq!(submarine_command, Some(SubmarineCommand::Down(37)));
    }

    #[test]
    fn submarine_command_from_str_err() {
        assert!("dow 37".parse::<SubmarineCommand>().is_err());
    }
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buf)?;

    // part 1

    let (h, d) = buf
        .lines()
        .map(|s| match s.parse::<SubmarineCommand>() {
            Ok(v) => v,
            Err(e) => panic!("Failed to parse input: {}", e),
        })
        .fold((0, 0), |(h, d), c| match c {
            SubmarineCommand::Forward(v) => (h + v, d),
            SubmarineCommand::Down(v) => (h, d + v),
            SubmarineCommand::Up(v) => (h, d - v),
        });

    println!("{}", h * d);

    // part 2

    let (h, d, _) = buf
        .lines()
        .map(|s| match s.parse::<SubmarineCommand>() {
            Ok(v) => v,
            Err(e) => panic!("Failed to parse input: {}", e),
        })
        .fold((0, 0, 0), |(h, d, a), c| match c {
            SubmarineCommand::Forward(v) => (h + v, d + v * a, a),
            SubmarineCommand::Down(v) => (h, d, a + v),
            SubmarineCommand::Up(v) => (h, d, a - v),
        });

    println!("{}", h * d);

    Ok(())
}
