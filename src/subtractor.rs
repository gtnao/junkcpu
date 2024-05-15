use crate::{adder::FullAdder, gate::NotGate};

pub struct NBitFullSubtractor<const N: usize> {
    a: [bool; N],
    b: [bool; N],
}
impl<const N: usize> NBitFullSubtractor<N> {
    pub fn new(a: [bool; N], b: [bool; N]) -> Self {
        Self { a, b }
    }
    pub fn output(&self) -> ([bool; N], bool) {
        let mut carry = true;
        let mut result = [false; N];
        (0..N).for_each(|i| {
            let not = NotGate::new(self.b[i]);
            let full_adder = FullAdder::new(self.a[i], not.output(), carry);
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
    fn test_n_bit_full_subtractor() {
        let n_bit_full_subtractor =
            NBitFullSubtractor::new([true, false, true], [true, true, false]);
        assert_eq!(n_bit_full_subtractor.output(), ([false, true, false], true));
    }
}
