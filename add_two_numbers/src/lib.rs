// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn from_vec(vec: Vec<i32>) -> Option<Box<Self>> {
        vec.into_iter().rfold(None, |head, val| {
            Some(Box::new(ListNode { val, next: head }))
        })
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut carry = 0;
    let mut result = vec![];

    let mut l1 = l1;
    let mut l2 = l2;
    while l1.is_some() || l2.is_some() || carry != 0 {
        let mut sum = carry;
        if let Some(node) = l1 {
            sum += node.val;
            l1 = node.next;
        }
        if let Some(node) = l2 {
            sum += node.val;
            l2 = node.next;
        }

        carry = sum / 10;
        result.push(sum % 10);
    }

    ListNode::from_vec(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = add_two_numbers(
            ListNode::from_vec(vec![2, 4, 3]),
            ListNode::from_vec(vec![5, 6, 4]),
        );
        assert_eq!(result, ListNode::from_vec(vec![7, 0, 8]));
    }

    #[test]
    fn test2() {
        let result = add_two_numbers(ListNode::from_vec(vec![0]), ListNode::from_vec(vec![0]));
        assert_eq!(result, ListNode::from_vec(vec![0]));
    }

    #[test]
    fn test3() {
        let result = add_two_numbers(
            ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]),
            ListNode::from_vec(vec![9, 9, 9, 9]),
        );
        assert_eq!(result, ListNode::from_vec(vec![8, 9, 9, 9, 0, 0, 0, 1]));
    }
}
