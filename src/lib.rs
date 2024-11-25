use std::{
    cell::RefCell,
    fmt::{self, Debug},
    rc::Rc,
};

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}
pub struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}
impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            // oldtailにprevがない<=>要素1つだけのリスト
            let new_tail = old_tail.borrow().prev.clone();
            match new_tail {
                //  old_tailへの参照をローカル変数old_tailだけにする
                Some(new_tail) => {
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head = None;
                    // self.tail = None; already None by take()
                }
            }
            Rc::into_inner(old_tail).unwrap().into_inner().data
        })
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            let new_head = old_head.borrow().next.clone();
            match new_head {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => self.tail = None,
            };
            Rc::into_inner(old_head).unwrap().into_inner().data
        })
    }
    pub fn push_back(&mut self, elm: T) {
        let new_tail = Rc::new(RefCell::new(Node {
            data: elm,
            next: None,
            prev: None,
        }));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&new_tail));
                new_tail.borrow_mut().prev = Some(old_tail);
            }
            None => {
                // first node
                self.head = Some(Rc::clone(&new_tail));
            }
        }
        self.tail = Some(Rc::clone(&new_tail));
    }
    pub fn push_front(&mut self, elm: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data: elm,
            next: None,
            prev: None,
        }));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::clone(&new_node));
                new_node.borrow_mut().next = Some(old_head);
            }
            None => {
                // first node
                self.tail = Some(Rc::clone(&new_node));
            }
        }
        self.head = Some(Rc::clone(&new_node));
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        // let a =
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.data))
    }
    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.data))
    }
}

impl<T: Debug + Clone> DoublyLinkedList<T> {
    pub fn read(&self, index: usize) -> Option<Rc<RefCell<Node<T>>>> {
        let mut cur = self.head.clone();
        let mut cnt = 0;
        loop {
            // println!("{cnt}");
            // if cnt == 4 {
            //     break;
            // }
            match cur {
                Some(_cur) if cnt < index => {
                    cur = _cur.borrow().next.clone();
                    cnt += 1;
                    println!("{:?}", _cur.borrow().data);
                }
                // Some(_cur) => return Some(&(_cur.borrow().data)),
                // None => return None,
                _ => break,
            }
        }

        // let cur = cur;
        cur.map(|cur| Rc::clone(&cur))
    }
    pub fn print_string_count(&self) {
        println!("string count");

        let mut cur = self.head.clone();
        while let Some(c) = cur {
            cur = c.borrow().next.clone();
            println!("{}", Rc::strong_count(&c));
        }
    }
}

impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Clone> From<&[T]> for DoublyLinkedList<T> {
    fn from(elms: &[T]) -> DoublyLinkedList<T> {
        let mut list = Self::new();
        elms.iter().for_each(|e| {
            list.push_back(e.clone());
        });
        list
    }
}
impl<T: Clone, const N: usize> From<&[T; N]> for DoublyLinkedList<T> {
    fn from(elms: &[T; N]) -> DoublyLinkedList<T> {
        Self::from(&elms[..])
    }
}
impl<T, const N: usize> From<[T; N]> for DoublyLinkedList<T> {
    fn from(elms: [T; N]) -> Self {
        let mut list = Self::new();
        elms.into_iter().for_each(|e| {
            list.push_back(e);
        });
        list
    }
}

impl<T> fmt::Display for DoublyLinkedList<T>
where
    T: fmt::Display + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = self.head.clone();
        write!(f, "(")?;

        while let Some(node) = current {
            // let n = RefCell::borrow(&node);
            let n = node.borrow();
            write!(f, "{}", n.data)?;
            // write!(f, "@{:p}", Rc::as_ptr(&node))?;

            current = n.next.clone();
            if current.is_some() {
                write!(f, "<--->")?;
            }
        }
        write!(f, ")")?;

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_back1() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        println!("{list}");
        assert_eq!(list.to_string(), "(1)")
    }
    #[test]
    fn push_back1_3() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.to_string(), "(1<--->2<--->3)")
    }
    #[test]
    fn push_front1_3() {
        let mut list = DoublyLinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.to_string(), "(3<--->2<--->1)")
    }
    #[test]
    fn push_back_front() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_front(4);
        list.push_front(5);
        assert_eq!(list.to_string(), "(5<--->4<--->1<--->2<--->3)");
    }

    #[test]
    fn from() {
        let list = DoublyLinkedList::from(&[1, 2, 3, 4, 5][..]);
        assert_eq!(list.to_string(), "(1<--->2<--->3<--->4<--->5)");
    }

    #[test]
    fn from_const_size() {
        let list = DoublyLinkedList::from(&[1, 2, 3, 4, 5]);
        assert_eq!(list.to_string(), "(1<--->2<--->3<--->4<--->5)");
    }

    #[test]
    fn pop_back() {
        let array = &[1, 2, 3, 4, 5];
        let mut list: DoublyLinkedList<i32> = array.into();
        for &e in array.iter().rev() {
            assert_eq!(e, list.pop_back().unwrap());
        }
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn pop_front() {
        let array = &[1, 2, 3, 4, 5];
        let mut list: DoublyLinkedList<i32> = array.into();
        for &e in array {
            assert_eq!(e, list.pop_front().unwrap());
        }
        assert_eq!(list.pop_front(), None);
    }
    #[test]
    fn peek_back() {
        assert!(DoublyLinkedList::<i32>::new().peek_back().is_none());

        let list = DoublyLinkedList::from(&[1, 2, 3, 4, 5]);
        assert_eq!(*list.peek_back().unwrap(), 5);
    }
    #[test]
    fn peek_front() {
        assert!(DoublyLinkedList::<i32>::new().peek_front().is_none());

        let list = DoublyLinkedList::from(&[1, 2, 3, 4, 5]);
        assert_eq!(*list.peek_front().unwrap(), 1);
    }

}
