use crate::irun;
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
        println!("lifetime run");
        let _node=get_node(1, "head");
    }
}
/************************ */
struct Node<'a>{
    id:i32,
    name:&'a str
}

fn get_node(id:i32,nm:&str)->Node{
    //Node{id:id,name:nm}
    Node{id:id,name:nm}
}
