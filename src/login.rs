use std::io::{self, Write};

fn main() {
    let mut username = String::new();
    let mut password = String::new();

    println!("=== LOGIN ===");

    print!("Username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).expect("Gagal membaca username");

    print!("Password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).expect("Gagal membaca password");

    // Hapus newline dari input
    let username = username.trim();
    let password = password.trim();

    let valid_username = "admin";
    let valid_password = "1234";

    if username == valid_username && password == valid_password {
        println!("\nYey, login berhasil! Halo, {}!", username);
    } else {
        println!("\nYh login dulu ");
    }
}
