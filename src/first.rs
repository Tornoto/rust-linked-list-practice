#![allow(unused)]
/// 尝试版 List
/// ```rust
/// pub enum List {
///     Empty,
///     Elem(i32, Box<List>),
/// }
/// ```
/// 假设有链表A→B，那么其内存布局如下：
/// ``` plaintext
/// [] = stack
/// () = heap
/// [ElemA, ptr] → (Elem B, ptr) → (Empty, *junk*)
/// ```
/// 注意，节点A并没有被Box包装，因此分配在栈上，而其他节点由于被Box包装，因此分配在堆上。
///
/// 这导致链表没有整体分配在堆上。在链表分割的场景，需要将堆上的节点拷贝到栈上，从而增加了一些不必要的复杂度。
///
/// 内存一致版本
/// ```rust
/// struct Node {
///     elem: i32,
///     next: List,
/// }
/// pub enum List {
///     Empty,
///     More(Box<Node>),
/// }
/// ```
///
/// 这种写法，内存布局是一致的。而且枚举用来表示链表，结构体类型用来表示Node。
///
/// 进一步分装，屏蔽内部细节，如下：

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    /// 插在链表首位
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: std::mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    // 一种借助自身pop方法的版本
    // fn drop(&mut self) {
    //     while let Some(_) = self.pop() {}
    // }
    fn drop(&mut self) {
        let mut cur = std::mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut node) = cur {
            // cur = std::mem::replace(&mut node.next, Link::Empty)
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
    for i in 0..100000 {
        list.push(i);
    }
    drop(list);
}
