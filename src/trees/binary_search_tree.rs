#[derive(Debug)]
struct Node<T>
where
    T: Debug + PartialEq,
{
    data: T,
    parent: Option<Box<Node<T>>>,
}

struct BinarySearchTree{
    root: Node,
    length: u32,
}
impl BinarySearchTree{
    fn size(&self)->u32{
        return self.length;
    }
}