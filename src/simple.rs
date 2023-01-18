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
 
    pub fn elem_at(&self, index: u32) -> Option<&T>{

        let mut counter = 0;
        let mut node = &self.head;
        while counter<index {

            match node {
                Some(actual_node) => {
                    counter+=1;
                    node=&actual_node.next;
                }
                None => {
                    return None;
                }
            }

        }
        //Some(&(node.as_ref().unwrap_or(None).elem))
        node.as_ref().map(|actual_node| &actual_node.elem)
    }
}

#[cfg(test)]
mod tests
{
    use super::List;

    #[test]
    fn basics() {
        let mut x = List::new();
        x.push(1);
        x.push(2);
        x.push(3);
        assert_eq!(x.elem_at(1),Some(&2));
        assert_eq!(x.elem_at(0),Some(&3));
        assert_eq!(x.elem_at(2),Some(&1));
        assert_eq!(x.elem_at(3),None);
    }
}