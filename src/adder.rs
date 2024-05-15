use crate::gate::{AndGate, OrGate, XorGate};

pub struct HalfAdder {
    a: bool,
    b: bool,
}
impl HalfAdder {
    pub fn new(a: bool, b: bool) -> Self {
        Self { a, b }
    }
    pub fn output(&self) -> (bool, bool) {
        let xor = XorGate::new(self.a, self.b);
        let and = AndGate::new(self.a, self.b);
        (xor.output(), and.output())
    }
}

pub struct FullAdder {
    a: bool,
    b: bool,
    carry_in: bool,
}
impl FullAdder {
    pub fn new(a: bool, b: bool, carry_in: bool) -> Self {
        Self { a, b, carry_in }
    }
    pub fn output(&self) -> (bool, bool) {
        let half_adder1 = HalfAdder::new(self.a, self.b);
        let half_adder1_output = half_adder1.output();
        let half_adder2 = HalfAdder::new(half_adder1_output.0, self.carry_in);
        let half_adder2_output = half_adder2.output();
        let or = OrGate::new(half_adder1_output.1, half_adder2_output.1);
        (half_adder2_output.0, or.output())
    }
}

pub struct NBitFullAdder<const N: usize> {
    a: [bool; N],
    b: [bool; N],
}
impl<const N: usize> NBitFullAdder<N> {
    pub fn new(a: [bool; N], b: [bool; N]) -> Self {
        Self { a, b }
    }
    pub fn output(&self) -> ([bool; N], bool) {
        let mut carry = false;
        let mut result = [false; N];
        (0..N).for_each(|i| {
            let full_adder = FullAdder::new(self.a[i], self.b[i], carry);
            let full_adder_output = full_adder.output();
            result[i] = full_adder_output.0;
            carry = full_adder_output.1;
        });
        (result, carry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_half_adder() {
        let half_adder = HalfAdder::new(false, false);
        assert_eq!(half_adder.output(), (false, false));

        let half_adder = HalfAdder::new(false, true);
        assert_eq!(half_adder.output(), (true, false));

        let half_adder = HalfAdder::new(true, false);
        assert_eq!(half_adder.output(), (true, false));

        let half_adder = HalfAdder::new(true, true);
        assert_eq!(half_adder.output(), (false, true));
    }

    #[test]
    fn test_full_adder() {
        let full_adder = FullAdder::new(false, false, false);
        assert_eq!(full_adder.output(), (false, false));

        let full_adder = FullAdder::new(false, false, true);
        assert_eq!(full_adder.output(), (true, false));

        let full_adder = FullAdder::new(false, true, false);
        assert_eq!(full_adder.output(), (true, false));

        let full_adder = FullAdder::new(false, true, true);
        assert_eq!(full_adder.output(), (false, true));

        let full_adder = FullAdder::new(true, false, false);
        assert_eq!(full_adder.output(), (true, false));

        let full_adder = FullAdder::new(true, false, true);
        assert_eq!(full_adder.output(), (false, true));

        let full_adder = FullAdder::new(true, true, false);
        assert_eq!(full_adder.output(), (false, true));

        let full_adder = FullAdder::new(true, true, true);
        assert_eq!(full_adder.output(), (true, true));
    }

    #[test]
    fn test_n_bit_full_adder() {
        let n_bit_full_adder = NBitFullAdder::new([true, true, false], [true, false, false]);
        assert_eq!(n_bit_full_adder.output(), ([false, false, true], false));

        let n_bit_full_adder = NBitFullAdder::new([true, true, true], [false, true, true]);
        assert_eq!(n_bit_full_adder.output(), ([true, false, true], true));
    }
}
