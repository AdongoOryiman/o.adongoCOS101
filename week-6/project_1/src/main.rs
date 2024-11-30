use std::io;

fn trap(){
    let mut height = String::new();
    let mut base_1 = String::new();
    let mut base_2 = String::new();

    println!("\nKindly input height");
    io::stdin().read_line(&mut height).expect("That was a wrong input. Have another go!");
    let h:f32 = height.trim().parse().expect("That is not an acceptable input! Please try again");

    println!("Kindly input base one");
    io::stdin().read_line(&mut base_1).expect("That was a wrong input. Have another go!");
    let b_1:f32 = base_1.trim().parse().expect("That is not an acceptable input! Please try again");

    println!("Kindly input base two");
    io::stdin().read_line(&mut base_2).expect("That was a wrong input. Have another go!");
    let b_2:f32 = base_2.trim().parse().expect("That is not an acceptable input! Please try again");

    let area:f32 = (h/2.0)*(b_1+b_2);
    println!("Area of Trapezium = {}",area );
}

fn rho(){
    let mut diag_1 = String::new();
    let mut diag_2 = String::new();

    println!("\nWhat is the value for the first diagonal?");
    io::stdin().read_line(&mut diag_1).expect("That was a wrong input. Have another go!");
    let d_1:f32 = diag_1.trim().parse().expect("That is not an acceptable input! Please try again");

    println!("What is the value for the second diagonal?");
    io::stdin().read_line(&mut diag_2).expect("That was a wrong input. Have another go!");
    let d_2:f32 = diag_2.trim().parse().expect("That is not an acceptable input! Please try again");

    let area:f32 = (1.0/2.0)*(d_1*d_2);
    println!("Area of Rhombus = {}",area );
}

fn par(){
    let mut base = String::new();
    let mut alt = String::new();

    println!("\nKindly input the value for base");
    io::stdin().read_line(&mut base).expect("That was a wrong input. Have another go!");
    let b:f32 = base.trim().parse().expect("That is not an acceptable input! Please try again");

    println!("What is the altitude?(perpendicular distance btw. the base & opp. side)");
    io::stdin().read_line(&mut alt).expect("That was a wrong input. Have another go!");
    let a:f32 = alt.trim().parse().expect("That is not an acceptable input! Please try again");

    let area:f32 = b*a;
    println!("Area of Parallelogram = {}",area );
}

fn cube(){
    let mut length = String::new();

    println!("\nKindly input the equivalent of length");
    io::stdin().read_line(&mut length).expect("That was a wrong input. Have another go!");
    let l:f32 = length.trim().parse().expect("That is not an acceptable input! Please try again");

    let lg:f32 = l.powf(2.0);
    let area:f32 = 6.0*lg;
    println!("Area of Cube = {}",area );
}

fn cyl(){
    let mut height = String::new();
    let mut rad = String::new();

    println!("\nKindly input the value for height");
    io::stdin().read_line(&mut height).expect("That was a wrong input. Have another go!");
    let h:f32 = height.trim().parse().expect("That is not an acceptable input! Please try again");

    println!("What is the radius?");
    io::stdin().read_line(&mut rad).expect("That was a wrong input. Have another go!");
    let r:f32 = rad.trim().parse().expect("That is not an acceptable input! Please try again");

    let s:f32 = r.powf(2.0);
    let tt:f32 = 22.0/7.0;
    let vol:f32 = tt*s*h;
    println!("Volume of Cylinder = {}",vol );
}

fn main() {
    let mut input_1 = String::new();

    println!("Hello there! Kindly select the equation no. of your choice.");

    println!("\n1.Area of Trapezium
              \n2.Area of a Rhombus
              \n3.Area of a Parallelogram
              \n4.Area of a Cube
              \n5.Volume of a Cylinder");
    io::stdin().read_line(&mut input_1).expect("That was a wrong input. Have another go!");

    if input_1.trim() == "1" {
        trap();
    }
    else if input_1.trim() == "2" {
        rho();
    }
    else if input_1.trim() == "3" {
        par();
    }
    else if input_1.trim() == "4" {
        cube();
    }
    else if input_1.trim() == "5" {
        cyl();
    }
}
