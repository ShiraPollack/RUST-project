// Group Number: 150060.5780.41
// Created by: Shira Yodaiken 315119461 and Beila Wellner 205823792
use std::{fs, io};
use std::path::Path;
use std::ffi::OsStr;
use std::fs::{OpenOptions, File, DirEntry};
use std::io::{Write, Read};

fn is_vm_file(e: &DirEntry) -> bool{
    return e.path().extension().and_then(OsStr::to_str) == Some("vm");
}

fn hello_file_handling(mut file: &std::fs::File, mut out_file: &std::fs::File) {
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Unable to read the file");
    for line in data.lines() {
        out_file.write_all(line.as_bytes()).expect("Unable to write data");
        out_file.write_all("\n".as_bytes()).expect("Unable to write data");
        if line.starts_with("you") || line.starts_with("You")||line.starts_with("are")||line.starts_with("Are"){
            println!("{}", line)
        }
    }
}

fn main() {
    //let path = Path::new(r"C:\rust project\targil0");

    let mut counter = 1;

    println!("please enter folder path >>");
    let mut user_path = String::new();
    io::stdin().read_line(&mut user_path).ok().expect("Couldn't read line");

    // Removes the last character (\n) from the input
    let len = user_path.len();
    user_path.truncate(len - 1);
    let path = Path::new(&user_path);

    // Go over files in a folder
    for entry in fs::read_dir(path).expect("Unable to list") {
        let entry = entry.expect("unable to get entry");
        if is_vm_file(&entry){
            // Open file to read, write, append
            let mut file = OpenOptions::new().read(true).write(true).append(true).open(entry.path()).unwrap();
            if entry.path().file_stem().unwrap() == "hello" {
                // Create a file for writing
                let out_file_name = user_path.to_owned() + &r"\hello.asm".to_string();
                let out_file = File::create(&out_file_name).expect("Unable to write file");
                hello_file_handling(&file, &out_file);
            }
            // Add a serial number to the end of a file
            file.write("\n".as_bytes()).expect("Unable to write data");
            file.write(counter.to_string().as_bytes()).expect("Unable to write data");
            print!("number {} was added to file: {}\n", counter, entry.path().display());
            counter += 1;
        }
    }
}