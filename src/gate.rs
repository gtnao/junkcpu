pub struct NandGate {
    a: bool,
    b: bool,
}
impl NandGate {
    pub fn new(a: bool, b: bool) -> NandGate {
        NandGate { a, b }
    }
    pub fn set_a(&mut self, a: bool) {
        self.a = a;
    }
    pub fn set_b(&mut self, b: bool) {
        self.b = b;
    }
    pub fn output(&self) -> bool {
        !(self.a && self.b)
    }
}

pub struct NotGate {
    a: bool,
}
impl NotGate {
    pub fn new(a: bool) -> NotGate {
        NotGate { a }
    }
    pub fn set_a(&mut self, a: bool) {
        self.a = a;
    }
    pub fn output(&self) -> bool {
        let nand = NandGate::new(self.a, self.a);
        nand.output()
    }
}

pub struct AndGate {
    a: bool,
    b: bool,
}
impl AndGate {
    pub fn new(a: bool, b: bool) -> AndGate {
        AndGate { a, b }
    }
    pub fn set_a(&mut self, a: bool) {
        self.a = a;
    }
    pub fn set_b(&mut self, b: bool) {
        self.b = b;
    }
    pub fn output(&self) -> bool {
        let nand = NandGate::new(self.a, self.b);
        let not = NotGate::new(nand.output());
        not.output()
    }
}

pub struct OrGate {
    a: bool,
    b: bool,
}
impl OrGate {
    pub fn new(a: bool, b: bool) -> OrGate {
        OrGate { a, b }
    }
    pub fn set_a(&mut self, a: bool) {
        self.a = a;
    }
    pub fn set_b(&mut self, b: bool) {
        self.b = b;
    }
    pub fn output(&self) -> bool {
        let not1 = NotGate::new(self.a);
        let not2 = NotGate::new(self.b);
        let nand = NandGate::new(not1.output(), not2.output());
        nand.output()
    }
}

pub struct NorGate {
    a: bool,
    b: bool,
}
impl NorGate {
    pub fn new(a: bool, b: bool) -> NorGate {
        NorGate { a, b }
    }
    pub fn set_a(&mut self, a: bool) {
        self.a = a;
    }
    pub fn set_b(&mut self, b: bool) {
        self.b = b;
    }
    pub fn output(&self) -> bool {
        let or = OrGate::new(self.a, self.b);
        let not = NotGate::new(or.output());
        not.output()
    }
}

pub struct XorGate {
    a: bool,
    b: bool,
}
impl XorGate {
    pub fn new(a: bool, b: bool) -> XorGate {
        XorGate { a, b }
    }
    pub fn set_a(&mut self, a: bool) {
        self.a = a;
    }
    pub fn set_b(&mut self, b: bool) {
        self.b = b;
    }
    pub fn output(&self) -> bool {
        let nand1 = NandGate::new(self.a, self.b);
        let output1 = nand1.output();
        let nand2 = NandGate::new(self.a, output1);
        let nand3 = NandGate::new(output1, self.b);
        let nand4 = NandGate::new(nand2.output(), nand3.output());
        nand4.output()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nand_gate() {
        let mut nand = NandGate::new(true, false);
        assert!(nand.output());
        nand.set_a(false);
        nand.set_b(true);
        assert!(nand.output());
        nand.set_a(false);
        nand.set_b(false);
        assert!(nand.output());
        nand.set_a(true);
        nand.set_b(true);
        assert!(!nand.output());
    }

    #[test]
    fn test_not_gate() {
        let mut not = NotGate::new(true);
        assert!(!not.output());
        not.set_a(false);
        assert!(not.output());
    }

    #[test]
    fn test_and_gate() {
        let mut and = AndGate::new(true, false);
        assert!(!and.output());
        and.set_a(false);
        and.set_b(true);
        assert!(!and.output());
        and.set_a(false);
        and.set_b(false);
        assert!(!and.output());
        and.set_a(true);
        and.set_b(true);
        assert!(and.output());
    }

    #[test]
    fn test_or_gate() {
        let mut or = OrGate::new(true, false);
        assert!(or.output());
        or.set_a(false);
        or.set_b(true);
        assert!(or.output());
        or.set_a(false);
        or.set_b(false);
        assert!(!or.output());
        or.set_a(true);
        or.set_b(true);
        assert!(or.output());
    }

    #[test]
    fn test_nor_gate() {
        let mut nor = NorGate::new(true, false);
        assert!(!nor.output());
        nor.set_a(false);
        nor.set_b(true);
        assert!(!nor.output());
        nor.set_a(false);
        nor.set_b(false);
        assert!(nor.output());
        nor.set_a(true);
        nor.set_b(true);
        assert!(!nor.output());
    }

    #[test]
    fn test_xor_gate() {
        let mut xor = XorGate::new(true, false);
        assert!(xor.output());
        xor.set_a(false);
        xor.set_b(true);
        assert!(xor.output());
        xor.set_a(false);
        xor.set_b(false);
        assert!(!xor.output());
        xor.set_a(true);
        xor.set_b(true);
        assert!(!xor.output());
    }
}
