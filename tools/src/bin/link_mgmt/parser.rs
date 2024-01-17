#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use pulldown_cmark::html;
use pulldown_cmark::BrokenLink;
use pulldown_cmark::BrokenLinkCallback;
use pulldown_cmark::CowStr;
use pulldown_cmark::Event;
use pulldown_cmark::LinkDef;
use pulldown_cmark::Options;
use pulldown_cmark::Parser;
use pulldown_cmark::Tag;
use pulldown_cmark_to_cmark::cmark;

// See: https://crates.io/crates/pulldown-cmark

// Private functions

fn get_options() -> Options {
    // Set up options and parser.
    // Strikethroughs, etc... are not part of the CommonMark standard
    // and we therefore must enable them explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TABLES);
    options
}

// In case the parser encounters any potential links that have a
// broken reference (e.g [foo] when there is no [foo]:  entry at the
// bottom) the provided callback will be called with the
// reference name, and the returned pair will be used as the link name
// and title if it is not None.
fn callback<'input>(
    broken_link: BrokenLink<'input>,
    markdown_input: &'input str,
) -> Option<(CowStr<'input>, CowStr<'input>)> {
    println!(
        "Issue with the markdown: reference: {}, `{}`, type: {:?}",
        broken_link.reference,
        &markdown_input[broken_link.span],
        broken_link.link_type,
    );
    Some(("https://TODO".into(), "".into()))
}

fn write_html_to_stdout<'a, I>(parser: I)
where
    I: Iterator<Item = Event<'a>>,
{
    // Write to stdout. This could also be anything implementing the
    // `Write` trait e.g., a file or network socket.
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(b"\nHTML output:\n").unwrap();
    html::write_html(&mut handle, parser).unwrap();
}

fn write_html_to_bytes<'a, I>(parser: I) -> Vec<u8>
where
    I: Iterator<Item = Event<'a>>,
{
    let mut bytes = Vec::new();
    // A Cursor wraps an in-memory buffer
    html::write_html(std::io::Cursor::new(&mut bytes), parser);
    bytes
}

fn write_html_to_string<'a, I>(parser: I) -> String
where
    I: Iterator<Item = Event<'a>>,
{
    // Write to a new String buffer
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}

fn write_markdown_to<'a, I, E, W>(
    parser: I,
    markdown_input_length: usize,
    mut w: W,
) -> Result<()>
where
    I: Iterator<Item = E>,
    E: Borrow<Event<'a>>,
    W: Write,
{
    let mut buf = String::with_capacity(markdown_input_length + 128);
    let options = pulldown_cmark_to_cmark::Options {
        // newlines_after_headline:,
        // newlines_after_paragraph:,
        // newlines_after_codeblock:,
        // newlines_after_table:,
        // newlines_after_rule:,
        // newlines_after_list:,
        // newlines_after_blockquote:,
        // newlines_after_rest:,
        // code_block_token_count:,
        // code_block_token: '',
        // list_token: '',
        // ordered_list_token: '',
        // increment_ordered_list_bullets: true,
        // emphasis_token: '',
        // strong_token: "",
        ..pulldown_cmark_to_cmark::Options::default()
    };
    pulldown_cmark_to_cmark::cmark_with_options(parser, &mut buf, options)?;
    w.write_all(buf.as_bytes())?;
    Ok(())
}

fn write_ref_defs<P: AsRef<Path>>(parser: &Parser, path: P) -> Result<()> {
    let refdefs = parser.reference_definitions();

    // Sorted Map
    let sorted_refdefs: BTreeMap<_, _> = refdefs.iter().collect();
    let mut f = std::fs::File::create(path)?;
    for (s, LinkDef { dest, title, .. }) in sorted_refdefs {
        if let Some(t) = title {
            writeln!(&mut f, "[{s}]: {dest} \"{t:?}\"");
        } else {
            writeln!(&mut f, "[{s}]: {dest}");
        }
    }
    Ok(())
}

// Public functions

// Parse the Markdown as events and print them all.
// See https://docs.rs/pulldown-cmark/latest/pulldown_cmark/enum.Event.html
// and https://docs.rs/pulldown-cmark/latest/pulldown_cmark/enum.Tag.html
pub fn debug_parse_to_stdout<S: AsRef<str>>(markdown_input: S) {
    println!("\nParsing markdown ---------------\n");

    //// Set up the parser. We can treat is as any other iterator.
    //// For each event, we print its details, such as the tag or string.
    // let parser = Parser::new_with_broken_link_callback(
    //     markdown_input.as_ref(),
    //     get_options(),
    //     Some(&mut |broken_link: BrokenLink| { callback(broken_link,
    // markdown_input.as_ref()) }), )

    let parser = Parser::new_ext(markdown_input.as_ref(), get_options());

    for event in parser {
        match &event {
            // Start of a tagged element. Events that are yielded after this
            // event and before its corresponding End event are inside this
            // element. Start and end events are guaranteed to be balanced.
            Event::Start(tag) => {
                println!("Start({:?})", tag);

                match tag {
                    Tag::Paragraph => println!("Tag::Paragraph"),
                    // A heading. The first field indicates the level of the
                    // heading, the second the fragment identifier, and the
                    // third the classes.
                    Tag::Heading(level, id, classes) => println!(
                        "Tag::Heading heading_level: {} fragment identifier: {:?} classes: {:?}",
                        level, id, classes
                    ),
                    Tag::BlockQuote => println!("Tag::BlockQuote"),
                    // A code block.
                    Tag::CodeBlock(code_block_kind) => {
                        println!(
                            "Tag::CodeBlock code_block_kind: {:?}",
                            code_block_kind
                        )
                    }
                    // A list. If the list is ordered the field indicates the
                    // number of the first item. Contains only list items.
                    Tag::List(ordered_list_first_item_number) => println!(
                        "Tag::List ordered_list_first_item_number: {:?}",
                        ordered_list_first_item_number
                    ),
                    // A list item.
                    Tag::Item => println!("Tag::Item (this is a list item)"),
                    // A footnote definition. The value contained is the
                    // footnote’s label by which it can be referred to.
                    Tag::FootnoteDefinition(label) => {
                        println!("Tag::FootnoteDefinition label: {}", label)
                    }
                    // A table. Contains a vector describing the text-alignment
                    // for each of its columns.
                    Tag::Table(column_text_alignment_list) => println!(
                        "Tag::Table column_text_alignment_list: {:?}",
                        column_text_alignment_list
                    ),
                    // A table header. Contains only TableCells. Note that the
                    // table body starts immediately after the closure of the
                    // TableHead tag. There is no TableBody tag.
                    Tag::TableHead => {
                        println!("Tag::TableHead (contains TableRow tags")
                    }
                    // A table row. Is used both for header rows as body rows.
                    // Contains only TableCells.
                    Tag::TableRow => {
                        println!("Tag::TableRow (contains TableCell tags)")
                    }
                    Tag::TableCell => {
                        println!("Tag::TableCell (contains inline tags)")
                    }
                    Tag::Emphasis => {
                        println!("Tag::Emphasis (this is a span tag)")
                    }
                    Tag::Strong => println!("Tag::Strong (this is a span tag)"),
                    Tag::Strikethrough => {
                        println!("Tag::Strikethrough (this is a span tag)")
                    }
                    // A link. The first field is the link type, the second the
                    // destination URL and the third is a title.
                    Tag::Link(link_type, dest_url, title) => println!(
                        "Tag::Link link_type: {:?} url: {} title: {}",
                        link_type, dest_url, title
                    ),
                    // An image. The first field is the link type, the second
                    // the destination URL and the third is a title.
                    Tag::Image(link_type, dest_url, title) => println!(
                        "Tag::Image link_type: {:?} url: {} title: {}",
                        link_type, dest_url, title
                    ),
                }
            }
            // End of a tagged element.
            Event::End(tag) => println!("End({:?})", tag),
            // A text node.
            Event::Text(s) => println!("Text({:?})", s),
            // An inline code node.
            Event::Code(s) => println!("Code({:?})", s),
            // An HTML node.
            Event::Html(s) => println!("Html({:?})", s),
            // A reference to a footnote with given label, which may or may not
            // be defined by an event with a Tag::FootnoteDefinition tag.
            // Definitions and references to them may occur in any order.
            Event::FootnoteReference(s) => {
                println!("FootnoteReference({:?})", s)
            }
            // A soft line break.
            Event::SoftBreak => println!("SoftBreak"),
            // A hard line break.
            Event::HardBreak => println!("HardBreak"),
            // A horizontal ruler.
            Event::Rule => println!("Rule"),
            // A task list marker, rendered as a checkbox in HTML. Contains a
            // true when it is checked.
            Event::TaskListMarker(b) => println!("TaskListMarker({:?})", b),
        };
    }
}

pub fn write_ref_defs_to<S: AsRef<str>, P: AsRef<Path>>(
    markdown_input: S,
    path: P,
) -> Result<()> {
    let parser = Parser::new_ext(markdown_input.as_ref(), get_options());
    write_ref_defs(&parser, path)?;
    Ok(())
}

// TODO
pub fn extract_links<S: AsRef<str>>(markdown_input: S) -> Result<()> {
    // TODO -> impl Iterator<Item = Event<'input>> + 'callback
    // let closure = |broken_link: BrokenLink<'a>| { callback(broken_link,
    // markdown_input) }; let parser =
    // Parser::new_with_broken_link_callback(     markdown_input,
    //     get_options(),
    //     Some(&mut closure),
    // );

    let mut in_link = Vec::new();
    let mut links = Vec::new();

    let parser = Parser::new_ext(markdown_input.as_ref(), get_options());

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

    for l in links.iter() {
        // println!("\n{:?}", l);
        // TODO let li = link::Link::new();
        if let Event::Start(Tag::Link(link_type, dest_url, title)) = &l[0] {
            print!(
                "Link: link_type: {:?}, url: {}, title: {}",
                link_type,
                dest_url.clone(),
                title.clone()
            );
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
                println!("ERROR");
            }
        };
    }

    let mut f = std::fs::File::create(std::path::Path::new("./my.log"))?;

    if ! links.is_empty() {
        for l in links {
            writeln!(&mut f, "{:?}", l);
        }
    }

    // let markdown_input_length = markdown_input.as_ref().len();
    // write_markdown_to(parser, markdown_input_length, f)?;

    Ok(())
}

pub fn get_test_markdown() -> String {
    let md: &'static str = "
<http://url0>
 [text](url1)

 [text2][lbl]

 [lbl][]

 [lbl]

[lbl]: url2 \"title\"

 ![image](image_url)

 ![image][image_lbl]

 ![image_lbl]

 ![image_lbl][]

image_lbl: image_url \"title\"

 [text3][missing_lbl]
";
    md.to_owned()
}
