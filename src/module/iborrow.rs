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
        testBorrow();
        testMutBorrow();
    }
}

fn testReference(){

    let mut s=String::from("rust language");
    let s1=&s;
    //s.clear(); //error
    println!("{}",*s1)
}
/*
当mut变量a被不可变(b)引用后,
a将变成不可变(条件是b在a变更后被使用),只到b被释放

当b引用了a,但a发生了变更,那么b可能将无法正确获取到数据.
无同步机制,或者无需同步机制
 */
fn testBorrow(){
    let mut s=String::from("hello rust");
    {
        let s1=&s;
        s.push_str("!");
        //println!("{}",s1);//如果s1被使用,那么上一句将不能编译通过(complie error)
    }


}
fn testMutBorrow(){
    let mut s=String::from("rust language");
    {
        let s2=&mut s;
        //let s3=&s; //complie error
        let s4=&s2;//这一句之所有能够编译通过,是因为s2在这时可能变成了只读;  
        let l=s4.len();  
        s2.push_str(" !");
        println!("s2:{}",s2);
        println!("s:{}",s);
        //上两句顺序为何不能互换:
        //一个可变变量只能有一个可变引用,或者多个不可变引用
        //println!("s:{}",s) 会传递s的一个不可用引用
        
        //let l=s4.len();    
    }
}