// Group Number: 150060.5780.41
// Created by: Shira Yodaiken 315119461 and Beila Wellner 205823792


use std::{fs, io};
use std::path::Path;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Write, Read};
//use std::fs::metadata;



static mut COUNTER:i32=0;

fn add()->String{
    let result=String::from("@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=D+M\n@SP\nM=M+1\n");
    return result;
}

fn eq() -> String{
    let mut result = String::from("@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nD=M-D\n@IF_TRUE");
    let num_true_false = count_true_false();
    result.push_str(num_true_false.as_str());
    result.push_str("\nD;JEQ\n@SP\nA=M\nM=0\n@IF_FALSE");
    result.push_str(num_true_false.as_str());
    result.push_str("\n0;JMP\n(IF_TRUE");
    result.push_str(num_true_false.as_str());
    result.push_str(")\n@SP\nA=M\nM=-1\n(IF_FALSE");
    result.push_str(num_true_false.as_str());
    result.push_str(")\n@SP\nM=M+1\n");
    return result;
}

fn lt() -> String{
    let mut result = String::from("@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nD=M-D\n@IF_TRUE");
    let num_true_false= count_true_false();
    result.push_str(num_true_false.as_str());
    result.push_str("\nD;JLT\n@SP\nA=M\nM=0\n@IF_FALSE");
    result.push_str(num_true_false.as_str());
    result.push_str("\n0;JMP\n(IF_TRUE");
    result.push_str(num_true_false.as_str());
    result.push_str(")\n@SP\nA=M\nM=-1\n(IF_FALSE");
    result.push_str(num_true_false.as_str());
    result.push_str(")\n@SP\nM=M+1\n");
    return result;
}

fn gt() -> String{
    let mut result = String::from("@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nD=M-D\n@IF_TRUE");
    let num_true_false = count_true_false();
    result.push_str(num_true_false.as_str());
    result.push_str("\nD;JGT\n@SP\nA=M\nM=0\n@IF_FALSE");
    result.push_str(num_true_false.as_str());
    result.push_str("\n0;JMP\n(IF_TRUE");
    result.push_str(num_true_false.as_str());
    result.push_str(")\n@SP\nA=M\nM=-1\n(IF_FALSE");
    result.push_str(num_true_false.as_str());
    result.push_str(")\n@SP\nM=M+1\n");
    return result;
}

fn sub() -> String{
    let result = String::from("@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=M-D\n@SP\nM=M+1\n");
    return result;
}

fn neg() -> String{
    let result = String::from("@SP\nM=M-1\nA=M\nD=-M\nM=D\n@SP\nM=M+1\n");
    return result;
}

fn and() -> String{
    let result = String::from("@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=M&D\n@SP\nM=M+1\n");
    return result;
}

fn or() -> String{
    let result = String::from("@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=M|D\n@SP\nM=M+1\n");
    return result;
}

fn push_constant(num:&String) ->String {
    let mut result=String::from("@");
    result.push_str(num.as_str());
    result.push_str("\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
    return result;
}

fn push_temp(num: &String) -> String{
    let mut result = String::from("@");
    let mut my_int: i32 = num.parse().unwrap();
    my_int += 5;
    let string_my_int: String = my_int.to_string();
    result.push_str(string_my_int.as_ref());
    result.push_str("\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
    return result;
}

fn push_local(num: &String) -> String{
    let mut result = String::from("@");
    result.push_str(num.as_str());
    result.push_str("\nD=A\n@LCL\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
    return result;
}

fn push_this(num: &String) -> String{
    let mut result = String::from("@");
    result.push_str(num.as_str());
    result.push_str("\nD=A\n@THIS\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
    return result;
}

fn push_that(num: &String) -> String{
    let mut result = String::from("@");
    result.push_str(num.as_str());
    result.push_str("\nD=A\n@THAT\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
    return result;
}

fn push_argument(num: &String) -> String{
    let mut result = String::from("@");
    result.push_str(num.as_str());
    result.push_str("\nD=A\n@ARG\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
    return result;
}

fn push_static(num: &String, file_name: &String) ->String{
    let mut result =String::from("@");
    result.push_str(file_name);
    result.push_str(".");
    result.push_str(num);
    result.push_str("\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
    return result;
}

fn push_pointer(num: &String) ->String{
    let mut result =String::from("@");
    match num.as_str() {
        "0"=> result.push_str("THIS"),
        "1"=> result.push_str("THAT"),
        _=> println!("wrong number"),
    }
    result.push_str("\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
    return result;
}

fn pop_this(num: &String) -> String{
    let mut result = String::from("@SP\nA=M-1\nD=M\n@THIS\nA=M\n");
    let mut my_int: i32 = num.parse().unwrap();
    while my_int > 0 {
        result.push_str("A=A+1\n");
        my_int -= 1;
    }
    result.push_str("M=D\n@SP\nM=M-1\n");
    return result;
}

fn pop_argument(num: &String) -> String{
    let mut result = String::from("@SP\nA=M-1\nD=M\n@ARG\nA=M\n");
    let mut my_int: i32 = num.parse().unwrap();
    while my_int > 0 {
        result.push_str("A=A+1\n");
        my_int -= 1;
    }
    result.push_str("M=D\n@SP\nM=M-1\n");
    return result;
}

fn pop_local(num: &String) -> String{
    let mut result = String::from("@SP\nA=M-1\nD=M\n@LCL\nA=M\n");
    let mut my_int: i32 = num.parse().unwrap();
    while my_int > 0 {
        result.push_str("A=A+1\n");
        my_int -= 1;
    }
    result.push_str("M=D\n@SP\nM=M-1\n");
    return result;
}

fn pop_that(num: &String) -> String{
    let mut result = String::from("@SP\nA=M-1\nD=M\n@THAT\nA=M\n");
    let mut my_int: i32 = num.parse().unwrap();
    while my_int > 0 {
        result.push_str("A=A+1\n");
        my_int -= 1;
    }
    result.push_str("M=D\n@SP\nM=M-1\n");
    return result;
}

fn pop_temp(num: &String) -> String{
    let mut result = String::from("@SP\nA=M-1\nD=M\n@");
    let mut my_int: i32 = num.parse().unwrap();
    my_int += 5;
    let string_my_int: String = my_int.to_string();
    result.push_str(string_my_int.as_ref());
    result.push_str("\nM=D\n@SP\nM=M-1\n");
    return result;
}

fn pop_static(num: &String, file_name: &String) ->String{
    let mut result =String::from("@SP\nA=M-1\nD=M\n@");
    result.push_str(file_name);
    result.push_str(".");
    result.push_str(num);
    result.push_str("\nM=D\n@SP\nM=M-1\n");
    return result;
}

fn pop_pointer(num: &String) ->String{
    let mut result =String::from("@SP\nA=M-1\nD=M\n@");
    match num.as_str() {
        "0"=> result.push_str("THIS"),
        "1"=> result.push_str("THAT"),
        _=> println!("wrong number"),
    }
    result.push_str("\nM=D\n@SP\nM=M-1\n");
    return result;
}


fn count_true_false()-> String{
    unsafe {
        let string_counter: String = COUNTER.to_string();
        COUNTER += 1;
        return string_counter;
    }
}

fn push(push_type: &String, num: &String, file_name: &String)-> String{
    let push_type_str: &str = push_type;
    let mut result = String::new();
    match push_type_str {
        "constant" => result = push_constant(&num),
        "local"    => result = push_local(&num),
        "argument" => result = push_argument(&num),
        "that"     => result = push_that(&num),
        "this"     => result = push_this(&num),
        "temp"     => result = push_temp(&num),
        "static"   => result = push_static(&num,&file_name),
        "pointer"  => result = push_pointer(&num),
        _=> println!("'push {}' is not exist!", push_type_str),
    }
    return result;
}

fn pop(pop_type: &String, num: &String, file_name: &String)-> String{
    let pop_type_str: &str = pop_type;
    let mut result = String::new();
    match pop_type_str {
        "local"    => result = pop_local(&num),
        "argument" => result = pop_argument(&num),
        "that"     => result = pop_that(&num),
        "this"     => result = pop_this(&num),
        "temp"     => result = pop_temp(&num),
        "static"   => result = pop_static(&num, &file_name),
        "pointer"  => result = pop_pointer(&num),
        _=> println!("'pop {}' is not exist!", pop_type_str),
    }
    return result;
}

fn main() {
    //let temp_path: String = r"C:\targil1-315119461-205823792\targil1\MemoryAccess\BasicTest".parse().unwrap();
    //let temp_path: String = r"C:\targil1-315119461-205823792\targil1\SimpleAdd".parse().unwrap();


    println!("please enter folder path >>");
    // Reading path from user
    let mut user_path = String::new();
    io::stdin().read_line(&mut user_path).ok().expect("Couldn't read line");

    // Removes the last character (\n) from the input-path
    let len = user_path.len();
    user_path.truncate(len - 1);
    let path = Path::new(&user_path);

    // Go over files in a folder
    for entry in fs::read_dir(path).expect("Unable to list") {
        let entry = entry.expect("unable to get entry");

        // If it is vm file
        if entry.path().extension().and_then(OsStr::to_str) == Some("vm") {
            // Open the vm file to read
            let mut file = File::open(entry.path()).expect("Unable to open file");

            // Creating asm file with the same name
            let file_name = entry.path().file_stem().unwrap().to_str().unwrap().to_string();
            let asm_file_name = user_path.to_owned() + &r"\" + file_name.as_ref() + &".asm";
            let mut asm_file = File::create(Path::new(&asm_file_name)).expect("Unable to create the file");

            // Reading from the vm file line by line
            let mut data = String::new();
            let mut output = String::new();
            file.read_to_string(&mut data).expect("Unable to read the file");
            for line in data.lines()  {
                if !(line == "") && !line.starts_with("//"){
                    asm_file.write("//".as_bytes()).expect("Unable to write data");
                    asm_file.write(line.as_bytes()).expect("Unable to write data");
                    asm_file.write("\n".as_bytes()).expect("Unable to write data");
                    println!("//{}", line);
                    let line_vector: Vec<_> = line.split(" ").collect();

                    match line_vector[0] {
                        "eq"   => output = eq(),
                        "lt"   => output = lt(),
                        "gt"   => output = gt(),
                        "sub"  => output = sub(),
                        "neg"  => output = neg(),
                        "and"  => output = and(),
                        "or"   => output = or(),
                        "push" => {
                            let num: String = line_vector[2].to_string();
                            let push_type: String = line_vector[1].to_string();
                            output = push(&push_type, &num, &file_name);
                        },
                        "pop"=>{
                            let num: String = line_vector[2].to_string();
                            let pop_type: String = line_vector[1].to_string();
                            output = pop(&pop_type, &num, &file_name);
                        },
                        "add" => output = add(),
                        _ => println!("'{}' action is not exist!",line_vector[0]),
                    }
                    print!("{}",output);
                    asm_file.write(output.as_bytes()).expect("Unable to write data");
                    asm_file.write("\n".as_bytes()).expect("Unable to write data");
                }

            }
        }
    }
}