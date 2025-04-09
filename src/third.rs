#![allow(unused)]
use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;
pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem,
                // Rc clone 方法仅仅增加引用计数
                next: self.head.clone(),
            })),
        }
    }

    // 返回移除首节点后的子链表
    pub fn tail(&self) -> List<T> {
        List {
            // 注意 Option::map方法的定义，由于node.next自身是Option类型，导致返回 Option<Option<...>>
            // head: self.head.as_ref().map(|node| node.next.clone()), // error
            // 改用 and_then
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_deref().map(|node| &node.elem)
    }
}

// todo 如何实现 IntoIter 和 IterMut

pub struct Iter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));
    }
}
