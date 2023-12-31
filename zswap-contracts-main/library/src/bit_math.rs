use std::str::FromStr;

use crate::num256::U256;

pub trait BitMathTrait {
    fn most_significant_bit(&self) -> u8;
    fn least_significant_bit(&self) -> u8;
}

impl BitMathTrait for U256 {
    /// @notice Returns the index of the most significant bit of the number,
    ///     where the least significant bit is at index 0 and the most significant bit is at index 255
    /// @dev The function satisfies the property:
    ///     x >= 2**most_significant_bit(x) and x < 2**(most_significant_bit(x)+1)
    /// @param x the value for which to compute the most significant bit, must be greater than 0
    /// @return r the index of the most significant bit
    fn most_significant_bit(&self) -> u8 {
        let mut x = *self;
        assert!(x > U256::zero(), "Value must be greater than 0");

        let mut r: u8 = 0;
        if x >= U256::from_str("0x100000000000000000000000000000000").unwrap() {
            x >>= 128;
            r += 128;
        }
        if x >= U256::from_str("0x10000000000000000").unwrap() {
            x >>= 64;
            r += 64;
        }
        if x >= U256::from_str("0x100000000").unwrap() {
            x >>= 32;
            r += 32;
        }
        if x >= U256::from_str("0x10000").unwrap() {
            x >>= 16;
            r += 16;
        }
        if x >= U256::from_str("0x100").unwrap() {
            x >>= 8;
            r += 8;
        }
        if x >= U256::from_str("0x10").unwrap() {
            x >>= 4;
            r += 4;
        }
        if x >= U256::from_str("0x4").unwrap() {
            x >>= 2;
            r += 2;
        }
        if x >= U256::from_str("0x2").unwrap() {
            r += 1;
        }
        r
    }

    /// @notice Returns the index of the least significant bit of the number,
    ///     where the least significant bit is at index 0 and the most significant bit is at index 255
    /// @dev The function satisfies the property:
    ///     (x & 2**least_significant_bit(x)) != 0 and (x & (2**(least_significant_bit(x)) - 1)) == 0)
    /// @param x the value for which to compute the least significant bit, must be greater than 0
    /// @return r the index of the least significant bit
    fn least_significant_bit(&self) -> u8 {
        let mut x: U256 = *self;
        assert!(x > U256::zero(), "Value must be greater than 0");

        let mut r: u8 = 255;
        if (x & (U256::MAX >> 128)) > U256::zero() {
            r -= 128;
        } else {
            x >>= 128;
        }
        if x & U256::MAX >> 64 > U256::zero() {
            r -= 64;
        } else {
            x >>= 64;
        }
        if x & U256::MAX >> 32 > U256::zero() {
            r -= 32;
        } else {
            x >>= 32;
        }
        if x & U256::MAX >> 16 > U256::zero() {
            r -= 16;
        } else {
            x >>= 16;
        }
        if x & U256::MAX >> 8 > U256::zero() {
            r -= 8;
        } else {
            x >>= 8;
        }
        if x & U256::from(0xf) > U256::zero() {
            r -= 4;
        } else {
            x >>= 4;
        }
        if x & U256::from(0x3) > U256::zero() {
            r -= 2;
        } else {
            x >>= 2;
        }
        if x & U256::from(0x1) > U256::zero() {
            r -= 1;
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_math() {
        let values = [
            U256::from(1234567890u128),
            U256::from_dec_str(
                "57896044618658097711785492504343953926634992332820282019728792004938939802342",
            )
            .unwrap(),
            U256::from_dec_str("1606938044259977626524336601268507632356953233627996783863721")
                .unwrap(),
            U256::from_dec_str("1037042214541001286141").unwrap(),
            !U256::zero(),
            U256::from_dec_str(
                "97896044618658097711785492504343953926634992332820282019728792004938939802342",
            )
            .unwrap(),
        ];

        let n = values.len();
        for i in 0..n {
            let x = values[i];
            let mut lsb = 0;
            let mut msb = 0;
            for j in 0..=255 {
                if ((x >> j) & U256::one()) == U256::one() {
                    lsb = j;
                    break;
                }
            }
            for j in 0..=255 {
                if ((x >> j) & U256::one()) == U256::one() {
                    msb = j;
                }
            }
            println!("{} {} {}", x, lsb, msb);
            assert_eq!(lsb, x.least_significant_bit());
            assert_eq!(msb, x.most_significant_bit());
        }
    }
}
