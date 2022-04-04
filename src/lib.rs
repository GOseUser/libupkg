#[allow(unused_imports)]
use std::{env, fs, io, process::*, string::*};
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
    if out==true {println!("{}", String::from_utf8_lossy(&output.stdout));}
}
#[allow(unused_variables)]
#[allow(non_camel_case_types)]
pub struct app_names_and_urls {
    pub app_names: Vec<String>,
    pub app_urls: Vec<String>,
}
#[allow(non_camel_case_types)]
pub struct repos {
    pub repos_names: Vec<String>,
    pub repos_urls: Vec<String>,
}
#[allow(non_camel_case_types)]
pub struct versions {
    pub versions_names: Vec<String>,
    pub versions_versions: Vec<i32>,
}
#[allow(unused_mut)]
pub fn get_versions() -> versions {
    let contents = fs::read_to_string(packages_file).expect("Something went wrong reading the file");
    let mut pkgs_names: Vec<String> = Vec::new();
    let mut pkgs_versions: Vec<i32> = Vec::new();
    let mut conts = &contents;
    let conts_split = conts.split("\n");
    for cont in conts_split {
        if cont.contains("###COMMENT###") {
            continue;
        }
        else {
            let mut splitted = cont.split("|");
            let name = splitted.next().unwrap();
            let url = splitted.next().unwrap();
            pkgs_names.push(name.to_string());
            pkgs_versions.push(url.parse::<i32>().unwrap());
        }
    }
    return versions {
        versions_names: pkgs_names,
        versions_versions: pkgs_versions,
    };
}
#[allow(unused_mut)]

pub fn get_app_names_and_urls(file: String) -> app_names_and_urls {
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
    let mut app_names: Vec<String> = Vec::new();
    let mut app_urls: Vec<String> = Vec::new();
    let conts = &contents;
    let conts_split = conts.split("\n");
    for cont in conts_split {
        if cont.contains("###COMMENT###") {
            continue;
        }
        else {
            let mut splitted = cont.split("|");
            let mut name = splitted.next().unwrap();
            let mut url = splitted.next().unwrap();
            app_names.push(name.to_string());
            app_urls.push(url.to_string());
        }
    }
    return app_names_and_urls {
        app_names: app_names,
        app_urls: app_urls,
    };
#[allow (dead_code)]
pub fn get_sources() -> repos {
    let contents = fs::read_to_string(sources_file).expect("Something went wrong reading the file");
    let mut repos_names: Vec<String> = Vec::new();
    let mut repos_urls: Vec<String> = Vec::new();
    let mut conts = &contents;
    let mut conts_split = conts.split("\n");
    for cont in conts_split {
        if cont.contains("###COMMENT###") {
            continue;
        }
        else {
            let mut splitted = cont.split("|");
            let mut name = splitted.next().unwrap();
            let mut url = splitted.next().unwrap();
            repos_names.push(name.to_string());
            repos_urls.push(url.to_string());
        }
    }
    return repos {
        repos_names: repos_names,
        repos_urls: repos_urls,
    };
}



/*
This is example code of how you will need to load code from the file, this is commented out because it should be used in your "upkg" wrapper
#[allow(non_camel_case_types)]

pub fn data_get(Type: &str, file: &str) {
    println!("Loading {}", file);
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
    let data = contents.split("\n");
    for line in data {
        if line.contains(Type) {
            let mut oline = line.to_owned();
            
            oline = oline.replace(Type, "");
            oline = oline.replace(",", "");
            if Type == "app_names_and_urls" {
                let appnames = get_app_names_and_urls(oline);
            }
            else if Type == "currentversions" {
                let versions = get_versions(oline);
            }
            else if Type == "latestversions" {
                let latestversions = get_latestversions(oline);
            }
        }
        
    }
}*/
}
