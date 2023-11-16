use std::fmt::Debug;

#[derive(Debug)]
struct Node<T> where 
    T: Debug + PartialEq
{
    data: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> where T:Debug + PartialEq {
    head: Option<Box<Node<T>>>,
}
impl<T: Debug + PartialEq> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }
    fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.head = old_head.next;
            old_head.data
        })
    }
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    fn insert(&mut self, index: usize, data: T) -> Result<(), &'static str> {
        if index == 0 {
            self.push(data);
            return Ok(());
        } else {
            let mut current = &mut self.head;

            for _ in 1..index - 1 {
                match current {
                    Some(node) => current = &mut node.next,
                    None => return Err("Index greater than link list length"),
                };
            }

            let current = match current {
                Some(node) => node,
                None => return Err("Final: Index greater than link list lenght"),
            };

            let temp = Some(Box::new(Node {
                data,
                next: current.next.take(),
            }));

            current.next = temp;

            return Ok(());
        }
    }

    fn delete(&mut self, index: usize) -> Result<(), &'static str> {
        if index == 0 {
            self.pop();
            return Ok(());
        } else {
            let mut current = &mut self.head;
            for _ in 1..index - 2 {
                match current {
                    Some(node) => current = &mut node.next,
                    None => return Err("Index greater than link list length"),
                };
            }

            let current = match current {
                Some(node) => node,
                None => {
                    return Err("Final:
            Index greater than link list lenght")
                }
            };
            println!("node to be deleted");
            let node_to_be_deleted = match current.next.take() {
                Some(node) => node,
                None => return Err("node to be deleted index out of range of the linked list"),
            };
            println!("node_to_be_deleted data: {:?}", node_to_be_deleted.data);
            current.next = node_to_be_deleted.next;
        }
        return Ok(());
    }
    fn includes(&self, data: T) -> Result<bool, &'static str> {
        let mut current = &self.head;

        while let Some(node) = current { 
            if node.data == data {
                return Ok(true);
            }
            current = &node.next;
        }
        return Ok(false);
    }
}

pub fn try_linked_list() -> Result<(), &'static str> {
    let mut linked_list = LinkedList::<u32>::new();

    for i in 0u32..10 {
        linked_list.push(i);
    }

    let _head_node_value: Option<&u32> = linked_list.peek();
    println!("after push");

    print_linked_list(&linked_list);

    // while let Some(popped_value) = linked_list.pop() {}
    println!("after insert");

    linked_list.insert(3, 6969696)?;

    print_linked_list(&linked_list);

    println!("after delete");
    linked_list.delete(4)?;

    print_linked_list(&linked_list);

    println!("Should be false: {}",linked_list.includes(99)?);

    println!("Should be true: {}",linked_list.includes(8)?);

    return Ok(());
}

fn print_linked_list<T: Debug+PartialEq>(list: &LinkedList<T>) {
    let mut current = &list.head;

    while let Some(node) = current {
        println!("{:?}", &node.data);
        current = &node.next;
    }
}
