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

    pub fn len(&self) -> u32 {

        let mut length:u32 = 0;
        let mut node = &self.head;
        loop {
            match node {
                Some(actual_node) => {
                    length+=1;
                    node = &actual_node.next;
                }
                None => {
                    break;
                }
            }
        }
        return length;

    }
    // add element at index
    // calc len of list
    // if index > len add at last
    // if index =0, push at start of the list
    // else count index and add at the appropriate location

    pub fn add_at(&mut self, index: u32, elem: T) {

        let length=self.len();
        let actual_index;
        if index > length {
            actual_index = length;
        }
        else {
            actual_index = index;
        }
        match actual_index {
            0 => {
                self.push(elem);
            }
            _ => {
                let prev_ele = self.get_ref_prev_to(actual_index);
                let mut counter = 0;
                let mut node = &self.head;
                while counter< (index-1) {

                    counter+=1;
                    node=&node.unwrap().next; //node will never be None

                }

                let new_node = Box::new(Node {
                    elem: elem,
                    next: prev_ele.next.take()
                });
        
                prev_ele.next = Some(new_node);
            }
        }
    }

    fn get_ref_prev_to(&self, index: u32) -> &Node<T> {

        let mut counter = 0;
        let mut node = &self.head;
        while counter< (index-1) {

            counter+=1;
            node=&node.unwrap().next; //node will never be None

        }
        //Some(&(node.as_ref().unwrap_or(None).elem))
        &*node.as_ref().unwrap()
        //node.as_ref().map(|actual_node| &actual_node.unwrap())
        
    }
}

#[cfg(test)]
mod tests
{
    use super::List;

    #[test]
    fn basics() {
        let mut x = List::new();
        assert_eq!(x.len(), 0);
        x.push(1);
        assert_eq!(x.len(), 1);
        x.push(2);
        assert_eq!(x.len(), 2);
        x.push(3);
        assert_eq!(x.len(), 3);
        assert_eq!(x.elem_at(1),Some(&2));
        assert_eq!(x.elem_at(0),Some(&3));
        assert_eq!(x.elem_at(2),Some(&1));
        assert_eq!(x.elem_at(3),None);
    }
}