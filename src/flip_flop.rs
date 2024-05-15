use crate::gate::{NandGate, NotGate};

struct RSLatch {
    s: bool,
    r: bool,
    q: bool,
    nq: bool,
}
impl RSLatch {
    fn new(s: bool, r: bool) -> RSLatch {
        if (s && r) || (!s && !r) {
            panic!("Invalid input");
        }
        if s {
            RSLatch {
                s,
                r,
                q: true,
                nq: false,
            }
        } else {
            RSLatch {
                s,
                r,
                q: false,
                nq: true,
            }
        }
    }
    fn set_s(&mut self, s: bool) {
        self.s = s;
        let mut nand1 = NandGate::new(self.s, self.nq);
        let nand2 = NandGate::new(nand1.output(), self.r);
        nand1.set_b(nand2.output());
        self.q = nand1.output();
        self.nq = nand2.output();
    }
    fn set_r(&mut self, r: bool) {
        self.r = r;
        let mut nand2 = NandGate::new(self.q, self.r);
        let nand1 = NandGate::new(self.s, nand2.output());
        nand2.set_a(nand1.output());
        self.q = nand1.output();
        self.nq = nand2.output();
    }
    fn output(&self) -> (bool, bool) {
        (self.q, self.nq)
    }
}

pub struct DFlipFlop {
    d: bool,
    clk: bool,
    rs1: RSLatch,
    rs2: RSLatch,
}
impl DFlipFlop {
    pub fn new() -> DFlipFlop {
        DFlipFlop {
            d: false,
            clk: false,
            rs1: RSLatch::new(false, true),
            rs2: RSLatch::new(false, true),
        }
    }
    pub fn set_d(&mut self, d: bool) {
        self.d = d;
        self.update();
    }
    pub fn set_clk(&mut self, clk: bool) {
        self.clk = clk;
        self.update();
    }
    pub fn output(&self) -> bool {
        self.rs2.output().0
    }
    fn update(&mut self) {
        let clock_not1 = NotGate::new(self.clk);
        let clock_not2 = NotGate::new(clock_not1.output());
        let d_not = NotGate::new(self.d);
        let nand1 = NandGate::new(self.d, clock_not1.output());
        let nand2 = NandGate::new(d_not.output(), clock_not1.output());
        self.rs1.set_s(nand1.output());
        self.rs1.set_r(nand2.output());
        let nand3 = NandGate::new(self.rs1.output().0, clock_not2.output());
        let nand4 = NandGate::new(self.rs1.output().1, clock_not2.output());
        self.rs2.set_s(nand3.output());
        self.rs2.set_r(nand4.output());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rs_latch() {
        let mut rs_latch = RSLatch::new(true, false);
        assert_eq!(rs_latch.output(), (true, false));
        rs_latch.set_r(true);
        assert_eq!(rs_latch.output(), (true, false));
        rs_latch.set_r(false);
        assert_eq!(rs_latch.output(), (false, true));
        rs_latch.set_r(true);
        assert_eq!(rs_latch.output(), (false, true));
        rs_latch.set_s(false);
        assert_eq!(rs_latch.output(), (true, false));
        rs_latch.set_s(true);
        assert_eq!(rs_latch.output(), (true, false));
    }

    #[test]
    fn test_d_flip_flop() {
        let mut d_flip_flop = DFlipFlop::new();
        d_flip_flop.set_d(true);
        assert!(!d_flip_flop.output());
        d_flip_flop.set_clk(true);
        assert!(d_flip_flop.output());
        d_flip_flop.set_d(false);
        assert!(d_flip_flop.output());
        d_flip_flop.set_clk(false);
        assert!(d_flip_flop.output());
        d_flip_flop.set_clk(true);
        assert!(!d_flip_flop.output());
    }
}
