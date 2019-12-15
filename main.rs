use std::fmt::Display;
use std::collections::HashMap;
#[derive(Debug)]
struct pair <T>{
    x:T,
    y:T,

}

impl <T> pair <T> {
    fn new (x:T, y:T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl <T: Display + PartialOrd> pair <T> {
    fn cmpDisplay(&self) {
        if self.x > self.y {
            println!("X:{} is greater than Y: {}",self.x,self.y);
        }
        else {
            println!("Y:{} is greater than X: {}",self.y,self.x);
        }


    }
}


fn main() {
    let mut h = HashMap::new();
    h.insert(1,2);
                //    HashMap does not follow partial Order trait
    let mut h1 = HashMap::new();
    h1.insert(3,4);


    let a = pair::new(-0.1,0.001);
    println!("{:#?}",a);
    a.cmpDisplay();  // there is a condition imposed for cmpDisplay method, partial Order and display trait must be implemented for type T
    //h.cmpDisplay(); // HashMap does not follow the partial order trait, therfore this statement will give an error.

    let b = pair::new(h,h1);  // in the new method function there are no restrictions i.e. 
    println!("{:#?}",b);
}


