use crate::gate::{AndGate, NorGate, NotGate, OrGate};

pub struct OneBitMultiplexer {
    a: bool,
    b: bool,
    sel: bool,
}

impl OneBitMultiplexer {
    pub fn new(a: bool, b: bool, sel: bool) -> OneBitMultiplexer {
        OneBitMultiplexer { a, b, sel }
    }
    fn set_a(&mut self, a: bool) {
        self.a = a;
    }
    fn set_b(&mut self, b: bool) {
        self.b = b;
    }
    fn set_sel(&mut self, sel: bool) {
        self.sel = sel;
    }
    pub fn output(&self) -> bool {
        let not = NotGate::new(self.sel);
        let and1 = AndGate::new(self.a, not.output());
        let and2 = AndGate::new(self.b, self.sel);
        let or = OrGate::new(and1.output(), and2.output());
        or.output()
    }
}

pub struct TwoBitMultiplexer {
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    sel: (bool, bool),
}
impl TwoBitMultiplexer {
    pub fn new(a: bool, b: bool, c: bool, d: bool, sel: (bool, bool)) -> TwoBitMultiplexer {
        TwoBitMultiplexer { a, b, c, d, sel }
    }
    fn set_a(&mut self, a: bool) {
        self.a = a;
    }
    fn set_b(&mut self, b: bool) {
        self.b = b;
    }
    fn set_c(&mut self, c: bool) {
        self.c = c;
    }
    fn set_d(&mut self, d: bool) {
        self.d = d;
    }
    pub fn set_sel(&mut self, sel: (bool, bool)) {
        self.sel = sel;
    }
    pub fn output(&self) -> bool {
        let mux1 = OneBitMultiplexer::new(self.a, self.b, self.sel.0);
        let mux2 = OneBitMultiplexer::new(self.c, self.d, self.sel.0);
        let mux3 = OneBitMultiplexer::new(mux1.output(), mux2.output(), self.sel.1);
        mux3.output()
    }
}

pub struct TwoBitDemultiplexer {
    sel: (bool, bool),
}
impl TwoBitDemultiplexer {
    pub fn new(sel: (bool, bool)) -> TwoBitDemultiplexer {
        TwoBitDemultiplexer { sel }
    }
    pub fn set_sel(&mut self, sel: (bool, bool)) {
        self.sel = sel;
    }
    pub fn output(&self) -> (bool, bool, bool, bool) {
        let not1 = NotGate::new(self.sel.0);
        let not2 = NotGate::new(self.sel.1);
        let not3 = NotGate::new(not1.output());
        let not4 = NotGate::new(not2.output());
        let nor1 = NorGate::new(not4.output(), not3.output());
        let nor2 = NorGate::new(not4.output(), not1.output());
        let nor3 = NorGate::new(not2.output(), not3.output());
        let nor4 = NorGate::new(not2.output(), not1.output());
        (nor1.output(), nor2.output(), nor3.output(), nor4.output())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_bit_multiplexer() {
        let mut mux = OneBitMultiplexer::new(true, false, false);
        assert!(mux.output());
        mux.set_sel(true);
        assert!(!mux.output());
    }

    #[test]
    fn test_two_bit_multiplexer() {
        let mut mux = TwoBitMultiplexer::new(false, false, false, false, (false, false));
        mux.set_a(true);
        assert!(mux.output());
        mux.set_a(false);
        mux.set_b(true);
        mux.set_sel((true, false));
        assert!(mux.output());
        mux.set_b(false);
        mux.set_c(true);
        mux.set_sel((false, true));
        assert!(mux.output());
        mux.set_c(false);
        mux.set_d(true);
        mux.set_sel((true, true));
        assert!(mux.output());
    }

    #[test]
    fn test_two_bit_demultiplexer() {
        let mut demux = TwoBitDemultiplexer::new((false, false));
        assert_eq!(demux.output(), (true, false, false, false));
        demux.set_sel((true, false));
        assert_eq!(demux.output(), (false, true, false, false));
        demux.set_sel((false, true));
        assert_eq!(demux.output(), (false, false, true, false));
        demux.set_sel((true, true));
        assert_eq!(demux.output(), (false, false, false, true));
    }
}
