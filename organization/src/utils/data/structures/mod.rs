pub struct SingleLinkedList<T> {
    // TODO: implement the linked list structure
    // Do we go with a `RefCel<Rc<Node<T>>>`
    //
    head: Option<Box<Node<T>>>,
}

pub struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Self {
        Self { elem, next: None }
    }
}

impl<T> SingleLinkedList<T> {
    pub fn new(elem: T) -> Self {
        Self {
            head: Some(Box::new(Node::new(elem))),
        }
    }

    pub fn empty() -> Self {
        Self { head: None }
    }

    pub fn head_borrow(&self) -> &Option<Box<Node<T>>> {
        &self.head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_head() {
        let mut list: SingleLinkedList<String> = SingleLinkedList::empty();
        let borrowed = list.head_borrow();
        if let Some(non_empty_option) = borrowed {
            panic!();
        }
    }
}
