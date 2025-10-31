use std::io::{self, Write};

fn eval(exp: &str) -> i64 {
    fn parse_expr(chars: &mut std::iter::Peekable<std::str::Chars>) -> i64 {
        let mut val = parse_factor(chars);
        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() { chars.next(); continue; }
            if ch == '+' || ch == '-' {
                chars.next();
                let next_val = parse_factor(chars);
                if ch == '+' {
                    val += next_val;
                } else {
                    val -= next_val;
                }
            } else {
                break;
            }
        }
        val
    }

    fn parse_factor(chars: &mut std::iter::Peekable<std::str::Chars>) -> i64 {
        let mut val = parse_term(chars);
        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() { chars.next(); continue; }
            if ch == '*' || ch == '/' {
                chars.next();
                let next_val = parse_term(chars);
                if ch == '*' {
                    val *= next_val;
                } else {
                    val /= next_val;
                }
            } else {
                break;
            }
        }
        val
    }

    fn parse_term(chars: &mut std::iter::Peekable<std::str::Chars>) -> i64 {
        let mut num = 0;
        let mut sign = 1;

        if let Some(&'-') = chars.peek() {
            chars.next();
            sign = -1;
        }

        if let Some(&'(') = chars.peek() {
            chars.next();
            let val = parse_expr(chars);
            if let Some(&')') = chars.peek() {
                chars.next();
            }
            return sign * val;
        }

        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() { chars.next(); continue; }
            if ch.is_ascii_digit() {
                num = num * 10 + ch.to_digit(10).unwrap() as i64;
                chars.next();
            } else {
                break;
            }
        }
        sign * num
    }

    let mut chars = exp.chars().peekable();
    parse_expr(&mut chars)
}



fn main() {
    let mut exp = String::new();

    println!("Enter equation to solve");
    println!("\tAvailable commands and symbols: +, -, *, /, (, )");

    // Get equation
    print!("Expresion: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut exp)
        .unwrap();

    let exp = exp.trim();
    let result = eval(exp);
    println!("Result = {}", result);
}
