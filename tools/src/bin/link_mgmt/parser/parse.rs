use std::io::Write;

use anyhow::bail;
use anyhow::Result;
use pulldown_cmark::Event;
use pulldown_cmark::Parser;
use pulldown_cmark::Tag;

// use crate::link::Link;

fn _extract_links(parser: Parser) -> Result<()> {
    // TODO -> impl Iterator<Item = Event<'input>> + 'callback
    // let closure = |broken_link: BrokenLink<'a>| { callback(broken_link,
    // markdown_input) }; let parser =
    // Parser::new_with_broken_link_callback(     markdown_input,
    //     get_options(),
    //     Some(&mut closure),
    // );

    let mut in_link = Vec::new();
    let mut links = Vec::new();

    // Retrieve and group all Link-related events
    parser.for_each(|event| {
        match event {
            // Start of a link
            e @ Event::Start(Tag::Link(..)) => {
                println!("{:?}", e);
                in_link.push(vec![e]);
            }

            // End of the link
            e @ Event::End(Tag::Link(..)) => {
                println!("{:?}", e);
                let mut l = in_link.pop().unwrap();
                l.push(e);
                links.push(l);
            }

            // Accumulate events while in the link
            e if !in_link.is_empty() => {
                println!("{:?}", e);
                in_link.last_mut().unwrap().push(e);
            }

            e => {
                println!("IGNORED: {:?}", e);
            }
        }
    });

    //
    for l in links.iter() {
        // let li = Link::new();
        if let Event::Start(Tag::Link(link_type, dest_url, title)) = &l[0] {
            print!(
                "Link: link_type: {:?}, url: {}, title: {}",
                link_type,
                dest_url.clone(),
                title.clone()
            );
        } else {
            bail!("Expected Event::Start");
        }
        match &l[1] {
            Event::Text(s) => {
                println!(", text: {}", s);
            }
            Event::Start(Tag::Image(link_type, dest_url, title)) => {
                print!(
                    "; image: link_type: {:?}, url: {}, title: {}",
                    link_type, dest_url, title
                );
                if let Event::Text(lbl) = &l[2] {
                    println!(", label: {}", lbl)
                }
            }
            _ => {
                bail!("Expected text or image.");
            }
        };
    }

    // let markdown_input_length = markdown_input.as_ref().len();
    // write_markdown_to(parser, markdown_input_length, f)?;

    Ok(())
}

// Parse the Markdown as events and write them all to e.g. file.
// See https://docs.rs/pulldown-cmark/latest/pulldown_cmark/enum.Event.html
// and https://docs.rs/pulldown-cmark/latest/pulldown_cmark/enum.Tag.html
pub fn debug_parse_to<W>(parser: Parser, mut f: W) -> Result<()>
where
    W: Write,
{
    for event in parser {
        match &event {
            // Start of a tagged element. Events that are yielded after this
            // event and before its corresponding End event are inside this
            // element. Start and end events are guaranteed to be balanced.
            Event::Start(tag) => {
                writeln!(&mut f, "Start({:?})", tag)?;

                match tag {
                    Tag::Paragraph => {
                        writeln!(&mut f, "Event::Start(Tag::Paragraph)")?;
                    }
                    // A heading. The first field indicates the level of the
                    // heading, the second the fragment identifier, and the
                    // third the classes.
                    Tag::Heading(level, id, classes) => {
                        writeln!(
                            &mut f,
                            "Event::Start(Tag::Heading( heading_level: {} fragment identifier: {:?} classes: {:?} ))",
                            level, id, classes
                        )?;
                    }
                    Tag::BlockQuote => {
                        writeln!(&mut f, "Event::Start(Tag::BlockQuote)")?;
                    }
                    // A code block.
                    Tag::CodeBlock(code_block_kind) => {
                        writeln!(
                            &mut f,
                            "Event::Start(Tag::CodeBlock(code_block_kind: {:?} ))",
                            code_block_kind
                        )?;
                    }
                    // A list. If the list is ordered the field indicates the
                    // number of the first item. Contains only list items.
                    Tag::List(ordered_list_first_item_number) => {
                        writeln!(
                            &mut f,
                            "Event::Start(Tag::List( ordered_list_first_item_number: {:?} ))",
                            ordered_list_first_item_number
                        )?;
                    }
                    // A list item.
                    Tag::Item => {
                        writeln!(
                            &mut f,
                            "Event::Start(Tag::Item) (this is a list item)"
                        )?;
                    }
                    // A footnote definition. The value contained is the
                    // footnote’s label by which it can be referred to.
                    Tag::FootnoteDefinition(label) => {
                        writeln!(
                            &mut f,
                            "Event::Start(Tag::FootnoteDefinition( label: {} ))",
                            label
                        )?;
                    }
                    // A table. Contains a vector describing the text-alignment
                    // for each of its columns.
                    Tag::Table(column_text_alignment_list) => {
                        writeln!(
                            &mut f,
                            "Event::Start(Tag::Table( column_text_alignment_list: {:?} ))",
                            column_text_alignment_list
                        )?;
                    }
                    // A table header. Contains only TableCells. Note that the
                    // table body starts immediately after the closure of the
                    // TableHead tag. There is no TableBody tag.
                    Tag::TableHead => {
                        writeln!(
                            &mut f,
                            "Event::Start(Tag::TableHead) (contains TableRow tags)"
                        )?;
                    }
                    // A table row. Is used both for header rows as body rows.
                    // Contains only TableCells.
                    Tag::TableRow => {
                        writeln!(
                            &mut f,
                            "Event::Start(Tag::TableRow) (contains TableCell tags)"
                        )?;
                    }
                    Tag::TableCell => {
                        writeln!(
                            &mut f,
                            "Event::Start(Tag::TableCell) (contains inline tags)"
                        )?;
                    }
                    Tag::Emphasis => {
                        writeln!(
                            &mut f,
                            "Event::Start(Tag::Emphasis) (this is a span tag)"
                        )?;
                    }
                    Tag::Strong => {
                        writeln!(
                            &mut f,
                            "Event::Start(Tag::Strong) (this is a span tag)"
                        )?;
                    }
                    Tag::Strikethrough => {
                        writeln!(
                            &mut f,
                            "Event::Start(Tag::Strikethrough) (this is a span tag)"
                        )?;
                    }
                    // A link. The first field is the link type, the second the
                    // destination URL and the third is a title.
                    Tag::Link(link_type, dest_url, title) => {
                        writeln!(
                            &mut f,
                            "Event::Start(Tag::Link() link_type: {:?} url: {} title: {} ))",
                            link_type, dest_url, title
                        )?;
                    }
                    // An image. The first field is the link type, the second
                    // the destination URL and the third is a title.
                    Tag::Image(link_type, dest_url, title) => {
                        writeln!(
                            &mut f,
                            "Event::Start(Tag::Image( link_type: {:?} url: {} title: {} ))",
                            link_type, dest_url, title
                        )?;
                    }
                }
            }
            // End of a tagged element.
            Event::End(tag) => {
                writeln!(&mut f, "Event::End({:?})", tag)?;
            }
            // A text node.
            Event::Text(s) => {
                writeln!(&mut f, "Event::Text({:?})", s)?;
            }
            // An inline code node.
            Event::Code(s) => {
                writeln!(&mut f, "Event::Code({:?})", s)?;
            }
            // An HTML node.
            Event::Html(s) => {
                writeln!(&mut f, "Event::Html({:?})", s)?;
            }
            // A reference to a footnote with given label, which may or may not
            // be defined by an event with a Tag::FootnoteDefinition tag.
            // Definitions and references to them may occur in any order.
            Event::FootnoteReference(s) => {
                writeln!(&mut f, "Event::FootnoteReference({:?})", s)?;
            }
            // A soft line break.
            Event::SoftBreak => {
                writeln!(&mut f, "Event::SoftBreak")?;
            }
            // A hard line break.
            Event::HardBreak => {
                writeln!(&mut f, "Event::HardBreak")?;
            }
            // A horizontal ruler.
            Event::Rule => {
                writeln!(&mut f, "Event::Rule")?;
            }
            // A task list marker, rendered as a checkbox in HTML. Contains a
            // true when it is checked.
            Event::TaskListMarker(b) => {
                writeln!(&mut f, "Event::TaskListMarker({:?})", b)?;
            }
        };
    }
    Ok(())
}
