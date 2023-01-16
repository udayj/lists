type Link<T> = Option<Box<Node<T>>>;

struct Node <T> {
    elem: T,
    next: Link<T>,
}

pub struct List<T> {
    head: Link<T>
}