use std::io;

fn main() {
    let mut usn = String::new();
    let mut pass = String::new();

    println!(===LOGIN===);

    print!(Username: );
    use std::io::Write;
    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut usn).expect("Gagal Membaca Username");

    print!("Password: ");
    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut pass).expect("Gagal Membaca Password");

    let valid_usn = "admin";
    let valid_pass = "@#t";

    if username == valid_usn && password == valid_pass {
        println!("\n Yey dah login, Halo, {}!.", usn);
    } else {
        println!("\n Yh login dulu");
    }
}