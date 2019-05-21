use super::super::irun;
//use irun;
//extern crate irun;
pub struct Runner{}

impl Runner{
    pub fn new()->Runner{
        return Runner{}
    }
}

impl irun::IRunner for Runner  {
    fn run(&self){
        println!("lifetime run")
    }
}