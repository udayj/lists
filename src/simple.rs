type Link<T> = Option<Box<Node<T>>>;

struct Node <T> {
    elem: T,
    next: Link<T>,
}

pub struct List<T> {
    head: Link<T>
}

//push, pop, peek, add after i'th element

impl <T> List <T> {

    pub fn new() -> Self {

        List {head: None}
    }

    pub fn push(&mut self, elem: T) {

        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take()
        });

        self.head = Some(new_node)
    }

    pub fn pop(&mut self) -> Option<T> {

        match self.head.take() {

            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
            None => {
                self.head = None;
                None
            }
        }
    }
}