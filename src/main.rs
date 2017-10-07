mod ch01;

fn main() {
    assert_eq!("desserts", ch01::reverse("stressed"));
    assert_eq!("パトカー", ch01::extract("パタトクカシーー", |idx| idx%2==0));
    assert_eq!("パタトクカシーー", ch01::join_alt("パトカー", "タクシー"))
}

