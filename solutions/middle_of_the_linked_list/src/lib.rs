// 876. Middle of the Linked List
// https://leetcode.com/problems/middle-of-the-linked-list/

// This was given in the problem.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// This is my solution.
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow = &head;
    let mut fast = &head;
    while fast.as_ref().is_some_and(|node| node.next.is_some()) {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }

    slow.clone()
}

#[cfg(test)]
mod tests {
    use crate::{middle_node, ListNode};

    #[test]
    fn sample_one() {
        let input = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));
        let expected = Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 5, next: None })),
            })),
        }));

        let actual = middle_node(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: Some(Box::new(ListNode { val: 6, next: None })),
                        })),
                    })),
                })),
            })),
        }));
        let expected = Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode { val: 6, next: None })),
            })),
        }));

        let actual = middle_node(input);
        assert_eq!(actual, expected);
    }
}
