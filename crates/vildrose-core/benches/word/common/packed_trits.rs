use vildrose_core::trit::Trit;

pub const fn trit_to_digit(trit: Trit) -> u8 {
    match trit {
        Trit::N => 0,
        Trit::Z => 1,
        Trit::P => 2,
    }
}

pub fn bits_to_trit(bits: u8) -> Trit {
    match bits {
        0b00 => Trit::N,
        0b01 => Trit::Z,
        0b10 => Trit::P,
        _ => unreachable!("2-bit trit must be 00, 01, or 10"),
    }
}

pub fn digit_to_trit(digit: u8) -> Trit {
    match digit {
        0 => Trit::N,
        1 => Trit::Z,
        2 => Trit::P,
        _ => unreachable!("base-3 digit must be 0, 1, or 2"),
    }
}

pub fn encode_1(trits: &[Trit]) -> Vec<u8> {
    trits.iter().copied().map(trit_to_digit).collect()
}

pub fn encode_4(trits: &[Trit]) -> Vec<u8> {
    let mut out = Vec::with_capacity(trits.len().div_ceil(4));

    let mut i = 0;
    let len = trits.len();

    while i + 4 <= len {
        let d0 = trit_to_digit(trits[i]);
        let d1 = trit_to_digit(trits[i + 1]);
        let d2 = trit_to_digit(trits[i + 2]);
        let d3 = trit_to_digit(trits[i + 3]);

        out.push(d0 | (d1 << 2) | (d2 << 4) | (d3 << 6));
        i += 4;
    }

    if i < len {
        let mut byte = 0u8;
        let mut shift = 0u8;

        while i < len {
            byte |= trit_to_digit(trits[i]) << shift;
            shift += 2;
            i += 1;
        }

        out.push(byte);
    }

    out
}

pub fn encode_5(trits: &[Trit]) -> Vec<u8> {
    let mut out = Vec::with_capacity(trits.len().div_ceil(5));

    let mut i = 0;
    let len = trits.len();

    while i + 5 <= len {
        let d0 = u16::from(trit_to_digit(trits[i]));
        let d1 = u16::from(trit_to_digit(trits[i + 1]));
        let d2 = u16::from(trit_to_digit(trits[i + 2]));
        let d3 = u16::from(trit_to_digit(trits[i + 3]));
        let d4 = u16::from(trit_to_digit(trits[i + 4]));

        let value = d0 + d1 * 3 + d2 * 9 + d3 * 27 + d4 * 81;
        out.push(u8::try_from(value).unwrap());
        i += 5;
    }

    if i < len {
        let mut value = 0u16;
        let mut place = 1u16;

        while i < len {
            value += (u16::from(trit_to_digit(trits[i]))) * place;
            place *= 3;
            i += 1;
        }

        out.push(u8::try_from(value).unwrap());
    }

    out
}

pub fn decode_1_direct(bytes: &[u8], trit_count: usize) -> Vec<Trit> {
    assert!(
        bytes.len() >= trit_count,
        "not enough bytes for requested trit count",
    );

    bytes[..trit_count]
        .iter()
        .copied()
        .map(digit_to_trit)
        .collect()
}

pub fn decode_4_direct(bytes: &[u8], trit_count: usize) -> Vec<Trit> {
    let mut result = Vec::with_capacity(trit_count);

    for &byte in bytes {
        let b0 = (byte) & 0b11;
        debug_assert!(b0 != 0b11, "invalid 2-bit trit in lane 0");
        result.push(bits_to_trit(b0));
        if result.len() == trit_count {
            return result;
        }

        let b1 = (byte >> 2) & 0b11;
        debug_assert!(b1 != 0b11, "invalid 2-bit trit in lane 1");
        result.push(bits_to_trit(b1));
        if result.len() == trit_count {
            return result;
        }

        let b2 = (byte >> 4) & 0b11;
        debug_assert!(b2 != 0b11, "invalid 2-bit trit in lane 2");
        result.push(bits_to_trit(b2));
        if result.len() == trit_count {
            return result;
        }

        let b3 = (byte >> 6) & 0b11;
        debug_assert!(b3 != 0b11, "invalid 2-bit trit in lane 3");
        result.push(bits_to_trit(b3));
        if result.len() == trit_count {
            return result;
        }
    }

    result
}

pub fn decode_5_div(bytes: &[u8], trit_count: usize) -> Vec<Trit> {
    let mut result = Vec::with_capacity(trit_count);

    for &byte in bytes {
        debug_assert!(byte < 243, "invalid 5-trit packed byte: {byte}");

        let mut value = byte;

        let d0 = value % 3;
        result.push(digit_to_trit(d0));
        if result.len() == trit_count {
            return result;
        }
        value /= 3;

        let d1 = value % 3;
        result.push(digit_to_trit(d1));
        if result.len() == trit_count {
            return result;
        }
        value /= 3;

        let d2 = value % 3;
        result.push(digit_to_trit(d2));
        if result.len() == trit_count {
            return result;
        }
        value /= 3;

        let d3 = value % 3;
        result.push(digit_to_trit(d3));
        if result.len() == trit_count {
            return result;
        }
        value /= 3;

        let d4 = value % 3;
        result.push(digit_to_trit(d4));
        if result.len() == trit_count {
            return result;
        }
    }

    result
}

pub fn decode_5_table(bytes: &[u8], trit_count: usize) -> Vec<Trit> {
    decode_with_table_5(bytes, trit_count, &DECODE_5_TABLE)
}

pub const INVALID: u16 = u16::MAX;

pub const fn make_decode_table_5() -> [u16; 256] {
    let mut table = [INVALID; 256];
    let mut byte = 0u16;

    while byte < 243 {
        let mut remaining = byte;
        let mut packed_trits = 0u16;
        let mut trit_index = 0u16;

        while trit_index < 5 {
            let digit = remaining % 3;
            packed_trits |= digit << (trit_index * 2);
            remaining /= 3;
            trit_index += 1;
        }

        table[byte as usize] = packed_trits;
        byte += 1;
    }

    table
}

pub const DECODE_5_TABLE: [u16; 256] = make_decode_table_5();

pub fn decode_with_table_5(bytes: &[u8], trit_count: usize, table: &[u16; 256]) -> Vec<Trit> {
    let mut result = Vec::with_capacity(trit_count);

    for &byte in bytes {
        let packed = table[usize::from(byte)];
        assert_ne!(packed, INVALID, "invalid packed ternary byte: {byte}");

        let b0 = ((packed) & 0b11) as u8;
        result.push(bits_to_trit(b0));
        if result.len() == trit_count {
            return result;
        }

        let b1 = ((packed >> 2) & 0b11) as u8;
        result.push(bits_to_trit(b1));
        if result.len() == trit_count {
            return result;
        }

        let b2 = ((packed >> 4) & 0b11) as u8;
        result.push(bits_to_trit(b2));
        if result.len() == trit_count {
            return result;
        }

        let b3 = ((packed >> 6) & 0b11) as u8;
        result.push(bits_to_trit(b3));
        if result.len() == trit_count {
            return result;
        }

        let b4 = ((packed >> 8) & 0b11) as u8;
        result.push(bits_to_trit(b4));
        if result.len() == trit_count {
            return result;
        }
    }

    result
}
