use std::io::Write;

use anyhow::Result;
use pulldown_cmark::Event;
use pulldown_cmark::Parser;
use pulldown_cmark::Tag;

/// Parse Markdown and write all raw events to e.g. a file
/// for debugging purposes
///
/// parser: source Markdown parser
/// w: writer / file to write to
///
/// See https://docs.rs/pulldown-cmark/latest/pulldown_cmark/enum.Event.html
/// and https://docs.rs/pulldown-cmark/latest/pulldown_cmark/enum.Tag.html
pub(super) fn debug_parse_to<W>(parser: Parser, mut w: W) -> Result<()>
where
    W: Write,
{
    for event in parser {
        match &event {
            // Start of a tagged element. Events that are yielded after this
            // event and before its corresponding End event are inside this
            // element. Start and end events are guaranteed to be balanced.
            Event::Start(tag) => {
                writeln!(&mut w, "Start({:?})", tag)?;

                match tag {
                    Tag::Paragraph => {
                        writeln!(&mut w, "Event::Start(Tag::Paragraph)")?;
                    }
                    // A heading. The first field indicates the level of the
                    // heading, the second the fragment identifier, and the
                    // third the classes.
                    Tag::Heading(level, id, classes) => {
                        writeln!(
                            &mut w,
                            "Event::Start(Tag::Heading( heading_level: {} fragment identifier: {:?} classes: {:?} ))",
                            level, id, classes
                        )?;
                    }
                    Tag::BlockQuote => {
                        writeln!(&mut w, "Event::Start(Tag::BlockQuote)")?;
                    }
                    // A code block.
                    Tag::CodeBlock(code_block_kind) => {
                        writeln!(
                            &mut w,
                            "Event::Start(Tag::CodeBlock(code_block_kind: {:?} ))",
                            code_block_kind
                        )?;
                    }
                    // A list. If the list is ordered the field indicates the
                    // number of the first item. Contains only list items.
                    Tag::List(ordered_list_first_item_number) => {
                        writeln!(
                            &mut w,
                            "Event::Start(Tag::List( ordered_list_first_item_number: {:?} ))",
                            ordered_list_first_item_number
                        )?;
                    }
                    // A list item.
                    Tag::Item => {
                        writeln!(
                            &mut w,
                            "Event::Start(Tag::Item) (this is a list item)"
                        )?;
                    }
                    // A footnote definition. The value contained is the
                    // footnote’s label by which it can be referred to.
                    Tag::FootnoteDefinition(label) => {
                        writeln!(
                            &mut w,
                            "Event::Start(Tag::FootnoteDefinition( label: {} ))",
                            label
                        )?;
                    }
                    // A table. Contains a vector describing the text-alignment
                    // for each of its columns.
                    Tag::Table(column_text_alignment_list) => {
                        writeln!(
                            &mut w,
                            "Event::Start(Tag::Table( column_text_alignment_list: {:?} ))",
                            column_text_alignment_list
                        )?;
                    }
                    // A table header. Contains only TableCells. Note that the
                    // table body starts immediately after the closure of the
                    // TableHead tag. There is no TableBody tag.
                    Tag::TableHead => {
                        writeln!(
                            &mut w,
                            "Event::Start(Tag::TableHead) (contains TableRow tags)"
                        )?;
                    }
                    // A table row. Is used both for header rows as body rows.
                    // Contains only TableCells.
                    Tag::TableRow => {
                        writeln!(
                            &mut w,
                            "Event::Start(Tag::TableRow) (contains TableCell tags)"
                        )?;
                    }
                    Tag::TableCell => {
                        writeln!(
                            &mut w,
                            "Event::Start(Tag::TableCell) (contains inline tags)"
                        )?;
                    }
                    Tag::Emphasis => {
                        writeln!(
                            &mut w,
                            "Event::Start(Tag::Emphasis) (this is a span tag)"
                        )?;
                    }
                    Tag::Strong => {
                        writeln!(
                            &mut w,
                            "Event::Start(Tag::Strong) (this is a span tag)"
                        )?;
                    }
                    Tag::Strikethrough => {
                        writeln!(
                            &mut w,
                            "Event::Start(Tag::Strikethrough) (this is a span tag)"
                        )?;
                    }
                    // A link. The first field is the link type, the second the
                    // destination URL and the third is a title.
                    Tag::Link(link_type, dest_url, title) => {
                        writeln!(
                            &mut w,
                            "Event::Start(Tag::Link() link_type: {:?} url: {} title: {} ))",
                            link_type, dest_url, title
                        )?;
                    }
                    // An image. The first field is the link type, the second
                    // the destination URL and the third is a title.
                    Tag::Image(link_type, dest_url, title) => {
                        writeln!(
                            &mut w,
                            "Event::Start(Tag::Image( link_type: {:?} url: {} title: {} ))",
                            link_type, dest_url, title
                        )?;
                    }
                }
            }
            // End of a tagged element.
            Event::End(tag) => {
                writeln!(&mut w, "Event::End({:?})", tag)?;
            }
            // A text node.
            Event::Text(s) => {
                writeln!(&mut w, "Event::Text({:?})", s)?;
            }
            // An inline code node.
            Event::Code(s) => {
                writeln!(&mut w, "Event::Code({:?})", s)?;
            }
            // An HTML node.
            Event::Html(s) => {
                writeln!(&mut w, "Event::Html({:?})", s)?;
            }
            // A reference to a footnote with given label, which may or may not
            // be defined by an event with a Tag::FootnoteDefinition tag.
            // Definitions and references to them may occur in any order.
            Event::FootnoteReference(s) => {
                writeln!(&mut w, "Event::FootnoteReference({:?})", s)?;
            }
            // A soft line break.
            Event::SoftBreak => {
                writeln!(&mut w, "Event::SoftBreak")?;
            }
            // A hard line break.
            Event::HardBreak => {
                writeln!(&mut w, "Event::HardBreak")?;
            }
            // A horizontal ruler.
            Event::Rule => {
                writeln!(&mut w, "Event::Rule")?;
            }
            // A task list marker, rendered as a checkbox in HTML. Contains a
            // true when it is checked.
            Event::TaskListMarker(b) => {
                writeln!(&mut w, "Event::TaskListMarker({:?})", b)?;
            }
        };
    }
    Ok(())
}
