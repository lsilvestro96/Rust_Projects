use std::io;

fn convert_f_to_c() {
    // Use a loop to keep asking for a valid number if a non-numeric characters
    // are entered
    loop {
        println!("Enter a number to convernt from Fahrenheit to Celsius.");

        let mut num = String::new();

        io::stdin().read_line(&mut num)
            .expect("Failed to read temperature");

        let num : f32 = match num.trim().parse() {
            Ok(n)  => n,
            Err(_) => {
                println!("Please enter a valid number.");
                continue
            }
        }
        break;
    }
    let res = (num - 32) / 1.8;
    println!("{} degrees Fahrenheit is {} degrees Celsius.", num, res)
}

fn menu() {
    loop {
        println!("Select a mode (1-3):");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        println!("3. exit");

        let mut selection = String::new();
        
        io::stdin().read_line(&mut selection)
            .expect("Failed to read selection");

        let selection : i32 = match selection.trim().parse() {
            Ok(s) => s,
            Err(_) => {
                println!("Invalid option, please select a number 1-3.");
                continue
            }
        }
        match selection {
            1 => convert_f_to_c(),
            2 => convert_c_to_f(),
            3     => break,
            _     => println!("Invalid option, please select a number 1-3.")
        }
    }
}

fn main() {
    menu()
}
