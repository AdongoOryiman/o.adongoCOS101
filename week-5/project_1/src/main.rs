use std::io;

fn main() {

    let mut input_1 = String::new();
    let mut input_2 = String::new();
    let mut input_3 = String::new();
   
    println!("\t\t\t\t\tWelcome To Fredds Fast Foods!");
    println!("Would you like to have a look at our menu?");
    io::stdin().read_line(&mut input_1).expect("That was a wrong input,please have another go let's see");

    if input_1.trim() == "yes" {
        println!("\nKindly Input;
                  \nP for Poundo & Edinkaiko soup - N3,200
                  \nF for Fried Rice & Chicken - N3,000
                  \nA for Amala & Ewedu soup - N2,500
                  \nE for Eba & Egusi soup - N2,000
                  \nW for White Rice & Stew - N2,500
                  \nKINDLY NOTE THAT WE TAKE ONLY ONE ORDER AT A TIME!");
        println!("What would you like to order?");
        io::stdin().read_line(&mut input_2).expect("That was a wrong input,please have another go let's see");

        println!("How many portions would you like?");
        io::stdin().read_line(&mut input_3).expect("That was a wrong input,please have another go let's see");
         let t:f64 = input_3.trim().parse().expect("That is incorrect!");

        if input_2.trim() == "P"{
         let p:f64 = 3200.0;
         let calc:f64 = p * t;
         if calc > 10000.0 {
            let v:f64 = 5.0/100.0;
            let r:f64 = v * calc;
            let dis:f64 = calc - r;
            println!("We have a 5% discount for purshases above N10000.00, so your total cost is {}", dis);
            println!("Thanks for choosing Fredds Fast Foods! Have a nice day!");
         }
          else if calc < 10000.0 {
            println!("That will cost {}",calc);
            println!("Thanks for choosing Fredds Fast Foods! Have a nice day!");
            }
          } 
        else if input_2.trim() == "F"{
         let f:f64 = 3000.0;
         let calc:f64 = f * t;
         if calc > 10000.0 {
            let v:f64 = 5.0/100.0;
            let r:f64 = v * calc;
            let dis:f64 = calc - r;
            println!("We have a 5% discount for purshases above N10000.00, so your total cost is {}", dis);
            println!("Thanks for choosing Fredds Fast Foods! Have a nice day!");
         }
          else if calc < 10000.0 {
            println!("That will cost {}",calc);
            println!("Thanks for choosing Fredds Fast Foods! Have a nice day!");
            }
          }
        else if input_2.trim() == "A"{
         let a:f64 = 2500.0;
         let calc:f64 = a * t;
         if calc > 10000.0 {
            let v:f64 = 5.0/100.0;
            let r:f64 = v * calc;
            let dis:f64 = calc - r;
            println!("We have a 5% discount for purshases above N10000.00, so your total cost is {}", dis);
            println!("Thanks for choosing Fredds Fast Foods! Have a nice day!");
         }
          else if calc < 10000.0 {
            println!("That will cost {}",calc);
            println!("Thanks for choosing Fredds Fast Foods! Have a nice day!");
            }
          }
        else if input_2.trim() == "E"{
         let e:f64 = 2000.0;
         let calc:f64 = e * t;
         if calc > 10000.0 {
            let v:f64 = 5.0/100.0;
            let r:f64 = v * calc;
            let dis:f64 = calc - r;
            println!("We have a 5% discount for purshases above N10000.00, so your total cost is {}", dis);
            println!("Thanks for choosing Fredds Fast Foods! Have a nice day!");
         }
          else if calc < 10000.0 {
            println!("That will cost {}",calc);
            println!("Thanks for choosing Fredds Fast Foods! Have a nice day!");
            }
          }
        else if input_2.trim() == "W"{
         let w:f64 = 2500.0;
         let calc:f64 = w * t;
          if calc > 10000.0 {
            let v:f64 = 5.0/100.0;
            let r:f64 = v * calc;
            let dis:f64 = calc - r;
            println!("We have a 5% discount for purshases above N10000.00, so your total cost is {}", dis);
            println!("Thanks for choosing Fredds Fast Foods! Have a nice day!");
         }
          else if calc < 10000.0 {
            println!("That will cost {}",calc);
            println!("Thanks for choosing Fredds Fast Foods! Have a nice day!");
         }
          }
        }
    else if input_1.trim() == "no" {
        println!("\nWhat would you like from our menu?");
        io::stdin().read_line(&mut input_2).expect("That was a wrong input,please have another go let's see");

        println!("How many portions would you like?");
        io::stdin().read_line(&mut input_3).expect("That was a wrong input,please have another go let's see");
         let t:f64 = input_3.trim().parse().expect("That is incorrect!");

         if input_2.trim() == "P"{
         let p:f64 = 3200.0;
         let calc:f64 = p * t;
         if calc > 10000.0 {
            let v:f64 = 5.0/100.0;
            let r:f64 = v * calc;
            let dis:f64 = calc - r;
            println!("We have a 5% discount for purshases above N10000.00, so your total cost is {}", dis);
            println!("Thanks for choosing Fredds Fast Foods! Have a nice day!");
         }
          else if calc < 10000.0 {
            println!("That will cost {}",calc);
            println!("Thanks for choosing Fredds Fast Foods! Have a nice day!");
          } 
        }
        else if input_2.trim() == "F"{
         let f:f64 = 3000.0;
         let calc:f64 = f * t;
         if calc > 10000.0 {
            let v:f64 = 5.0/100.0;
            let r:f64 = v * calc;
            let dis:f64 = calc - r;
            println!("We have a 5% discount for purshases above N10000.00, so your total cost is {}", dis);
            println!("Thanks for choosing Fredds Fast Foods! Have a nice day!");
         }
          else if calc < 10000.0 {
            println!("That will cost {}",calc);
            println!("Thanks for choosing Fredds Fast Foods! Have a nice day!");
          }
        } 
        else if input_2.trim() == "A"{
         let a:f64 = 2500.0;
         let calc:f64 = a * t;
         if calc > 10000.0 {
            let v:f64 = 5.0/100.0;
            let r:f64 = v * calc;
            let dis:f64 = calc - r;
            println!("We have a 5% discount for purshases above N10000.00, so your total cost is {}", dis);
            println!("Thanks for choosing Fredds Fast Foods! Have a nice day!");
         }
          else if calc < 10000.0 {
            println!("That will cost {}",calc);
            println!("Thanks for choosing Fredds Fast Foods! Have a nice day!");
            }
          }
        else if input_2.trim() == "E"{
         let e:f64 = 2000.0;
         let calc:f64 = e * t;
         if calc > 10000.0 {
            let v:f64 = 5.0/100.0;
            let r:f64 = v * calc;
            let dis:f64 = calc - r;
            println!("We have a 5% discount for purshases above N10000.00, so your total cost is {}", dis);
            println!("Thanks for choosing Fredds Fast Foods! Have a nice day!");
         }
          else if calc < 10000.0 {
            println!("That will cost {}",calc);
            println!("Thanks for choosing Fredds Fast Foods! Have a nice day!");
            }
          }
        else if input_2.trim() == "W"{
         let w:f64 = 2500.0;
         let calc:f64 = w * t;
         if calc > 10000.0 {
            let v:f64 = 5.0/100.0;
            let r:f64 = v * calc;
            let dis:f64 = calc - r;
            println!("We have a 5% discount for purshases above N10000.00, so your total cost is {}", dis);
            println!("Thanks for choosing Fredds Fast Foods! Have a nice day!");
         }
          else if calc < 10000.0 {
            println!("That will cost {}",calc);
            println!("Thanks for choosing Fredds Fast Foods! Have a nice day!");
            }
          }
    }
}