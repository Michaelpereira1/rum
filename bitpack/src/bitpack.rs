use std::convert::TryInto;

/// performs signed right shift by "shift_amount" number of bits.
///
/// # Arguments:
/// * `left_value`: A signed integer value
/// * `shift_value`: the amount to shift left_value by
pub fn signed_right_shift(left_value: i64, shift_amount: u64) -> i64{
    if shift_amount >= 64 || shift_amount <= 0{
        panic!("0 <= Width <= 64");
    }else{
        left_value >> shift_amount
    }
}
/// performs unsigned right shift by "shift_amount" number of bits.
///
/// # Arguments:
/// * `left_value`: An unsigned integer value
/// * `shift_value`: the amount to shift left_value by
pub fn unsigned_right_shift(left_value: u64, shift_amount: u64) -> u64{
    if shift_amount >= 64 || shift_amount <= 0{
        panic!("0 <= Width <= 64");
    }else{
        left_value >> shift_amount
    }
}
/// Returns true iff the signed value `n` fits into `width` signed bits.
///
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(number: i64, width: u64) -> bool {
    let sign_bit;
    if number >= 0 {
        sign_bit = signed_right_shift(number, width-1);
    } else {
        sign_bit = signed_right_shift(!number, width-1);
    }
    return sign_bit == 0;
}
/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
///
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(number: u64, width: u64) -> bool {    
    return (unsigned_right_shift(number, width)) == 0;
}
/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
    if width <= 64 || (width + lsb) < 64 {
        let word_shift = word >> lsb;
        let msb = 1 << (width - 1);
        let value = (1 << width) - 1;
        let result:i64 = (value & word_shift).try_into().unwrap();
        let comparison:u64 = (result as u64) & msb; 
        if comparison > 0{
            return !((1 << width ) - result) + 1;
        }else{
            return result;
        }
        
    }else{
        panic!("width >= 64 or width <= 0 or (width + lsb) >= 64")
    }
}
/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
    if width <= 64 || (width + lsb) < 64 {
        let value = (1 << width) - 1;
        let word_shift = word >> (lsb);
        let result = value & word_shift;
        return result;
    }else{
        panic!("width >= 64 or width <= 0 or (width + lsb) >= 64")
    }
}
/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    if width <= 64 || (width + lsb) < 64 {
        if fitsu(value, width) == false {
            return None;
        }
        if lsb == 0 {
            let new_lsb = 1;
            let hi = (word >> (lsb + width)) << (lsb + width);
            let lo = (word << 64 - new_lsb) >> (64 - new_lsb);
            let result = hi | lo | (value << 0);
            return Some(result);
        }else{
            let hi = (word >> (lsb + width)) << (lsb + width);
            let lo = (word << 64 - lsb) >> (64 - lsb);
            let result = hi | lo | (value << lsb);
            return Some(result);

        }        
    }else{
        panic!("width >= 64 or width <= 0 or (width + lsb) >= 64");
    }
}
/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64>{  
    if width <= 64 || (width + lsb) < 64 {
        if fitss(value, width) == false {
            return None;
        }
        let hi = (word >> (lsb + width)) << (lsb + width);
        let lo = (word << 64 - lsb) >> (64 - lsb);       
        if value < 0 {
            let unsigned_value:u64 = value as u64;
            let converted_value = !unsigned_value + 1;
            let result = hi | lo | (converted_value << lsb);
            let real_result = (1 << width) - getu(result, width, lsb);
            return Some(real_result << lsb);
        }else{
            let unsigned_value:u64 = value as u64;
            let result = hi | lo | (unsigned_value << lsb);
            return Some(result);
        }
    }else{
        panic!("width >= 64 or width <= 0 or (width + lsb) >= 64");
    }    
}

#[cfg(test)]
mod tests {
    use crate::bitpack;
    #[test]
    fn fits_a() {
        let signed_fit = bitpack::fitss(-4, 3);
        let unsigned_fit = bitpack::fitsu(4, 3);
        let get_unsigned = bitpack::getu(0x3f4, 6, 2);
        let get_signed = bitpack::gets(151, 5, 2);
        let get_new = bitpack::newu(1024, 6, 2, 19).unwrap();
        let get_new_s = bitpack::news(32, 3, 2, -3).unwrap();
        let eq_check = bitpack::getu(bitpack::newu(128, 5, 2, 3).unwrap(), 5, 2);
        let signed_eq_check = bitpack::gets(bitpack::news(128, 5, 2, -4).unwrap(), 5, 2);
        assert_eq!(signed_fit, true);
        assert_eq!(unsigned_fit, true);
        assert_eq!(get_unsigned, 61);
        assert_eq!(get_signed, -5);
        assert_eq!(get_new, 1100);
        assert_eq!(get_new_s, 44);
        assert_eq!(eq_check, 3);
        assert_eq!(signed_eq_check, -4);
    }
    #[test]
    fn fits_b() {
        let signed_fit = bitpack::fitss(5, 3);
        let unsigned_fit = bitpack::fitsu(5, 3);
        let eq_check = bitpack::getu(bitpack::newu(128, 5, 2, 6).unwrap(), 3, 8);
        let signed_eq_check = bitpack::gets(bitpack::news(128, 4, 1, -4).unwrap(), 2, 5);
        assert_eq!(signed_fit, false);  
        assert_eq!(unsigned_fit, true);
        assert_eq!(eq_check, bitpack::getu(128, 3, 8));
        assert_eq!(signed_eq_check, bitpack::gets(128, 2, 5));
        assert_eq!(bitpack::news(0, 5, 13, -3).unwrap() , 29);
        assert_eq!(bitpack::news(0, 5, 8, -1).unwrap() , 31);
  
    }
    #[test]
    fn fits_c() {
        let signed_fit = bitpack::fitss(2, 3);
        let unsigned_fit = bitpack::fitsu(2, 3);
        assert_eq!(signed_fit, true);
        assert_eq!(unsigned_fit, true);

    }
    #[test]
    fn fits_d() {
        let signed_fit = bitpack::fitss(12, 8);
        let unsigned_fit = bitpack::fitsu(12, 8);
        assert_eq!(signed_fit, true);
        assert_eq!(unsigned_fit, true);

    }
    #[test]
    fn fits_e() {
        let signed_fit = bitpack::fitss(-15, 3);
        let unsigned_fit = bitpack::fitsu(15, 3);
        assert_eq!(signed_fit, false);
        assert_eq!(unsigned_fit, false);

    }
    #[test]
    fn fits_f() {
        let signed_fit = bitpack::fitss(4, 3);
        let unsigned_fit = bitpack::fitsu(4, 3);
        assert_eq!(signed_fit, false);
        assert_eq!(unsigned_fit, true);

    }
    #[test]
    fn fits_g() {
        let signed_fit = bitpack::fitss(-6, 3);
        let unsigned_fit = bitpack::fitsu(6, 3);
        assert_eq!(signed_fit, false);
        assert_eq!(unsigned_fit, true);

    }
    #[test]
    fn fits_h() {
        let signed_fit = bitpack::fitss(-6, 4);
        let unsigned_fit = bitpack::fitsu(6, 4);
        assert_eq!(signed_fit, true);
        assert_eq!(unsigned_fit, true);

    }
    #[test]
    fn fits_i() {
        let signed_fit = bitpack::fitss(4, 30);
        let unsigned_fit = bitpack::fitsu(4, 30);
        assert_eq!(signed_fit, true);
        assert_eq!(unsigned_fit, true);

    }
    #[test]
    fn fits_j() {
        let signed_fit = bitpack::fitss(1, 3);
        let unsigned_fit = bitpack::fitsu(1, 3);
        assert_eq!(signed_fit, true);
        assert_eq!(unsigned_fit, true);

    }
    #[test]
    fn fits_k() {
        let signed_fit = bitpack::fitss(400, 5);
        let unsigned_fit = bitpack::fitsu(400, 5);
        assert_eq!(signed_fit, false);
        assert_eq!(unsigned_fit, false);

    }
    #[test]
    fn fits_l() {
        let signed_fit = bitpack::fitss(40, 3);
        let unsigned_fit = bitpack::fitsu(40, 3);
        assert_eq!(signed_fit, false);
        assert_eq!(unsigned_fit, false);

    }
    #[test]
    fn fits_m() {
        let signed_fit = bitpack::fitss(-754, 3);
        let unsigned_fit = bitpack::fitsu(754, 3);
        assert_eq!(signed_fit, false);
        assert_eq!(unsigned_fit, false);

    }
    #[test]
    fn fits_n() {
        let signed_fit = bitpack::fitss(40, 64);
        let unsigned_fit = bitpack::fitsu(40, 23);
        assert_eq!(signed_fit, true);
        assert_eq!(unsigned_fit, true);

    }
    #[test]
    fn fits_o() {
        let signed_fit = bitpack::fitss(14, 32);
        let unsigned_fit = bitpack::fitsu(14, 32);
        assert_eq!(signed_fit, true);
        assert_eq!(unsigned_fit, true);
    }
    #[test]
    fn fits_p() {
        let signed_fit = bitpack::fitss(4, 16);
        let unsigned_fit = bitpack::fitsu(4, 16);
        assert_eq!(signed_fit, true);
        assert_eq!(unsigned_fit, true);
    }
}