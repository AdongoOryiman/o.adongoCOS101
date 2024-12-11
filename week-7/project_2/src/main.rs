use std::io;

fn main() {

    let mut no_dev = String::new();
    let mut process = Vec::new();
    let mut cycle = Vec::new();

    println!("How many developers would you like to evaluate?");
    io::stdin().read_line(&mut no_dev).expect("That's a wrong input!");
    let nd:u32 = no_dev.trim().parse().expect("Incorrect input");

    for crew in 1..= nd {
        let mut dev_name = String::new();
        let mut exp = String::new();

        println!("\nKindly input the name for #developer{}",crew);
        io::stdin().read_line(&mut dev_name).expect("That's a wrong input!");

        println!("How many years has this developer worked?");
        io::stdin().read_line(&mut exp).expect("That's a wrong input!");

        process.push(dev_name);
        cycle.push(exp);
    }

    match cycle.iter().max() {
        Some(max_num) => println!("\nWhoever has {}years of working experience has the most years of working experience", max_num),
        None => println!("No brainer"),
    }

    println!("\nBelow is a list of developers with the number of years they have worked");
    
    for x in 0..no_dev.len() {
        println!("{} With total work experience years of {} ",process[x],cycle[x] );
    }
}
