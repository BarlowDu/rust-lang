use super::super::irun;

pub struct Runner {}
impl Runner{
    pub fn new()->Runner{
        Runner{}
    }
}

impl irun::IRunner for Runner{
    fn run(&self){
        println!("enum run");
        test_type_and_value_match();
    }

}


fn test_type_and_value_match(){
    let i:i32=1;
    let re:Result<i32,i32>=Result::Ok(1);
    let r=Some(1);
    
    match re{
        Ok(1)=>println!("result match 1"),
        Ok(t)=>println!("result match t"),
        Err(t)=>println!("result match none")
    }

    
    match r{
        Some(1)=>println!("option match 1"),
        Some(t)=>println!("option match t"),
        None=>println!("option match none")
    }
}