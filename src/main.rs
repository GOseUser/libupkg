use std::process::Command;
use std::env;
use std::io;
use std::fs;

 fn main() {
    println!("Which function do you want to test?");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    if input == "1" {
        launch_command("pfetch", "-d");
    }
    else if input == "2" {
        download_file("https://gist.githubusercontent.com/mrquantumoff/0b443e43759830f88075514dfdae8df4/raw/ef47c07edc54909e668ce9b7625ff3f73d097a64/.zshrc", "./.qzshrc");
    }
}

pub struct appnameandurls{
    name: Vec<String>,
    url:  Vec<String>
}
pub struct repos {
    name:  Vec<String>,
    url:  Vec<String>
}
pub fn launch_command(command: &str, args: &str) {
    let output = Command::new(command)
        .arg(args)
        .output()
        .expect("Failed to execute process");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
pub fn download_file(url: &str, output_file: &str) {
    println!("Downloading {}", url);
    let output = Command::new("/bin/wget")
        .arg("-q")
        .arg("-O \'")
        .arg(output_file)
        .arg("\'")
        .arg("\"")
        .arg(url)
        .arg("\"")
        .output()
        .expect("Failed to execute process");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
pub fn data_load(Type: &str, file: &str) {
    println!("Loading {}", file);
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
    let mut data = contents.split("\n");
    for line in data {
        if line.contains(Type) {
            let type_chars = Type.chars().count();
            let mut oline = line.to_owned();
            oline = oline.replace(Type, "");
            if Type == "app_names_and_urls" {
                
            }
            else if Type == "repos" {
                
            }
            else if Type == "currentversions" {
                
            }
            else if Type == "latestversions" {
                
            }
        }
    }
    
}