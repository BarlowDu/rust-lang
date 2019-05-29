use crate::irun;

pub struct Runner{}
impl Runner{
    pub fn new()->Runner{
        Runner{}
    }
}
impl irun::IRunner for Runner {
    fn run(&self) {
        println!("string runner");
        let s=String::from("string runner");
        let _s1="string runner";
        println!("len:{}",first_word(&s));
        //println!("len:{}",first_word(&s1));
    }
}
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
