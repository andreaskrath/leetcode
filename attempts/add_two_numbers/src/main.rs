fn main() {
    let one_inner = Box::new(ListNode::new(3));
    let mut one_mid = Box::new(ListNode::new(4));
    one_mid.next = Some(one_inner);
    let mut one_outer = Box::new(ListNode::new(2));
    one_outer.next = Some(one_mid);

    let two_inner = Box::new(ListNode::new(4));
    let mut two_mid = Box::new(ListNode::new(6));
    two_mid.next = Some(two_inner);
    let mut two_outer = Box::new(ListNode::new(5));
    two_outer.next = Some(two_mid);

    let expected_inner = Box::new(ListNode::new(8));
    let mut expected_mid = Box::new(ListNode::new(0));
    expected_mid.next = Some(expected_inner);
    let mut expected_outer = Box::new(ListNode::new(7));
    expected_outer.next = Some(expected_mid);

    let actual = add_two_numbers(Some(one_outer), Some(two_outer));
    assert_eq!(actual, Some(expected_outer));
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let l1_str: String = get_number_from_list(l1).chars().rev().collect();
    let l2_str: String = get_number_from_list(l2).chars().rev().collect();

    let mut new_str = String::new();
    let mut l1_iter = l1_str.chars();
    let mut l2_iter = l2_str.chars();
    let mut has_carry = false;

    loop {
        let l1_val = l1_iter.next();
        let l2_val = l2_iter.next();
        match (l1_val, l2_val) {
            (None, None) => {
                if has_carry {
                    new_str.push('1');
                }
                break;
            }
            (None, Some(n)) | (Some(n), None) => {
                let num = n as u8 - 48;
                let sum = match has_carry {
                    true => num + 1,
                    false => num,
                };
                if sum > 9 {
                    new_str.push((sum - 10) as char);
                    has_carry = true;
                } else {
                    new_str.push(sum as char);
                    has_carry = false;
                }
            }
            (Some(a), Some(b)) => {
                let num_a = a as u8 - 48;
                let num_b = b as u8 - 48;
                let sum = match has_carry {
                    true => num_a + num_b + 1,
                    false => num_a + num_b,
                };
                if sum > 9 {
                    new_str.push((sum - 10) as char);
                    has_carry = true;
                } else {
                    new_str.push(sum as char);
                    has_carry = false;
                }
            }
        }
    }

    println!("{new_str:#?}");
    let mut num_iter = new_str.trim().chars().rev();
    let first_ch = num_iter.next().expect("failed to get first ch from iter");
    let mut new_list = Some(Box::new(ListNode::new(match first_ch.to_digit(10) {
        Some(n) => n as i32,
        None => {
            println!("{first_ch:#?}");
            panic!();
        }
    })));

    for ch in num_iter {
        let mut temp = Box::new(ListNode::new(
            ch.to_digit(10).expect("failed to convert ch to digit") as i32,
        ));
        temp.next = new_list;
        new_list = Some(temp);
    }

    new_list
}

fn get_number_from_list(l: Option<Box<ListNode>>) -> String {
    let mut new_str = String::new();
    let mut current = &l;
    while let Some(ref unpacked) = current {
        let ch = char::from_digit(unpacked.val as u32, 10).expect("failed to get char from digit");
        new_str.push(ch);
        current = &unpacked.next;
    }

    new_str
}

#[cfg(test)]
mod add_two_numbers {
    use crate::{add_two_numbers, ListNode};

    #[test]
    fn sample_one() {
        let one_inner = Box::new(ListNode::new(3));
        let mut one_mid = Box::new(ListNode::new(4));
        one_mid.next = Some(one_inner);
        let mut one_outer = Box::new(ListNode::new(2));
        one_outer.next = Some(one_mid);

        let two_inner = Box::new(ListNode::new(4));
        let mut two_mid = Box::new(ListNode::new(6));
        two_mid.next = Some(two_inner);
        let mut two_outer = Box::new(ListNode::new(5));
        two_outer.next = Some(two_mid);

        let expected_inner = Box::new(ListNode::new(8));
        let mut expected_mid = Box::new(ListNode::new(0));
        expected_mid.next = Some(expected_inner);
        let mut expected_outer = Box::new(ListNode::new(7));
        expected_outer.next = Some(expected_mid);

        let actual = add_two_numbers(Some(one_outer), Some(two_outer));
        assert_eq!(actual, Some(expected_outer));
    }
}

#[cfg(test)]
mod get_number_from_list {
    use crate::{get_number_from_list, ListNode};

    #[test]
    fn sample_one() {
        let inner = Box::new(ListNode::new(3));
        let mut mid = Box::new(ListNode::new(4));
        mid.next = Some(inner);
        let mut outer = Box::new(ListNode::new(2));
        outer.next = Some(mid);

        let expected = String::from("243");
        let actual = get_number_from_list(Some(outer));
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let inner = Box::new(ListNode::new(4));
        let mut mid = Box::new(ListNode::new(6));
        mid.next = Some(inner);
        let mut outer = Box::new(ListNode::new(5));
        outer.next = Some(mid);

        let expected = String::from("564");
        let actual = get_number_from_list(Some(outer));
        assert_eq!(actual, expected);
    }
}
