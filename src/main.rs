use linked_list::List;

fn main() {
    let mut list = List::new();

    list = list.insert(30);
    list = list.insert(28);
    list = list.insert(400);
    list = list.insert(54);
    list = list.insert(68);
    
    println!("linked list has length: {}", list.len());
    println!("{}", list.print());

    list.remove(54);
    list.remove(68);

    println!("{}", list.print());
    println!("linked list has length: {}", list.len());
}