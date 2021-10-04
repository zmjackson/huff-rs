use huff_rs::Tree;
fn main() {
    let tree = Tree::new("mississippi").unwrap();
    let encoded = tree.encode("mississippi");
    match encoded {
        Some(encoding) => {
            println!("Encoded text: {}", encoding);
            let decoded = tree.decode(&encoding);
            match decoded {
                Some(decoding) => println!("Decoded text: {}", decoding),
                None => println!("Decoding failed"),
            }
        }
        None => print!("Encoding failed"),
    };
}
