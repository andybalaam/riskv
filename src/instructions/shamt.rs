use std::ops::Deref;

pub(super) struct Shamt(u8);

impl Shamt {
    const MASK: u32 = u32::from_le(0b_0000001_11111_00000_000_00000_0000000);
    const RSHIFT: usize = 20;
}

impl From<u32> for Shamt {
    fn from(value: u32) -> Self {
        Self(((value & Self::MASK) >> Self::RSHIFT) as u8)
    }
}

impl Deref for Shamt {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn from_u32() {
        let instruction = u32::from_le(0b_0100100_00110_01100_101_01000_0010011);
        assert_eq!(*Shamt::from(instruction), 6);
        // For 64 bit architectures, bit 25 is intepreted as part of the shift amount
        let instruction = u32::from_le(0b_0100101_00110_01100_101_01000_0010011);
        assert_eq!(*Shamt::from(instruction), 38);
    }
}
