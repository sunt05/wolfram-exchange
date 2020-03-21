use crate::{utils::SYSTEM_SYMBOLS, ToWolfram, WolframValue};
use std::{
    collections::BTreeSet,
    mem::{size_of, transmute},
    ops::Deref,
};

impl WolframValue {
    pub fn to_string(&self) -> String {
        match self {
            WolframValue::Function => unimplemented!(),
            WolframValue::String(s) => format!("{:?}", s),
            WolframValue::Bytes => unimplemented!(),
            WolframValue::Symbol(s) => format!("{}", s),
            WolframValue::Integer8(n) => format!("{}", n),
            WolframValue::Integer16(n) => format!("{}", n),
            WolframValue::Integer32(n) => format!("{}", n),
            WolframValue::Integer64(n) => format!("{}", n),
            WolframValue::Decimal64(_) => unimplemented!(),
            WolframValue::BigInteger(_) => unimplemented!(),
            WolframValue::BigDecimal(_) => unimplemented!(),
            WolframValue::PackedArray => unimplemented!(),
            WolframValue::NumericArray(_) => unimplemented!(),
            WolframValue::Association => unimplemented!(),
            WolframValue::Rule => unimplemented!(),
            WolframValue::RuleDelayed => unimplemented!(),
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.push(b'8');
        out.push(b':');
        for c in self.to_bytes_inner() {
            out.push(c)
        }
        return out;
    }
    pub fn to_compressed(&self) -> Vec<u8> {
        unimplemented!()
    }

    fn to_bytes_inner(&self) -> Vec<u8> {
        match self {
            WolframValue::Function => unimplemented!(),
            WolframValue::String(s) => {
                let mut out = Vec::with_capacity(2 * s.len());
                out.push(b'S');
                for c in length_encoding(&s) {
                    out.push(c)
                }
                for c in s.as_bytes() {
                    out.push(*c)
                }
                return out;
            }
            WolframValue::Bytes => unimplemented!(),
            WolframValue::Symbol(s) => {
                let symbol = standardized_symbol_name(s);
                let mut out = Vec::with_capacity(2 * s.len());
                out.push(b's');
                for c in length_encoding(&symbol) {
                    out.push(c)
                }
                for c in symbol.as_bytes() {
                    out.push(*c)
                }
                return out;
            }
            WolframValue::Integer8(n) => {
                let mut v = Vec::with_capacity(2);
                v.push(b'C');
                let le: [u8; 1] = unsafe { transmute(n.to_le()) };
                for c in le.iter() {
                    v.push(*c)
                }
                return v;
            }
            WolframValue::Integer16(n) => {
                let mut v = Vec::with_capacity(3);
                v.push(b'j');
                let le: [u8; 2] = unsafe { transmute(n.to_le()) };
                for c in le.iter() {
                    v.push(*c)
                }
                return v;
            }
            WolframValue::Integer32(n) => {
                let mut v = Vec::with_capacity(5);
                v.push(b'i');
                let le: [u8; 4] = unsafe { transmute(n.to_le()) };
                for c in le.iter() {
                    v.push(*c)
                }
                return v;
            }
            WolframValue::Integer64(n) => {
                let mut v = Vec::with_capacity(9);
                v.push(b'L');
                let le: [u8; 8] = unsafe { transmute(n.to_le()) };
                for c in le.iter() {
                    v.push(*c)
                }
                return v;
            }
            WolframValue::Decimal64(_) => unimplemented!(),
            WolframValue::BigInteger(n) => {
                let mut v = Vec::with_capacity(2 * n.len());
                v.push(b'I');
                for c in length_encoding(n) {
                    v.push(c)
                }
                for c in n.as_bytes() {
                    v.push(*c)
                }
                return v;
            }
            WolframValue::BigDecimal(_) => unimplemented!(),
            WolframValue::PackedArray => unimplemented!(),
            WolframValue::NumericArray(_) => unimplemented!(),
            WolframValue::Association => unimplemented!(),
            WolframValue::Rule => unimplemented!(),
            WolframValue::RuleDelayed => unimplemented!(),
        }
    }
}

fn standardized_symbol_name(input: &str) -> String {
    if input.contains('`') {
        return format!("{}", input);
    }
    let mut set = BTreeSet::new();
    for sys in SYSTEM_SYMBOLS.iter() {
        set.insert(*sys);
    }
    if set.contains(input) { format!("{}", input) } else { format!("Global`{}", input) }
}

///
/// ```wl
/// bits = IntegerDigits[9999, 2]
/// grouped7 = Partition[Reverse[bits], UpTo[7]]
/// grouped8 = Map[Composition[PadLeft[#, 8] &, Reverse], grouped7]
/// varint = ReplacePart[grouped8, {i_, 1} /; i < Length[grouped8] :> 1]
/// Map[FromDigits[#, 2] &, varint]
///```
fn length_encoding(input: &str) -> Vec<u8> {
    let len = input.len();
    if len <= 127 {
        return [len as u8].to_vec();
    }
    else {
        unimplemented!()
    }
}
