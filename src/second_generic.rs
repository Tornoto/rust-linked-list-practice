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

// rust 标准库中的一些数据类型，也是为数据类型提供迭代器，再为迭代器实现 Iterator triat
// 由于 IntoIter 迭代器会消耗数据对象，因此可以转移所有权给迭代器。
// 这样一来，无需考虑生命周期，实现起来也比较容易
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // 元组结构体的访问方式
        self.0.pop()
    }
}

// 由于 Iter 迭代器存放的是数据对象成员的引用，需要考虑引用的生命周期
// 'a 表示引用的生命周期，T 表示泛型，T: 'a 表示数据对象成员的生命周期至少长于 'a
// 写成 `pub struct Iter<'a, T>` 亦可
pub struct Iter<'a, T: 'a> {
    current: Option<&'a Node<T>>,
}

impl<T> List<T> {
    // 可以基于生命周期自动推导，写成
    // pub fn iter(&self) -> Iter<'_, T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            current: self.head.as_ref().map(|node| &**node),
            // current: self.head.as_deref(),
        }
    }
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_ref().map(|node| &**node);
            // self.current = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    current: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            current: self.head.as_deref_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // match self.current.take() {
        //     Some(node) => {
        //         self.current = node.next.as_deref_mut();
        //         Some(&mut node.elem)
        //     }
        //     None => None,
        // }

        // 注意这里使用的take()
        self.current.take().map(|node| {
            self.current = node.next.as_deref_mut();
            &mut node.elem
        })
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

#[test]
fn test_into_iter() {
    let mut list: List<String> = List::new();

    list.push("A".to_string());
    list.push("B".to_string());
    list.push("C".to_string());

    for node in list.into_iter() {
        println!("{:?}", node);
    }
}

#[test]
fn test_iter() {
    let mut list: List<String> = List::new();

    list.push("A".to_string());
    // list.push("B".to_string());
    // list.push("C".to_string());

    for node in list.iter() {
        println!("{:?}", node);
    }
}

#[test]
fn test_iter_mut() {
    let mut list: List<String> = List::new();

    list.push("A".to_string());
    list.push("B".to_string());
    list.push("C".to_string());

    for node in list.iter_mut() {
        node.push('*');
    }

    for node in list.iter() {
        println!("{:?}", node);
    }
}
