use crate::find_two;

#[test]
fn example1() {
    let input = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(find_two(&2020, &input), Some((1721, 299)));
}
#[test]
fn example2() {
    let input = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(find_two(&(2020 - 979), &input), Some((366, 675)));
}
