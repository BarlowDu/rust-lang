mod module;
mod irun;

use module::ilifetime;
use module::ienum;
use module::istring;
use module::iborrow;

use irun::IRunner;

fn main() {
    let mut runners:Vec<Box<dyn IRunner>>=Vec::new();
    runners.push(Box::new(ilifetime::Runner::new()));
    runners.push(Box::new(ienum::Runner::new()));
    runners.push(Box::new(istring::Runner::new()));
    runners.push(Box::new(iborrow::Runner::new()));
    for runner in runners{
        runner.run()
    } 
}

