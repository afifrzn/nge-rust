use std::io;

fn main() {
    let mut username = String::new();
    let mut password = String::new();

    println!("===LOGIN===");

    print!("Username: ");
    use std::io::Write;
    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).expect("Gagal Membaca Username");

    print!("password: ");
    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).expect("Gagal Membaca password");

    let valid_username = "admin";
    let valid_password = "@#t";

    if username == valid_username && password == valid_password {
        println!("\n Yey dah login, Halo, {}!.", username);
    } else {
        println!("\n Yh login dulu");
    }
}