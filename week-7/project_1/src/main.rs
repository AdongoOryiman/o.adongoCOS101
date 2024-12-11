use std::io;

fn tea() {
    let x = "APS 1-2";
    let v = "APS 3-5";
    let r = "APS 5-8";
    let q = "EL1 8-10";
    let f = "EL2 10-13";
    let g = "SES";
    let mut input6 = String::new();
    let tea = vec!["Placement","Classroom Teacher","Sn.Teacher","Leading Teacher","Dep.Princ.","Princ."];
    let ch = tea[0];
    let ch1 = tea[1];
    let ch2 = tea[2];
    let ch3 = tea[3];
    let ch4 = tea[4];
    let ch5 = tea[5];

    println!("How many years of working experience do you have?");
    io::stdin().read_line(&mut input6).expect("Wrong input");
    let c:u8 = input6.trim().parse().expect("That was incorrect!");
    if c == 1 || c == 2 {
        println!("You're a {:?} Teacher on level {}",ch,x);
    }
    else if c >= 3 && c < 5 {
        println!("You're a {} on level {}",ch1,v);
    }
    else if c >= 5 && c < 8 {
        println!("You're a {} on level {}",ch2,r);
    }
    else if c >= 8 && c < 10 {
        println!("You're a {} on level {}",ch3,q);
    }
    else if c >= 10 && c < 13 {
        println!("You're a {} on level {}",ch4,f);
    }
    else if c >= 13  {
        println!("You're a {} on level {}",ch5,g);
    }
}


fn law() {
    let x = "APS 1-2";
    let v = "APS 3-5";
    let r = "APS 5-8";
    let q = "EL1 8-10";
    let f = "EL2 10-13";
    let g = "SES";
    let mut input5 = String::new();
    let law = vec!["Paralegal","Jn.Ass.","Ass.","Sn.Ass.1-2","Sn.Ass.3-4","Partner"];
    let ch = law[0];
    let ch1 = law[1];
    let ch2 = law[2];
    let ch3 = law[3];
    let ch4 = law[4];
    let ch5 = law[5];

    println!("How many years of working experience do you have?");
    io::stdin().read_line(&mut input5).expect("Wrong input");
    let c:u8 = input5.trim().parse().expect("That was incorrect!");
    if c == 1 || c == 2 {
        println!("You're a {:?} on level {}",ch,x);
    }
    else if c >= 3 && c < 5 {
        println!("You're a {} on level {}",ch1,v);
    }
    else if c >= 5 && c < 8 {
        println!("You're an {} on level {}",ch2,r);
    }
    else if c >= 8 && c < 10 {
        println!("You're a {} on level {}",ch3,q);
    }
    else if c >= 10 && c < 13 {
        println!("You're a {} on level {}",ch4,f);
    }
    else if c >= 13  {
        println!("You're a {} on level {}",ch5,g);
    }
}

fn aca() {
    let v = "APS 3-5";
    let r = "APS 5-8";
    let q = "EL1 8-10";
    let f = "EL2 10-13";
    let g = "SES";
    let mut input4 = String::new();
    let acad = vec!["Res.Asst","PhD.Cand","PDR","Sn.Lec","Dean"];
    let ch = acad[0];
    let ch1 = acad[1];
    let ch2 = acad[2];
    let ch3 = acad[3];
    let ch4 = acad[4];
    println!("How many years of working experience do you have?");
    io::stdin().read_line(&mut input4).expect("Wrong input");
    let c:u8 = input4.trim().parse().expect("That was incorrect!");
    if c >= 3 && c < 5 {
        println!("You're a {} on level {}",ch,v);
    }
    else if c >= 5 && c < 8 {
        println!("You're a {} on level {}",ch1,r);
    }
    else if c >= 8 && c < 10 {
        println!("You're a {} on level {}",ch2,q);
    }
    else if c >= 10 && c < 13 {
        println!("You're a {} on level {}",ch3,f);
    }
    else if c >= 13  {
        println!("You're a {} on level {}",ch4,g);
    }
}

fn off_adm() {
    let x = "APS 1-2";
    let v = "APS 3-5";
    let r = "APS 5-8";
    let q = "EL1 8-10";
    let f = "EL2 10-13";
    let g = "SES";
    let mut input3 = String::new();
    let off_admin = vec!["Intern","Admin","Sn.Admin","Ofc.Mgr","Dir","CEO"];
    let ch = off_admin[0];
    let ch1 = off_admin[1];
    let ch2 = off_admin[2];
    let ch3 = off_admin[3];
    let ch4 = off_admin[4];
    let ch5 = off_admin[5];
    println!("How many years of working experience do you have?");
    io::stdin().read_line(&mut input3).expect("Wrong input");
    let c:u8 = input3.trim().parse().expect("That was incorrect!");
    if c == 1 || c == 2 {
        println!("You're an {:?} on level {}",ch,x);
    }
    else if c >= 3 && c < 5 {
        println!("You're an {} on level {}",ch1,v);
    }
    else if c >= 5 && c < 8 {
        println!("You're a {} on level {}",ch2,r);
    }
    else if c >= 8 && c < 10 {
        println!("You're an {} on level {}",ch3,q);
    }
    else if c >= 10 && c < 13 {
        println!("You're a {} on level {}",ch4,f);
    }
    else if c >= 13  {
        println!("You're a {} on level {}",ch5,g);
    }
}

fn class() {
    let mut input2 = String::new();
    println!("Kindly input the number you fall under (1)Office Admin., (2)Academic, (3)Lawyer and, (4) Teacher.");
    io::stdin().read_line(&mut input2).expect("Wrong input");
    if input2.trim() == "1" {
        off_adm();
    }
    else if input2.trim() == "2" {
        aca();
    }
    else if input2.trim() == "3" {
        law();
    }
    else if input2.trim() == "4" {
        tea();
    }
}

fn main() {

    let mut input1 = String::new();

    println!("\nWelcome to the FGN APS Level Checker
              \n Are you interested in validating your staff level");
    io::stdin().read_line(&mut input1).expect("Wrong input");

    if input1.trim().to_lowercase() == "yes" {
        class();
    }
    else {
        println!("Thank you for your time, have a nice day!");
    }
}
