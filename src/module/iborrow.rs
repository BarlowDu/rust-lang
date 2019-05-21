use super::super::irun;

pub struct Runner{}

impl Runner{
    pub  fn new()->Runner{
        Runner{}
    }
}

impl irun::IRunner for Runner{
    fn run(&self){
        testReference();
        testMutBorrow();
    }
}

fn testReference(){

    let mut s=String::from("rust language");
    let s1=&s;
    //s.clear(); //error
    println!("{}",*s1)
}
fn testMutBorrow(){
    let mut s=String::from("rust language");
    let s2=&mut s;
    s2.push_str(" hello");
    println!("s:{}",s);
    //println!("s2:{}",s2); //error
}