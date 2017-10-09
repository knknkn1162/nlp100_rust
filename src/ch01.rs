use std::collections::{HashMap, HashSet};

/// ch1.00 reverse "stressed"
///
/// ```
/// chs = str.chars(): Chars // An iterator over the chars of a string slice.
/// rev = chs.rev(): std::iter::Rev // Reverses an iterator's direction.
/// ans = rev.collect::<String>() // Transforms an iterator into a collection.
/// ```
///
pub fn reverse(str: &str)-> String {
    str.chars().rev().collect::<String>()
}

/// ch1.01 extract chars of odd position from "パタトクカシーー"
///
pub fn extract<Pred>(str: &str, pred: Pred)-> String
    where Pred : Fn(usize) -> bool
{
    str.chars()
        .enumerate()
        .filter(|&(idx, _)| pred(idx))
        .map(|(_, elem)| elem)
        .collect::<String>()
}

/// ch1.02 concat two words alternatively. (e.g. "abc", "def" => "adbecf")
///
pub fn join_alt(str1: &str, str2: &str)-> String {
    str1.chars()
        .zip(str2.chars())
        .map(|(ch1, ch2)| format!("{}{}", ch1,ch2))
        .collect::<String>()
}

/// ch1.03 convert "piem" to Pi.
/// Note) "piem" is the sentence,
/// "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics."
///  The sequence of its word-length means the ratio of the circumference of a circle to the diameter.
///
pub fn convert_piem() -> Vec<u32> {
    let piem = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
    piem.split_whitespace()
        .map(|s| (s.trim_matches(|c| c == ',' || c == '.').len()%10) as u32)
        .collect()
}

/// ch1.04 convert mnemoric to periodic table which type is HashMap.
///
pub fn generate_periodic_table<'a>()-> HashMap<&'a str, u32> {
    let mnemoric = "Hi He Lied Because Boron Could Not Oxidize Fluorine. \
        New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let word_length = mnemoric.split_whitespace().count();
    let indexes: Vec<usize> = [1, 5, 6, 7, 8, 9, 15, 16, 19].iter().map(|s| ((s-1) as usize)).collect();
    let take_mapping: HashMap<usize, usize> =
        (0..word_length)
            .map(
            |s|
                if indexes.contains(&s) {(s, 1)} else {(s, 2)})
            .collect();
    mnemoric.split_whitespace()
        .enumerate()
        .map(|(idx, elem)|
            (&elem[0..take_mapping[&idx]], ((idx+1) as u32) )
        )
        .collect()

}

/// ch1.05 n-gram
///
pub fn generate_ngram(text: &str, size: usize, analysis_type: AnalysisType)-> HashSet<String> {
    match analysis_type {
        AnalysisType::Word => {
            let txt_lst: Vec<&str> = text.split_whitespace().collect();
            (0..txt_lst.len()-(size-1))
                .map(|idx| txt_lst[idx..idx+size].join(" "))
                .collect()
        },
        AnalysisType::Character => {
            (0..text.len()-(size-1))
                .map(|idx| (&text[idx..idx+size]).to_string())
                .collect()
        }
    }

}


pub enum AnalysisType {
    Word,
    Character,

}