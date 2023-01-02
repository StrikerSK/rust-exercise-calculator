pub fn get_input_string() -> String {
    let mut input_text: String = String::new();
    std::io::stdin().read_line(&mut input_text).unwrap();
    return input_text.trim().to_string();
}

pub fn get_input_integer() -> i32 {
    let mut input: String = get_input_string();
    loop {
        match input.parse::<i32>() {
            Ok(i) => {
                if i > 0 {
                    return i;
                } else {
                    println!("Please enter a positive number");
                    input = get_input_string();
                }
            },
            Err(..) => {
                println!("This element was not of Integer type! Please try again!");
                input = get_input_string();
            },
        }
    }
}

pub fn get_input_float() -> f32 {
    let mut input: String = get_input_string();
    loop {
        match input.parse::<f32>() {
            Ok(i) => {
                if i > 0.0 {
                    return i;
                } else {
                    println!("Please enter a positive number");
                    input = get_input_string();
                }
            },
            Err(..) => {
                println!("This element was not of Float type! Please try again!");
                input = get_input_string();
            },
        }
    }
}