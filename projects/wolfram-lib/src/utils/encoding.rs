use crate::{utils::SYSTEM_SYMBOLS, WolframValue};
use std::{collections::BTreeSet, mem::size_of, ops::Deref};

impl WolframValue {
    pub fn to_string(&self) -> String {
        match self {
            WolframValue::Function => unimplemented!(),
            WolframValue::String(s) => format!("{:?}", s),
            WolframValue::Bytes => unimplemented!(),
            WolframValue::Symbol(s) => format!("{}", s),
            WolframValue::Integer8(_) => unimplemented!(),
            WolframValue::Integer16(_) => unimplemented!(),
            WolframValue::Integer32(_) => unimplemented!(),
            WolframValue::Integer64(_) => unimplemented!(),
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
            WolframValue::Integer8(_) => unimplemented!(),
            WolframValue::Integer16(_) => unimplemented!(),
            WolframValue::Integer32(_) => unimplemented!(),
            WolframValue::Integer64(_) => unimplemented!(),
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

#[test]
fn test() {
    let o = WolframValue::Symbol(Box::from("None"));
    let v = o.to_bytes();
    println!("{:?}", v)
}