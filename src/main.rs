use std::process::Command;
fn main() {
    println!("Which function do you want to test?");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    if input == "1" {
        launch_command("pfetch", "-d");
    }
    if input == "2" {
        download_file("https://gist.githubusercontent.com/mrquantumoff/0b443e43759830f88075514dfdae8df4/raw/ef47c07edc54909e668ce9b7625ff3f73d097a64/.zshrc");
    }
}
fn launch_command(command: &str, args: &str) {
    let output = Command::new(command)
        .arg(args)
        .output()
        .expect("Failed to execute process");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
fn download_file(url: &str) {
    println!("Downloading {}", url);
    let output = Command::new("wget")
        .arg("-q")
        .arg("\"")
        .arg(url)
        .arg("\"")
        .output()
        .expect("Failed to execute process");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}