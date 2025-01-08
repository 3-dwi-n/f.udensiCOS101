struct laptop {
brand: string,
price:u32,
}
impl laptop {
fn final_cost{&self, volume:u32 } -> u32 {
    self.price * volume
}



}
fn main(){
let hp= laptop{
    brand: String::from ("HP"),
      price: 650_000,
};
let ibm= laptop{
brand:String::from("IBM"),
price: 755_000,
};
let toshiba = laptop{
brand:String::from ("TOSHIBA")
price:550_000,
};
let dell = laptop{
brand:String::from ("DELL")
price: 850_000,
};
let final_cost=hp.final_cost*3 + ibm.final_cost*3 +toshiba.final_cost*3 +dell.final_cost*3


println("the final cost of 3 laptops from each brand is N{}",total_cost:)

}
