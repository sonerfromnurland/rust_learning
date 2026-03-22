use std::io;

fn main() {

loop {

    println!("Enter degree: ");

    let mut degree = String::new();
    io::stdin()
        .read_line(&mut degree)
        .expect("stdin should be enabled");

    let degree: f64 = degree
        .trim()
        .parse()
        .expect("all chars must be digit except for beginning minus sign");

    println!("Unit type: ");


    let mut unit = String::new();
    io::stdin()
            .read_line(&mut unit)
            .expect("stdin should be enabled");


    let result: f64 = match unit.trim() {
        "F" => fahren_to_celc(degree),
        "C" => celc_to_fahren(degree),
        _ => { println!("Unknown unit"); continue; }
    };

    println!("result degree {}", result);
}
    
}

fn fahren_to_celc(f: f64) -> f64 {
    5f64 / 9f64 * (f - 32.0)
}

fn celc_to_fahren(c: f64) -> f64 {
    9f64 / 5f64 * c + 32.0
}