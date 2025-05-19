fn main() {
    let source = include_str!("../test-files/basic-structs.rs");
    println!("{}", source);

    let mut iter = source.chars();
    while let Some(c) = iter.next() {
        match c {
            '{' => {
                make_block(&mut iter);
            }
            _ => {}
        }
    }
}

fn make_block(iter: &mut impl Iterator<Item=char>) {
    while let Some(c) = iter.next() {
        if c == '}' {
            break;
        }

        
    }
}

fn parse_word() {

}
