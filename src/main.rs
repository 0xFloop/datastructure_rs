mod linked_list;
fn main() {
    let result = linked_list::try_linked_list();
    match result {
        Ok(()) => (),
        Err(err) => panic!("{}", err),
    }
}
