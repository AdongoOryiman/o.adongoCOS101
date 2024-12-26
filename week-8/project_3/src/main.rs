fn main() {
 let header = vec!["Name of Commisioner","      Ministry","          Geopolitical Zone"];
    for pew in &header{print!("{}",pew);}
     println!("");

 let min = vec!["    Internal Affairs","      Justice"," Defense","     Power & Steel","     Petroleum"];
  let one = min[0];
  let two = min[1];
  let three = min[2];
  let four = min[3];
  let five = min[4];

 let geo = vec!["  South West","           North East","           South South","     South West","         South East"];
  let a = geo[0];
  let b = geo[1];
  let c = geo[2];
  let d = geo[3];
  let e = geo[4];

 let container = vec![vec!["Aigbogun Alamba Dauda"],vec!["Murtala Afeez Bendu"],vec!["Okorocha Calistus Obgona"],vec!["Adewale Jimoh Akandi"],vec!["Osazuwa Faith Etieye"]];

  let mut clm_1 = container[0].clone();
   clm_1.push(&one);
   clm_1.push(&a);
    for line in &clm_1{print!("{}",line);}
     println!("");

  let mut clm_2 = container[1].clone();
   clm_2.push(&two);
   clm_2.push(&b);
    for row in &clm_2{print!("{}",row);}
     println!("");

  let mut clm_3 = container[2].clone();
   clm_3.push(&three);
   clm_3.push(&c);
    for record in &clm_3{print!("{}",record);}
     println!("");

  let mut clm_4 = container[3].clone();
   clm_4.push(&four);
   clm_4.push(&d);
    for entry in &clm_4{print!("{}",entry);}
     println!("");

  let mut clm_5 = container[4].clone();
   clm_5.push(&five);
   clm_5.push(&e);
    for tuple in &clm_5{print!("{}",tuple);}
     println!("");
}
