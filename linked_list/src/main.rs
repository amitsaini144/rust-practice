use std::collections::LinkedList;


fn create_linkedlist(list: &[u32]) -> LinkedList<u32> {
    let mut link_list = LinkedList::new();

    for &item in list {
        link_list.push_back(item);
    }
    link_list
}

fn main() {
    let list = vec![5, 4, 3, 2, 1];

    let linked_list = create_linkedlist(&list);

    for item in &linked_list {
        println!("{}", item);
    }

    let mut iter = linked_list.iter();

    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);
}
