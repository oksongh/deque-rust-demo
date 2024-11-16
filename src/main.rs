use doublylinkedlist::DoublyLinkedList;
fn main() {
    let mut list = DoublyLinkedList::new();
    list.push_back(1);
    assert_eq!(list.to_string(), "(1)");

    let a = std::cell::Cell::new(10); // immutable object with interior mutability
    let b = a.replace(20);
    dbg!(a.get()); // a.get() = 20
    dbg!(b); // b = 10

    let c = a.clone().into_inner(); // turn Cell<T> into T
    dbg!(c); // c = 20
    dbg!(a); // borrow check - Error

    // let s = std::cell::Cell::new(String::from("aaaa"));
}
