fn main() {
    let mut node = Node {
        key: 1,
        next: None
    };
    let mut node2 = Node{
        key: 2,
        ..node
    };
    let mut list = LinkedList {
        head: None
    };
    print_list_head_key(&list);
    // let mut populated_list = create_list_with_node(&node);
    list.insert(&mut node);
    print_list_head_key(&list);
    // list.insert(&mut node2);
    // print_list_head_key(&list);
}

fn print_list_head_key(list: &LinkedList) {
    match list.head {
        Some(node) => {
            println!("{}", node.key)
        }
        None => println!("none")
    }
}

struct Node<'n> {
    key: usize,
    next: Option<&'n Node<'n>>
}

struct LinkedList<'n> {
    head: Option<&'n Node<'n>>
}

impl<'n> LinkedList<'n> {
    fn insert(&mut self, x: &'n mut Node<'n>) {
        x.next = self.head;
        self.head = Some(x);
    }
}

fn create_list_with_node<'n>(node: &'n Node<'n>) -> LinkedList {
    LinkedList { head: Some(&node) }
}
