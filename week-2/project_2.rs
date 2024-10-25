fn main() {
let t:f64 = 450000.0; //t for toshiba
let m:f64 = 1500000.0; //m for mac
let h:f64 = 750000.0; //h for hp
let d:f64 = 2850000.0; //d for dell
let ac:f64 = 250000.0; //ac for acer
let s = (t*2.0)+m+(h*3.0)+(d*3.0)+ac;
println!("Sum is {}",s);
let a = s/(5.0);//the no.s listed are the qty of each item
println!("Average is {}", a);
}