use std::{
    sync::{Arc, Mutex},
    thread,
};

use junkcpu::{flip_flop::DFlipFlop, gate::NotGate};

struct OneBitCPU {
    dff: DFlipFlop,
}
impl OneBitCPU {
    pub fn new() -> OneBitCPU {
        OneBitCPU {
            dff: DFlipFlop::new(),
        }
    }
    pub fn on(&mut self) {
        self.dff.set_d(true);
    }
    pub fn tick(&mut self, clk: bool) {
        self.dff.set_clk(clk);
        let ouput = self.dff.output();
        let not = NotGate::new(ouput);
        self.dff.set_d(not.output());
    }
    pub fn dump(&self) {
        let output = self.dff.output();
        println!("Q: {}", output);
    }
}

fn main() {
    let cpu = Arc::new(Mutex::new(OneBitCPU::new()));
    cpu.lock().unwrap().on();

    // thread clock
    let clock_sec = 2;
    let cpu_clone = cpu.clone();
    let mut clk = false;
    thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_secs(clock_sec));
        clk = !clk;
        cpu_clone.lock().unwrap().tick(clk);
        println!("tick {}", clk);
    });

    loop {
        cpu.lock().unwrap().dump();
        thread::sleep(std::time::Duration::from_millis(500));
    }
}
