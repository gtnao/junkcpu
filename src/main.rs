use std::{
    sync::{Arc, Mutex},
    thread,
};

use junkcpu::{
    flip_flop::DFlipFlop,
    gate::NotGate,
    multiplexer::{OneBitMultiplexer, TwoBitDemultiplexer, TwoBitMultiplexer},
};

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

fn one_bit_cpu_sample() {
    let cpu = Arc::new(Mutex::new(OneBitCPU::new()));
    cpu.lock().unwrap().on();

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

struct OneBitFourRegisterCPU {
    dff: [DFlipFlop; 4],
    sy: (bool, bool),
    sx: (bool, bool),
}
impl OneBitFourRegisterCPU {
    pub fn new() -> OneBitFourRegisterCPU {
        OneBitFourRegisterCPU {
            dff: [
                DFlipFlop::new(),
                DFlipFlop::new(),
                DFlipFlop::new(),
                DFlipFlop::new(),
            ],
            sy: (false, false),
            sx: (false, false),
        }
    }
    pub fn on(&mut self) {
        self.dff[0].set_d(true);
    }
    pub fn set_sel(&mut self, sy: (bool, bool), sx: (bool, bool)) {
        self.sy = sy;
        self.sx = sx;
    }
    pub fn tick(&mut self, clk: bool) {
        for i in 0..4 {
            self.dff[i].set_clk(clk);
        }
        let two_bit_mux = TwoBitMultiplexer::new(
            self.dff[0].output(),
            self.dff[1].output(),
            self.dff[2].output(),
            self.dff[3].output(),
            self.sy,
        );
        let demux = TwoBitDemultiplexer::new(self.sx);
        let one_bit_mux0 =
            OneBitMultiplexer::new(self.dff[0].output(), two_bit_mux.output(), demux.output().0);
        let one_bit_mux1 =
            OneBitMultiplexer::new(self.dff[1].output(), two_bit_mux.output(), demux.output().1);
        let one_bit_mux2 =
            OneBitMultiplexer::new(self.dff[2].output(), two_bit_mux.output(), demux.output().2);
        let one_bit_mux3 =
            OneBitMultiplexer::new(self.dff[3].output(), two_bit_mux.output(), demux.output().3);
        self.dff[0].set_d(one_bit_mux0.output());
        self.dff[1].set_d(one_bit_mux1.output());
        self.dff[2].set_d(one_bit_mux2.output());
        self.dff[3].set_d(one_bit_mux3.output());
    }
    pub fn dump(&self) {
        for i in 0..4 {
            let output = self.dff[i].output();
            println!("Q{}: {}", i, output);
        }
    }
}

fn one_bit_four_register_cpu_sample() {
    let mut cpu = OneBitFourRegisterCPU::new();
    cpu.dump();
    println!("--------------------");
    cpu.on();
    println!("on");
    cpu.dump();
    println!("--------------------");
    cpu.tick(true);
    println!("tick");
    cpu.dump();
    println!("--------------------");
    println!("mov a, b");
    cpu.set_sel((false, false), (true, false));
    cpu.tick(false);
    cpu.dump();
    println!("--------------------");
    cpu.tick(true);
    println!("tick");
    cpu.dump();
    println!("--------------------");
    println!("mov b, c");
    cpu.set_sel((true, false), (false, true));
    cpu.tick(false);
    cpu.dump();
    println!("--------------------");
    cpu.tick(true);
    println!("tick");
    cpu.dump();
    println!("--------------------");
    println!("mov c, d");
    cpu.set_sel((false, true), (true, true));
    cpu.tick(false);
    cpu.dump();
    println!("--------------------");
    cpu.tick(true);
    println!("tick");
    cpu.dump();
    println!("--------------------");
}

fn main() {
    one_bit_four_register_cpu_sample();
}
