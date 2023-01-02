use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link
}

impl List {

    pub fn new() -> Self {
        List {head: Link::Empty}
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty) 

        });

        self.head = Link::More(new_node);

    }

    pub fn pop(&mut self) -> Option<i32> {

        let result;
        match mem::replace(&mut self.head, Link::Empty) {

            Link::Empty => {
                result = None;
            }
            Link::More(node) => {
                result = Some(node.elem);
                self.head = node.next;
            }
        }
        result

    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}


#[cfg(test)]
mod test {
    
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

    }
}