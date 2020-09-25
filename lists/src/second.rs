// Struct with one field, so the struct size is the same as the field. Zero-cost abstraction
pub struct List {
    head: Link,
}

struct Node {
    elem: i32,
    next: Link,
}

type Link = Option<Box<Node>>;

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(2);
        list.push(3);
        
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        
        list.push(4);
        list.push(5);
        
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
       
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
