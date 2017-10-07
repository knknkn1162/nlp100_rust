mod ch01;

fn main() {
    assert_eq!("desserts", ch01::reverse("stressed"));
    assert_eq!("パトカー", ch01::extract("パタトクカシーー", |idx| idx%2==0));
    assert_eq!("パタトクカシーー", ch01::join_alt("パトカー", "タクシー"));
    assert_eq!(vec![3,1,4,1,6,9,2,7,5,3,5,8,9,7,0], ch01::convert_piem());
}

