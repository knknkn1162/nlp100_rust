//! This is the answer in http://www.cl.ecei.tohoku.ac.jp/nlp100/#ch1
//!

extern crate rand;

use std::collections::{HashMap, HashSet};
use std::fmt::Display;

use ch01::structure::{AnalysisType, CalcType};

use self::rand::{Rng, thread_rng};


/// ch1.00 reverse "stressed"
///
pub fn reverse(str: &str)-> String {
    str.chars().rev().collect()
}

/// ch1.01 extract chars of odd position from "パタトクカシーー"
///
pub fn extract<Pred>(str: &str, pred: Pred)-> String
    where Pred : Fn(usize) -> bool
{
    str.chars()
        .enumerate()
        .filter_map(|(idx, elem)| if pred(idx) {Some(elem)} else {None})
        .collect()
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


/// helper for ch1.03
/// convert Pi to vec<u32>
#[warn(dead_code)]
fn get_pi_digits(n: usize)->Vec<u32> {
    use std::f64;
    format!("{}", f64::consts::PI)
        .chars()
        .take(n)
        .filter_map(|s| s.to_digit(10))
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
pub fn generate_ngram(text: &str, size: usize, analysis_type: &AnalysisType)-> HashSet<String> {
    match *analysis_type {
        AnalysisType::Word => {
            let txt_lst: Vec<&str> = text.split_whitespace().collect();
            (0..txt_lst.len() - (size - 1))
                .map(|idx| txt_lst[idx..idx + size].join(" "))
                .collect()
        },
        AnalysisType::Character => {
            (0..text.len() - (size - 1))
                .map(|idx| (&text[idx..idx + size]).to_string())
                .collect()
        }
    }
}


/// ch01.06 Intersection, union, difference of two HashSets
///
pub fn calc_two_bigrams(text1: &str, text2: &str, calc_type: CalcType)-> HashSet<String> {
    let size = 2;
    let analysis_type = AnalysisType::Character;
    let bigram1: HashSet<String> = generate_ngram(text1, size, &analysis_type);
    let bigram2: HashSet<String> = generate_ngram(text2, size, &analysis_type);
    match calc_type {
        CalcType::InterSection => bigram1.intersection(&bigram2).map(|s| s.to_string()).collect(),
        CalcType::Union => bigram1.union(&bigram2).map(|s| s.to_string()).collect(),
        CalcType::Difference => bigram1.difference(&bigram2).map(|s| s.to_string()).collect(),
    }
}


/// ch01.07 generate description
pub fn generate_description<A: Display, B: Display, C: Display>(x: A, y: B, z: C)-> String {
    format!("{}時の{}は{}", x, y, z)
}

/// ch01.08 cipher text
pub fn generate_cipher(text: &str)-> String {
    text.chars().map(|s| if s.is_lowercase() {219 as char} else {s}).collect()
}

/// ch01.09 Typoglycemia. That means each character of word is randomize except first & last character.
///
pub fn generate_typoglycemia(text: &str)-> String {
    let mut rng = thread_rng();
    text.split_whitespace().map(
        |s| match s.len()  {
        1 ... 4 => s.to_string(),
            _ => {
                let mut iter = s.chars();
                let ch_first = iter.next().unwrap();
                let mut vec = iter.collect::<Vec<char>>();
                let ch_last = vec.pop().unwrap();
                rng.shuffle(&mut vec);
                format!("{}{}{}",
                        ch_first,
                        vec.into_iter().collect::<String>(),
                        ch_last)
            },
        }
    ).collect::<Vec<String>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_ch01_00_reverse() {
        assert_eq!("desserts", reverse("stressed"));
    }

    #[test]
    fn test_ch01_01_extract() {
        assert_eq!("パトカー", extract("パタトクカシーー", |idx| idx % 2 == 0));
    }

    #[test]
    fn test_ch01_02_join_alt() {
        assert_eq!("パタトクカシーー", join_alt("パトカー", "タクシー"));
    }

    #[test]
    fn test_helper_ch01_03_get_pi_digits() {
        let str_pi = get_pi_digits(16); // 16 significant figures
        assert_eq!(vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9], str_pi);
    }

    #[test]
    fn test_ch01_03_convert_piem() {
        assert_eq!(get_pi_digits(16), convert_piem());
    }

    #[test]
    fn test_ch01_04_generate_periodic_table() {
        let periodic_table = [
            ("H", 1), ("He", 2), ("Li", 3), ("Be", 4), ("B", 5), ("C", 6), ("N", 7), ("O", 8),
            ("F", 9), ("Ne", 10), ("Na", 11), ("Mi", 12), // In fact mnemonics "might" doesn't strictly same as "Mg"
            ("Al", 13), ("Si", 14), ("P", 15),
            ("S", 16), ("Cl", 17), ("Ar", 18), ("K", 19), ("Ca", 20)
        ].iter().cloned().collect::<HashMap<_, _>>();
        assert_eq!(periodic_table, generate_periodic_table());
    }

    #[test]
    fn test_ch01_05_generate_ngram() {
        let sentence = "I am an NLPer";
        assert_eq!(
            HashSet::from_iter(vec!["I am", "am an", "an NLPer"]
                .into_iter()
                .map(|s| s.to_string())
            ),
            generate_ngram(sentence, 2, &AnalysisType::Word)
        );
        assert_eq!(
            HashSet::from_iter(vec!["I ", " a", "am", "m ", " a", "an", "n ", " N", "NL", "LP", "Pe", "er"]
                    .into_iter()
                    .map(|s| s.to_string())
            ),
            generate_ngram(sentence, 2, &AnalysisType::Character)
        );
    }

    #[test]
    fn test_ch01_06_calc_two_diagrams() {
        // bigram of word1 is {"pa", "ar", "ra", "ap", "ad", "di", "is", "se"}
        // bigram of word2 is {"pa", "ar", "ag", "gr", "ap", "ph"]
        let (word1, word2) = ("paraparaparadise","paragraph");

        let union =
            calc_two_bigrams(word1, word2, CalcType::Union);

        assert_eq!(
            HashSet::from_iter(vec!["pa", "ad", "gr", "ph", "ap", "is", "se", "ar", "ra", "ag", "di"]
                    .into_iter()
                    .map(|s| s.to_string())
            ),
            union
        );
        assert!(union.contains("se"));

        assert_eq!(
            HashSet::from_iter(vec!["ar", "pa", "ra", "ap"]
                    .into_iter()
                    .map(|s| s.to_string())
            ),
            calc_two_bigrams(word1, word2, CalcType::InterSection)
        );

        assert_eq!(
            HashSet::from_iter(
                vec!["ad", "is", "di", "se"]
                    .into_iter()
                    .map(|s| s.to_string())
            ),
            calc_two_bigrams(word1, word2, CalcType::Difference)
        );
    }

    #[test]
    fn test_ch01_07_generate_description() {
        // ch01.Q07
        assert_eq!("12時の気温は22.4", generate_description(12, "気温", 22.4));
    }

    #[test]
    fn test_ch01_08_generate_cipher() {
        // ch01.Q08
        let sample = "12aBcdE8Qq";
        //let ch_219 = 'Û';
        assert_eq!("12ÛBÛÛE8QÛ", generate_cipher(sample));
    }

    #[test]
    fn test_ch01_09_generate_typoglycemia() {
        let sample_txt = "I couldn't believe that I could actually understand what I was reading :\
     the phenomenal power of the human mind.";

        println!("{}", generate_typoglycemia(sample_txt));
    }
}