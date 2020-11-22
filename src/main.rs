type NodeRef<T> = Box<Node<T>>;
type NodeOption<U> = Option<NodeRef<U>>;

#[derive(Debug)]
pub struct Node<U> {
    val: U,
    next: NodeOption<U>,
}

impl<T> Node<T> {
    pub fn new(val: T, next: Option<NodeRef<T>>) -> Node<T> {
        Node {
            val,
            next,
        }
    }
    pub fn new_empty(value: T) -> Node<T> {
        Node {
            val: value,
            next: None,
        }
    }
}

#[derive(Debug)]
pub struct ListL<V> {
    head: Option<Node<V>>
}

impl<U> ListL<U> {
    pub fn new() -> ListL<U> {
        ListL {
            head: None
        }
    }

    pub fn push(&mut self, value: U) {
        let old_head = self.head.replace(Node::new_empty(value));
        match &mut self.head {
            Some(head) => {
                if old_head.is_some() {
                    head.next = Some(Box::new(old_head.unwrap()));
                }
            }
            None => {}
        }
    }


    pub fn length(&self) -> usize {
        let mut count = 0;
        let mut current_node = self.head.as_ref();
        while let Some(node) = current_node {
            count = count + 1;
            current_node = node.next.as_ref().map(|node| &**node)
        }
        count
    }

    fn get_nth_node_mut(&mut self, n: usize) -> Option<&mut Node<U>> {
        let mut nth_node = self.head.as_mut();
        for _ in 0..n {
            nth_node = match nth_node {
                None => return None,
                Some(node) => node.next.as_mut().map(|node| &mut **node),
            }
        }
        nth_node
    }

    fn pop_tail(&mut self) -> U{
        let len = self.length();
        let mut tail = self.get_nth_node_mut(len - 2);
        let mut_node = tail.take();
        let mut res = None;
        mut_node.map(|val| {
            res = val.next.take();
            val.next = None;
        });
        res.unwrap().val
    }

    fn delete_at_index(&mut self, index: &usize) {
        let mut prev = self.get_nth_node_mut(index - 2);
        prev.map(|val| {
            let temp = val.next.take().unwrap().next;
            val.next.replace(temp.unwrap());
        });
    }
}

fn main() {
    let mut list: ListL<u16> = ListL::new();
    list.push(1);
    list.push(3);
    list.push(4);
    list.push(5);
    list.push(6);
    list.push(7);
    println!("{:?}", list);
    println!("List length {}", list.length());

    let el = list.pop_tail();
    println!("Deleted {}", el);
    println!("{:?}", list);
    println!("List length {}", list.length());

    list.delete_at_index(&3);
    println!("{:?}", list);
    println!("List length {}", list.length());
}
