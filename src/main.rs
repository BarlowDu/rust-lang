
mod irun;
mod module;
use module::iborrow;
use module::ienum;
use module::ilifetime;
use module::istring;

use module::iborrow::Runner;
use core::ops::Deref;


use irun::IRunner;

fn main() {
    let mut runners: Vec<Box<dyn IRunner>> = Vec::new();
    runners.push(Box::new(ilifetime::Runner::new()));
    runners.push(Box::new(ienum::Runner::new()));
    runners.push(Box::new(istring::Runner::new()));
    runners.push(Box::new(iborrow::Runner::new()));
    for runner in runners {
        //&runner.deref()
        //run(runner.deref());
        runner.run();
    }
   
}

fn run(runner: &impl irun::IRunner) {
    runner.run();
}

