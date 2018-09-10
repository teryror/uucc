use std::char::from_u32;
use std::collections::HashSet;
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

const SCR: &'static str = include_str!("../dat/Scripts.txt");
const SCRIPT_NAMES: [&'static str; 149] = [
    "Unknown", "Adlam", "Ahom", "Anatolian_Hieroglyphs", "Arabic", "Armenian", "Avestan", 
    "Balinese", "Bamum", "Bassa_Vah", "Batak", "Bengali", "Bhaiksuki", "Bopomofo", 
    "Brahmi", "Braille", "Buginese", "Buhid", "Canadian_Aboriginal", "Carian", "Caucasian_Albanian", 
    "Chakma", "Cham", "Cherokee", "Common", "Coptic", "Cuneiform", "Cypriot", "Cyrillic", 
    "Deseret", "Devanagari", "Dogra", "Duployan", "Egyptian_Hieroglyphs", "Elbasan", 
    "Ethiopic", "Georgian", "Glagolitic", "Gothic", "Grantha", "Greek", "Gujarati", 
    "Gunjala_Gondi", "Gurmukhi", "Han", "Hangul", "Hanifi_Rohingya", "Hanunoo", "Hatran", 
    "Hebrew", "Hiragana", "Imperial_Aramaic", "Inherited", "Inscriptional_Pahlavi", 
    "Inscriptional_Parthian", "Javanese", "Kaithi", "Kannada", "Katakana", "Kayah_Li", 
    "Kharoshthi", "Khmer", "Khojki", "Khudawadi", "Lao", "Latin", "Lepcha", "Limbu", 
    "Linear_A", "Linear_B", "Lisu", "Lycian", "Lydian", "Mahajani", "Makasar", "Malayalam", 
    "Mandaic", "Manichaean", "Marchen", "Masaram_Gondi", "Medefaidrin", "Meetei_Mayek", 
    "Mende_Kikakui", "Meroitic_Cursive", "Meroitic_Hieroglyphs", "Miao", "Modi", 
    "Mongolian", "Mro", "Multani", "Myanmar", "Nabataean", "New_Tai_Lue", "Newa", 
    "Nko", "Nushu", "Ogham", "Ol_Chiki", "Old_Hungarian", "Old_Italic", "Old_North_Arabian", 
    "Old_Permic", "Old_Persian", "Old_Sogdian", "Old_South_Arabian", "Old_Turkic", 
    "Oriya", "Osage", "Osmanya", "Pahawh_Hmong", "Palmyrene", "Pau_Cin_Hau", "Phags_Pa", 
    "Phoenician", "Psalter_Pahlavi", "Rejang", "Runic", "Samaritan", "Saurashtra", 
    "Sharada", "Shavian", "Siddham", "SignWriting", "Sinhala", "Sogdian", "Sora_Sompeng", 
    "Soyombo", "Sundanese", "Syloti_Nagri", "Syriac", "Tagalog", "Tagbanwa", "Tai_Le", 
    "Tai_Tham", "Tai_Viet", "Takri", "Tamil", "Tangut", "Telugu", "Thaana", "Thai", 
    "Tibetan", "Tifinagh", "Tirhuta", "Ugaritic", "Vai", "Warang_Citi", "Yi", "Zanabazar_Square", 
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
            if let Some(c) = from_u32(c) {
                cats.insert(c, value as u64);
            }
        }
    }
    
    let mut scripts = Trie::new();
    for l in SCR.lines() {
        let s = l[..l.find('#').unwrap_or(l.len())].trim();
        if s.len() == 0 { continue; }
        
        let (range, value) = split2(s, ";").unwrap();
        let (range, value) = (range.trim(), value.trim());
        
        let value = SCRIPT_NAMES.iter().position(|&s| s == value).unwrap();
        let (start, end) = split2(range, "..").unwrap_or((range, range));
        
        let start = u32::from_str_radix(start, 16).unwrap();
        let end   = u32::from_str_radix(end,   16).unwrap();
        
        for c in start..end+1 {
            if let Some(c) = from_u32(c) {
                scripts.insert(c, value as u64);
            }
        }
    }
    
    let mut f = BufWriter::new(io::stdout());
    writeln!(f, "// \n// Generated code file\n// \n").expect("io error");
    cats.write_tables("CAT", "u16", &mut f);
    scripts.write_tables("SCRIPT", "u16", &mut f);
}
