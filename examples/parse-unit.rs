use anyhow::Result;
use facts::syntax::parser;
use rowan::NodeOrToken;
use std::collections::VecDeque;

fn print(node: parser::SyntaxNode) {
    let mut queue = VecDeque::new();
    queue.push_back((0, NodeOrToken::Node(node)));

    while let Some((indent, n)) = queue.pop_front() {
        print!("{:indent$}", "", indent = indent);

        match n {
            NodeOrToken::Node(node) => {
                println!("- {:?}", node.kind());
                let v = node.children_with_tokens().collect::<Vec<_>>();

                for child in v.into_iter().rev() {
                    queue.push_front((indent + 2, child));
                }
            }
            NodeOrToken::Token(token) => {
                println!(
                    "- {:?} {:?} ({:?})",
                    token.text(),
                    token.kind(),
                    token.text_range()
                );
            }
        }
    }
}

fn main() -> Result<()> {
    let mut it = std::env::args();
    it.next();
    let query = it.collect::<Vec<_>>().join(" ");

    let parser = parser::Parser::new(&query);
    let node = parser.parse_unit();
    print(node);
    Ok(())
}
