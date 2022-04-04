#[allow(unused_imports)]
use std::{env, fs, io, process::*};
#[allow(dead_code)]

#[allow(non_upper_case_globals)]
pub const config_path: &str = "/etc/upkg/config.d/";
#[allow(non_upper_case_globals)]
pub const sources_file: &str = "/etc/upkg/config.d/sources.Ud";
#[allow(non_upper_case_globals)]
pub const packages_file: &str = "/etc/upkg/config.d/packages.Ud";
#[allow(non_upper_case_globals)]
pub const downloads_path: &str = "/var/lib/upkg/downloads/";
#[allow(non_upper_case_globals)]
pub const packages_path: &str = "/var/lib/upkg/packages/";
#[allow(non_upper_case_globals)]
pub const sources_path: &str = "/var/lib/upkg/sources/";
#[allow(non_upper_case_globals)]
pub const cache_path: &str = "/var/lib/upkg/cache/";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");


pub fn launch_command(command: &str, args: &str) -> String {
    let output = Command::new(command)
        .arg(args)
        .output()
        .expect("Failed to execute process");
    let outt: String = String::from_utf8_lossy(&output.stdout).to_string();
    let out = &outt;
    return out.to_string();
}
pub fn download_file(url: &str, output_file: &str, out: bool) {
    if out==true {println!("Downloading {}", url);}
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
}
#[allow(unused_variables)]
#[allow(non_camel_case_types)]
pub fn data_load(Type: &str, file: &str) {
    println!("Loading {}", file);
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
    let data = contents.split("\n");
    for line in data {
        if line.contains(Type) {
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