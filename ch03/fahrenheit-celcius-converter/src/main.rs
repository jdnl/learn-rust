use std::io;

fn main() {

    loop {
        println!("Provide temp");

        let mut user_temp = String::new();

        io::stdin()
            .read_line(&mut user_temp)
            .expect("Failed to read line");

        let user_temp: f32 = match user_temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your temp: {user_temp}");
        println!("Would you like to convert to C or F? (c/f)");

        let mut user_convert_option = String::new();

        io::stdin()
            .read_line(&mut user_convert_option)
            .expect("Failed to read line");

        let user_convert_option: char = match user_convert_option.trim().parse() {
            Ok(char) => char,
            Err(_) => continue,
        };

        if user_convert_option == 'c' {
            let result = convert_c_to_f(user_temp);
            println!("Result {result}");
        } else if user_convert_option == 'f' {
            let result = convert_f_to_c(user_temp);
            println!("Result: {result}");
        } else {
            println!("Please enter either c or f")
        }

        break;
    }

    fn convert_c_to_f (temp_in_c: f32) -> f32 {
        (temp_in_c * 1.8) + 32.0
    }

    fn convert_f_to_c (temp_in_f: f32) -> f32 {
        (temp_in_f - 32.0) / 1.8
    }

    
}
