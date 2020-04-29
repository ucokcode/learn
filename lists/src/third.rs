use std::cell::RefCell;
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}
impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }
}

// pub trait Drop {
//     fn drop(&mut self);
// }
//
// impl<T> Drop for List<T> {
//     fn drop(&mut self) {
//         let mut head = self.head.take();
//         while let Some(node) = head {
//             if let Ok(mut node) = Rc::try_unwrap(node) {
//                 head = node.next.take();
//             } else {
//                 break;
//             }
//         }
//     }
// }
// // pub struct IterMut<'a, T> {
// //     next: Option<&'a mut Node<T>>,
// // }
// //
// pub struct Iter<'a, T> {
//     next: Option<&'a Node<T>>,
// }
//
// impl<T> List<T> {
//     pub fn iter<'a>(&'a self) -> Iter<'a, T> {
//         Iter {
//             next: self.head.as_ref().map(|node| &**node),
//         }
//     }
//     // pub fn iter_mut(&mut self) -> IterMut<'_, T> {
//     //     IterMut {
//     //         next: self.head.as_mut().map(|node| &mut **node),
//     //     }
//     // }
// }
// //
// // impl<'a, T> Iterator for IterMut<'a, T> {
// //     type Item = &'a mut T;
// //     fn next(&mut self) -> Option<Self::Item> {
// //         self.next.take().map(|node| {
// //             self.next = node.next.as_mut().map(|node| &mut **node);
// //             &mut node.elem
// //         })
// //     }
// // }
// //
// impl<'a, T> Iterator for Iter<'a, T> {
//     type Item = &'a T;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.next.map(|node| {
//             self.next = node.next.as_ref().map(|node| &**node);
//             &node.elem
//         })
//     }
// }
// //
// #[cfg(test)]
// mod test {
//     use super::List;
//
//     #[test]
//     fn basics() {
//         let list = List::new();
//         assert_eq!(list.head(), None);
//
//         let list = list.append(1).append(2).append(3);
//         assert_eq!(list.head(), Some(&3));
//
//         let list = list.tail();
//         assert_eq!(list.head(), Some(&2));
//
//         let list = list.tail();
//         assert_eq!(list.head(), Some(&1));
//
//         let list = list.tail();
//         assert_eq!(list.head(), None);
//
//         // Make sure empty tail works
//         let list = list.tail();
//         assert_eq!(list.head(), None);
//     }
//     #[test]
//     fn iter() {
//         let list = List::new().append(1).append(2).append(3);
//
//         let mut iter = list.iter();
//         assert_eq!(iter.next(), Some(&3));
//         assert_eq!(iter.next(), Some(&2));
//         assert_eq!(iter.next(), Some(&1));
//     }
// }
