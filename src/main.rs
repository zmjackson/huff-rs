use huff_rs::Tree;
fn main() {
    let tree = Tree::new("mississippi".chars()).unwrap();
    let encoded = tree.encode("mississippi".chars());
    match encoded {
        Some(encoding) => {
            println!("Encoded text: {}", encoding);
            let decoded = tree.decode(&encoding).iter().collect::<String>();
            println!("{}", decoded);
        }
        None => print!("Encoding failed"),
    };
}
