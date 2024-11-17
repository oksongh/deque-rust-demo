use std::vec;

use doublylinkedlist::DoublyLinkedList;
fn main() {
    let mut list = DoublyLinkedList::new();
    list.push_back(1);
    assert_eq!(list.to_string(), "(1)");

    let mut list: DoublyLinkedList<_> = "1.hello! 3.linked 4.list 2.doubly"
        .split(' ')
        .collect::<Vec<_>>()
        .as_slice()
        .into();

    println!(
        "{} {} {} {}",
        list.pop_front().unwrap(),
        list.pop_back().unwrap(),
        list.pop_front().unwrap(),
        list.pop_back().unwrap()
    );
}
