#![allow(unused)]
/// 上一个版本中，使用了
/// ```rust
/// enum Link {
///     Empty,
///     More(Box<Node>),
/// }
/// ```
/// 这个在形式上和Rust的`Option<Box<Node>>`类型很相似，因此可以改写为：
/// ```rust
/// pub struct List {
///     head: Option<Box<Node>>,
/// }
/// ```
/// 为了代码的可读性，给`Option<Box<Node>>`起个别名
/// ```rust
/// type Link = Option<Box<Node>>;
/// ```
///
pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_head = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_head);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
            None => None,
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(node) = cur {
            // cur = node.next.take();
            cur = node.next;
        }
    }
}

#[test]
fn test_first_linked_list() {
    let mut list = List::new();

    assert_eq!(list.pop(), None);

    list.push(1);
    list.push(2);

    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(1));

    assert_eq!(list.pop(), None);
}

#[test]
fn long_list() {
    let mut list = List::new();
    for i in 0..100000000 {
        list.push(i);
    }
    drop(list);
}
