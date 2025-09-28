#[derive(Debug, PartialEq)]
enum DivisionError {
    DivisionByZero,
}

fn divide_two_number(&value_one: &u32, &value_two: &u32) -> Result<u32, DivisionError> {
    if value_two == 0 {
        return Err(DivisionError::DivisionByZero);
    }

    let ans = value_one / value_two;
    Ok(ans)
}



fn main() {
    let value_one = 1000000;
    let value_two = 277;
    
    let result = divide_two_number(&value_one, &value_two);

    match result {
        Ok(value) => {
            println!("{}", value)
        }
        Err(err) => {
            println!("{:?}", err)
        }
    }
}
