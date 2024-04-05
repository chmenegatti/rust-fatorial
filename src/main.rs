use std::io;

fn convert_to_int(s: & String) -> i32 {
    let x: i32 = s.trim().parse::<i32>().unwrap();
    x
}
fn main() {
    let mut input = String::new();
    println!("Enter a number: ");
    io::stdin().read_line(&mut input).expect("Failed to read number");
    let mut fatorial = 1;

    let mut n = convert_to_int(&input);

    while convert_to_int(&input) > 1 {

        fatorial = fatorial * convert_to_int(&input);
        n = n - 1;
        
    }
}
