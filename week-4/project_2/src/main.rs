use std::io;

fn main() 
{
    let mut exp = String::new();
    let mut age = String::new();

    println!("\t\t\tWelcome To Greg. & Co. Interview Platform!");

    println!("Are you experienced/inexperienced(yes/no)");
    io::stdin().read_line(&mut exp).expect("Failed to read input");

    println!("How old are you?");
    io::stdin().read_line(&mut age).expect("Failed to read");
    let p:i32 = age.trim().parse().expect("Input not an integer");
    
    if  exp.trim() == "yes" && p >= 40{
        println!("Your Annual Incentive is N1,560,000");
    }
    else if exp.trim() == "yes" && p >= 30 && p < 40{
        println!("Your Annual Incentive is N1,480,000");
    } 
    else if exp.trim() == "yes" && p < 28 {
        println!("Your Annual Incentive is N1,300,000");
    }
    else if exp.trim() == "no" {
        println!("Your Annual Incentive is N100,000");
    }
}
