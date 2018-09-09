use std::char::from_u32;
use std::io::prelude::*;
use std::io::{self, BufWriter};

mod trie;
use trie::Trie;

const DGC: &'static str = include_str!("../dat/DerivedGeneralCategory.txt");
const CATEGORY_NAMES: [&'static str; 30] = [
    "Lu", "Ll", "Lt", "Lm", "Lo",
    "Mn", "Mc", "Me",
    "Nd", "Nl", "No",
    "Pc", "Pd", "Ps", "Pe", "Pi", "Pf", "Po",
    "Sm", "Sc", "Sk", "So",
    "Zs", "Zl", "Zp",
    "Cc", "Cf", "Cs", "Co", "Cn"
];

fn split2<'a>(s: &'a str, sep: &str) -> Option<(&'a str, &'a str)> {
    let mut s = s.splitn(2, sep);
    s.next().and_then(|one| s.next().map(|two| (one, two)))
}

fn main() {
    let mut cats = Trie::new();
    for l in DGC.lines() {
        let s = l[..l.find('#').unwrap_or(l.len())].trim();
        if s.len() == 0 { continue; }
        
        let (range, value) = split2(s, ";").unwrap();
        let (range, value) = (range.trim(), value.trim());
        
        let value = CATEGORY_NAMES.iter().position(|&s| s == value).unwrap();
        let (start, end) = split2(range, "..").unwrap_or((range, range));
        
        let start = u32::from_str_radix(start, 16).unwrap();
        let end   = u32::from_str_radix(end,   16).unwrap();
        
        for c in start..end+1 {
            match from_u32(c) {
                Some(c) => cats.insert(c, value as u64),
                None => ()
            }
        }
    }
    
    let mut f = BufWriter::new(io::stdout());
    writeln!(f, "// \n// Generated code file\n// \n").expect("io error");
    cats.write_tables("CAT", "u16", &mut f);
}
