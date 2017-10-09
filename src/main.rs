use std::collections::HashMap;
mod ch01;
use ch01::AnalysisType; // use ch01.Q05

fn main() {
    //ch01.Q00
    assert_eq!("desserts", ch01::reverse("stressed"));

    //ch01.Q01
    assert_eq!("パトカー", ch01::extract("パタトクカシーー", |idx| idx%2==0));

    //ch01.Q02
    assert_eq!("パタトクカシーー", ch01::join_alt("パトカー", "タクシー"));

    //ch01.Q03
    let str_pi = get_pi_lst(16); // 16 significant figures
    assert_eq!(vec![3,1,4,1,5,9,2,6,5,3,5,8,9,7,9], str_pi);
    assert_eq!(str_pi, ch01::convert_piem());

    //ch01.Q04
    let periodic_table = get_periodic_table();
    assert_eq!(periodic_table, ch01::generate_periodic_table());

    //ch01.Q05
    let sentence = "I am an NLPer";
    assert_eq!(vec!["I am", "am an", "an NLPer"],
               ch01::generate_ngram(sentence, 2, AnalysisType::Word)
    );
    assert_eq!(vec!["I ", " a", "am", "m ", " a", "an","n ", " N", "NL", "LP", "Pe", "er"],
        ch01::generate_ngram(sentence, 2, AnalysisType::Character)
    )
}

/// helper for ch1.03
/// convert Pi to vec<u32>
fn get_pi_lst(n: usize)->Vec<u32> {
    format!("{}", std::f64::consts::PI)
        .chars()
        .take(n)
        .filter_map(|s| s.to_digit(10))
        .collect()
}

/// helper for ch1.04
/// get raw periodic table for test ch1.04
fn get_periodic_table<'a>()-> HashMap<&'a str, u32> {
    [
        ("H", 1), ("He", 2), ("Li", 3), ("Be", 4), ("B", 5), ("C", 6), ("N", 7), ("O", 8),
        ("F", 9), ("Ne", 10), ("Na", 11), ("Mi", 12), // In fact mnemonics "might" doesn't strictly same as "Mg"
        ("Al", 13), ("Si", 14), ("P", 15),
        ("S", 16), ("Cl", 17), ("Ar", 18), ("K", 19), ("Ca", 20)
    ].iter().cloned().collect()
}
