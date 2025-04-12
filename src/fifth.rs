#![allow(unused)]
// 一个不错的 UnSafe 队列
// 由于混用 Box 和裸指针，易写出UB代码
use std::ptr;

type Link<T> = Option<Box<Node<T>>>;
pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<'a, T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node { elem, next: None });

        let raw_tail: *mut _ = &mut *new_tail;

        if !self.tail.is_null() {
            // 如果 old_tail 存在，使其指向 new_tail
            // `self.tail` is a raw pointer; try dereferencing it: `(*`, `)`
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }
        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }
            head.elem
        })
    }
}

#[test]
fn test() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), None);
}
