use std::fs::File;
use std::io;
use std::io::{BufRead,BufReader};

fn database(){
    println!("");
    project();
    println!("");
    employee();
    println!("");
    customer();
    println!("");
    dataplan();
    println!("");
}

fn project() -> Result <(), Box<dyn std::error::Error>> {
    println!("");
    let mut file = File::open("globalcom_dbase.sql")?;
    let mut reader = BufReader::new(&mut file);
    let mut current_line = String::new();
    let mut in_copy_block = false; 

    while let Ok(_) = reader.read_line(&mut current_line) {
        let current_line_trimmed = current_line.trim(); 
        if current_line_trimmed.starts_with("COPY public.project") {
            in_copy_block = true;
        } 
        if in_copy_block { 
            if current_line_trimmed == "\\." {
                in_copy_block = false; 
                break;
            } else {
                println!("{}", current_line_trimmed);
            }
        }
        current_line.clear(); 
    }
    Ok(())
}

fn employee() -> Result <(), Box<dyn std::error::Error>> {
    println!("");
    let mut file = File::open("globalcom_dbase.sql")?;
    let mut reader = BufReader::new(&mut file);
    let mut current_line = String::new();
    let mut in_copy_block = false; 

    while let Ok(_) = reader.read_line(&mut current_line) {
        let current_line_trimmed = current_line.trim(); 
        if current_line_trimmed.starts_with("COPY public.employees") {
            in_copy_block = true;
        } 
        if in_copy_block { 
            if current_line_trimmed == "\\." {
                in_copy_block = false; 
                break;
            } else {
                println!("{}", current_line_trimmed);
            }
        }
        current_line.clear(); 
    }
    Ok(())
}

fn customer() -> Result <(), Box<dyn std::error::Error>> {
    println!("");
    let mut file = File::open("globalcom_dbase.sql")?;
    let mut reader = BufReader::new(&mut file);
    let mut current_line = String::new();
    let mut in_copy_block = false; 

    while let Ok(_) = reader.read_line(&mut current_line) {
        let current_line_trimmed = current_line.trim(); 
        if current_line_trimmed.starts_with("COPY public.customer") {
            in_copy_block = true;
        } 
        if in_copy_block { 
            if current_line_trimmed == "\\." {
                in_copy_block = false; 
                break;
            } else {
                println!("{}", current_line_trimmed);
            }
        }
        current_line.clear(); 
    }
    Ok(())
}

fn dataplan() -> Result <(), Box<dyn std::error::Error>> {
    println!("");
    let mut file = File::open("globalcom_dbase.sql")?;
    let mut reader = BufReader::new(&mut file);
    let mut current_line = String::new();
    let mut in_copy_block = false; 

    while let Ok(_) = reader.read_line(&mut current_line) {
        let current_line_trimmed = current_line.trim(); 
        if current_line_trimmed.starts_with("COPY public.dataplan") {
            in_copy_block = true;
        } 
        if in_copy_block { 
            if current_line_trimmed == "\\." {
                in_copy_block = false; 
                break;
            } else {
                println!("{}", current_line_trimmed);
            }
        }
        current_line.clear(); 
    }
    Ok(())
}

fn main() {
    let mut input_1 = String::new();

    println!("\t\t\t\tWelcome To Globalcom Ltd.");
    println!("");
    println!("Kindly state who is requesting access to our Database.");
    io::stdin().read_line(&mut input_1).expect("Wrong input");
    let fun = input_1.trim().to_lowercase();

    if fun == "admin" {
        database();
    }
    else if fun == "project mgr" {
        project();
    }
    else if fun == "employee" {
        employee();
    }
    else if fun == "customer" {
        customer();
    }
    else if fun == "vendor" {
        dataplan();
    }
    else {
        println!("It seems you ran into an error!\nNext time; Type 'Admin', If you want to access as Administrator,
           Type 'Project mgr', If you want to access as a Project Manager,
           Type 'Employee', If you want access as an Employee,
           Type 'Customer', If you want to access as Customer,
           Type 'Vendor', If you want to access as Vendor");
    }
}
