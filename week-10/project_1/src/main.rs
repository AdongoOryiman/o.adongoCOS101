struct PriceTag {
    cost:f32
}

impl PriceTag {
    fn sum(&self)->f32 {
        self.cost * 3.0
    }
}

fn main() {
    let hp = PriceTag {
        cost:650000.0
    };
    let ibm = PriceTag {
        cost:755000.0
    };
    let toshiba = PriceTag {
        cost:550000.0
    };
    let dell = PriceTag {
        cost:850000.0
    };

    let total = hp.sum() + ibm.sum() + toshiba.sum() + dell.sum();   
    println!("   Dear Esteemed Customer, thank you for your patience.
 Below is the total cost of your purchase;
 ${}",total);
}
