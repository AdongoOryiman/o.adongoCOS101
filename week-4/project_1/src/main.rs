use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("What is the equivalent of a in the equation?");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f64 = input1.trim().parse().expect("Not a valid string");

    println!("What is the equivalent of b in the equation?");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f64 = input2.trim().parse().expect("Not a valid string");

    println!("What is the equivalent of c in the equation?");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f64 = input3.trim().parse().expect("Not a valid string");
    
    let b = b.powf(2.0);
    let d = b - 4.0 * a * c;
    let di:f64 = b - 4.0 * a * c; 
    let mut ab = di.abs(); 
    ab = ab.sqrt();
    
    if d > 0.0 {
        let x_1:f64 = (-b + ab)/ 2.0 * a ;
        let x_2:f64 = (-b - ab)/ 2.0 * a ;
        println!("The Roots Of The Quadratic Equation are; {} & {} " ,x_1,x_2);
    }
    else if d == 0.0 {
        let x:f64 = -b / 2.0 * a ;
        println!("The Root Of The Quadratic Equation is {}",x);
    }
    else if d < 0.0 {
        let x_1:f64 = (-b + ab)/ 2.0 * a ;
        let x_2:f64 = (-b - ab)/ 2.0 * a ;
        println!("The Roots Of The Quadratic Equations are; {} & {}" ,x_1,x_2);
    }
}
