mod ch01;

fn main() {
    assert_eq!("desserts", ch01::reverse("stressed"));
    assert_eq!("パトカー", ch01::extract("パタトクカシーー", |idx| idx%2==0));
    assert_eq!("パタトクカシーー", ch01::join_alt("パトカー", "タクシー"));
    let str_pi = get_pi_lst(16); // 16 significant figures
    assert_eq!(vec![3,1,4,1,5,9,2,6,5,3,5,8,9,7,9], str_pi);
    assert_eq!(str_pi, ch01::convert_piem());
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

