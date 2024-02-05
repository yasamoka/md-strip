use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;
use comrak::{
    format_commonmark,
    nodes::{AstNode, NodeValue},
    parse_document, Arena, Options,
};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    input_filepath: PathBuf,
    #[arg(short, long)]
    output_filepath: PathBuf,
}

fn main() -> Result<()> {
    let Args {
        input_filepath,
        output_filepath,
    } = Args::parse();

    let options = Options::default();
    let arena = Arena::new();

    let root = {
        let input = read_to_string(input_filepath)?;
        parse_document(&arena, &input, &options)
    };

    iter_nodes(root, &|node| {
        let mut value = &mut node.data.borrow_mut().value;
        match &mut value {
            NodeValue::Link(link) => {
                *value = NodeValue::Text(link.title.clone());
            }
            _ => {}
        }
    });

    {
        let mut output = Vec::new();
        format_commonmark(root, &options, &mut output)?;
        write(output_filepath, output)?;
    }

    Ok(())
}

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
where
    F: Fn(&'a AstNode<'a>),
{
    f(node);
    for c in node.children() {
        iter_nodes(c, f);
    }
}
