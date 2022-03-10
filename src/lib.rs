use std::fmt::{Display, Formatter};
use std::num::Wrapping;
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign,
    Shr, ShrAssign, Sub, SubAssign, Not,
};
use std::str;

const BIN_TABLE: &[u8; 256] = b"\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\xff\xff\xff\xff\xff\xff\xff\x0a\x0b\x0c\x0d\x0e\x0f\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\x0a\x0b\x0c\x0d\x0e\x0f\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff";
const HEX_TABLE: &[u8; 16] = b"0123456789abcdef";
const LEN8TAB: &[u8; 256] = b"\x00\x01\x02\x02\x03\x03\x03\x03\x04\x04\x04\x04\x04\x04\x04\x04\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08";
const BAD_NIBBLE: u8 = 0xff;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct Uint256([u128; 2]);

impl Uint256 {
    pub fn from_hex(hex: &str) -> Self {
        let mut n: Uint256 = Uint256([0, 0]);
        check_number(hex);
        if hex.len() > 66 {
            panic!("hex number > 256 bits");
        }
        let mut end = hex.len() as i32;
        for i in 0..2 {
            let mut start = end - 32;
            if start < 2 {
                start = 2;
            }
            for ri in start..end {
                let nib = BIN_TABLE[hex.as_bytes()[ri as usize] as usize];
                if nib == BAD_NIBBLE {
                    panic!("invalid hex string")
                }
                n.0[i] = n.0[i] << 4;
                n.0[i] += nib as u128;
            }
            end = start
        }
        n
    }

    pub fn hex(&self) -> String {
        let mut output: [char; 66] = ['\0'; 66];
        // nibbles indicates how many bytes are needed
        let mut nibbles = (self.bit_len() + 3) / 4;
        if nibbles == 0 {
            nibbles = 1;
        }
        let z_dword = (nibbles - 1) / 32;
        for i in (0..=z_dword).rev() {
            let off = (1 - i) * 32;
            for j in 0..32 {
                output[off + j + 2] =
                    HEX_TABLE[((self.0[i] >> (32 - j - 1) * 4) & 0xf) as usize] as char;
            }
        }
        output[64 - nibbles] = '0';
        output[65 - nibbles] = 'x';
        output[64 - nibbles..].iter().collect()
    }

    pub fn bit_len(&self) -> usize {
        if self.0[1] != 0 {
            bit_len128(self.0[1]) + 128
        } else {
            bit_len128(self.0[0])
        }
    }

    fn rsh128(&mut self) {
        self.0[0] = self.0[1];
        self.0[1] = 0;
    }

    fn lsh128(&mut self) {
        self.0[1] = self.0[0];
        self.0[0] = 0;
    }
}

impl Display for Uint256 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hex())
    }
}

impl Add for Uint256 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut n = Uint256::default();
        let carry;
        (n.0[0], carry) = add128(self.0[0], rhs.0[0], 0);
        (n.0[1], _) = add128(self.0[1], rhs.0[1], carry);
        n
    }
}

impl AddAssign for Uint256 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Uint256 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut n = Uint256::default();
        let carry;
        (n.0[0], carry) = sub128(self.0[0], rhs.0[0], 0);
        (n.0[1], _) = sub128(self.0[1], rhs.0[1], carry);
        n
    }
}

impl SubAssign for Uint256 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Shl<usize> for Uint256 {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut n = self;
        n <<= rhs;
        n
    }
}

impl Shr<usize> for Uint256 {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        let mut n = self;
        n >>= rhs;
        n
    }
}

impl ShlAssign<usize> for Uint256 {
    fn shl_assign(&mut self, mut rhs: usize) {
        if rhs == 128 {
            self.lsh128();
            return;
        }
        if rhs > 128 {
            if rhs > 256 {
                self.0[0] = 0;
                self.0[1] = 0;
                return;
            }
            self.lsh128();
            rhs -= 128;
            self.0[1] <<= rhs;
            return;
        }
        if rhs == 0 {
            return;
        }
        let r = self.0[0] >> (128 - rhs);
        self.0[0] = self.0[0] << rhs;
        self.0[1] = (self.0[1] << rhs) | r;
    }
}

impl ShrAssign<usize> for Uint256 {
    fn shr_assign(&mut self, mut rhs: usize) {
        if rhs == 128 {
            self.rsh128();
            return;
        }
        if rhs > 128 {
            if rhs > 256 {
                self.0[0] = 0;
                self.0[1] = 0;
                return;
            }
            self.rsh128();
            rhs -= 128;
            self.0[0] >>= rhs;
            return;
        }
        if rhs == 0 {
            return;
        }
        let r = self.0[1] << (128 - rhs);
        self.0[1] = self.0[1] >> rhs;
        self.0[0] = (self.0[0] >> rhs) | r;
    }
}

impl BitAnd for Uint256 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut n = Uint256::default();
        n.0[0] = self.0[0] & rhs.0[0];
        n.0[1] = self.0[1] & rhs.0[1];
        n
    }
}

impl BitAndAssign for Uint256 {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for Uint256 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut n = Uint256::default();
        n.0[0] = self.0[0] | rhs.0[0];
        n.0[1] = self.0[1] | rhs.0[1];
        n
    }
}

impl BitOrAssign for Uint256 {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for Uint256 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut n = Uint256::default();
        n.0[0] = self.0[0] ^ rhs.0[0];
        n.0[1] = self.0[1] ^ rhs.0[1];
        n
    }
}

impl BitXorAssign for Uint256 {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl Not for Uint256 {
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut n = Uint256::default();
        n.0[0] = !self.0[0];
        n.0[1] = !self.0[1];
        n
    }
}

impl From<u128> for Uint256 {
    fn from(u: u128) -> Self {
        Uint256([u, 0])
    }
}

impl Default for Uint256 {
    fn default() -> Self {
        Uint256([0, 0])
    }
}

fn check_number(input: &str) {
    let input = input.as_bytes();
    let l = input.len();
    if l == 0 {
        panic!("empty hex string");
    }
    if l < 2 || input[0] != '0' as u8 || (input[1] != 'x' as u8 && input[1] != 'X' as u8) {
        panic!("invalid hex string");
    }
    if l == 2 {
        panic!("hex string \"0x\"");
    }
}

fn bit_len128(mut x: u128) -> usize {
    let mut n: usize = 0;
    if x >= 1 << 64 {
        x >>= 64;
        n = 64;
    }
    if x >= 1 << 32 {
        x >>= 32;
        n += 32;
    }
    if x >= 1 << 16 {
        x >>= 16;
        n += 16;
    }
    if x >= 1 << 8 {
        x >>= 8;
        n += 8;
    }
    n + (LEN8TAB[x as usize] as usize)
}

fn add128(x: u128, y: u128, carry: u128) -> (u128, u128) {
    let sum = Wrapping(x) + Wrapping(y) + Wrapping(carry);
    let carry_out = ((x & y) | ((x | y) & !sum.0)) >> 127;
    (sum.0, carry_out)
}

fn sub128(x: u128, y: u128, borrow: u128) -> (u128, u128) {
    let diff = Wrapping(x) - Wrapping(y) - Wrapping(borrow);
    let borrow_out = ((!x & y) | (!(x ^ y) & diff.0)) >> 127;
    (diff.0, borrow_out)
}

#[cfg(test)]
mod tests;
