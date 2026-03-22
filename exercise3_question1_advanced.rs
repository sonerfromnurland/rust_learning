use std::io;

fn main() {
    loop {
        println!("Enter temperature like '32 F' or '0 C' (or 'q' to quit):");

        let input = read_input();

        if input.trim().eq_ignore_ascii_case("q") {
            break;
        }

        let (value, unit) = match parse_temperature(&input) {
            Ok(result) => result,
            Err(message) => {
                println!("{message}");
                continue;
            }
        };

        match unit {
            Unit::Fahrenheit => {
                println!("{value}°F = {:.2}°C", fahrenheit_to_celsius(value));
            }
            Unit::Celsius => {
                println!("{value}°C = {:.2}°F", celsius_to_fahrenheit(value));
            }
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");
    input
}

#[derive(Debug, Clone, Copy)]
enum Unit {
    Celsius,
    Fahrenheit,
}

fn parse_temperature(input: &str) -> Result<(f64, Unit), &'static str> {
    let mut parts = input.split_whitespace();

    let value_str = parts.next().ok_or("Missing temperature value")?;
    let unit_str = parts.next().ok_or("Missing unit (use C or F)")?;

    if parts.next().is_some() {
        return Err("Too many parts. Example: 32 F");
    }

    let value: f64 = value_str
        .parse()
        .map_err(|_| "Temperature must be a valid number")?;

    let unit = match unit_str {
        "C" | "c" => Unit::Celsius,
        "F" | "f" => Unit::Fahrenheit,
        _ => return Err("Unknown unit. Use C or F"),
    };

    Ok((value, unit))
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (5.0 / 9.0) * (f - 32.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (9.0 / 5.0) * c + 32.0
}