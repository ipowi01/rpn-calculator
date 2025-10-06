use std::io;


fn eval(a:&f64,b:&f64, c:&char) -> Result<f64,String>{
    let result = match *c {
        '-' => *a - *b,
        '+' => *a + *b,
        '*' => *a * *b,
        '/' => *a / *b,
        _ => return Err("Mismatched types".into()),
    };
    Ok(result)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let operators = "+-*/";
    let mut input = String::new();
    let mut stack: Vec<f64> = Vec::new();

    loop {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let s = input.trim();

        match s.chars().next() {
            Some(c) if c.is_ascii_digit() => {
                match s.parse::<f64>() {
                    Ok(num) => stack.push(num),
                    Err(_) => return Err(format!("Invalid number: {}", s).into()),
                }}
            Some(c) if operators.contains(c) && s.len() == 1 => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(eval(&a, &b, &c)?);
            }
            Some(_) if s == "do" => break,
            _ => return Err("Invalid character".into()),
        }
    }
    println!("{}", stack.last().unwrap());


    Ok(())
}