use std::io;
use rand::Rng;

fn main() {
    println!("Rust Password generator");
    let mut i_loop : bool = true;
    while i_loop == true {
        println!("Choose password length");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error while reading");

        let input = input.trim();
        let password_length: usize = input.parse().expect("Please put an integer.");
        let pool = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()";
        let mut password_generated = String::new();


        for _ in 0..password_length {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..pool.len());
            let c = pool.chars().nth(index).unwrap();
            password_generated.push(c);
        }

        println!("New password: {}" , password_generated);
        let mut choose = String::new();
        println!("Do you want to generate a new password? y/n");
        io::stdin()
            .read_line(&mut choose)
            .expect("Error while reading");
        let choose = choose.trim();
        if choose == "n"{
            i_loop = false;
        } else if choose == "y" {
            i_loop = true;
        } else {
            println!("Invalid input");
            i_loop = false;
        }
    }
}