#![allow(unused)]
/// 上一个版本使用`Option<Box<Node>>`优化了代码
/// 在这个版本上，加入泛型
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn push(&mut self, elem: T) {
        let new_head = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_head);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(node) = cur {
            cur = node.next;
        }
    }
}

#[test]
fn test_list() {
    let mut list: List<&str> = List::new();

    assert_eq!(list.pop(), None);

    list.push("A");
    list.push("B");
    list.push("C");

    assert_eq!(Some("C"), list.pop());
    assert_eq!(Some("B"), list.pop());
    assert_eq!(Some("A"), list.pop());

    assert_eq!(list.pop(), None);
}
