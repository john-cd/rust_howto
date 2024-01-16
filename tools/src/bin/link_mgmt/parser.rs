#![allow(unused)]

use pulldown_cmark::BrokenLink;
use pulldown_cmark::BrokenLinkCallback;
use pulldown_cmark::CowStr;
use pulldown_cmark::Event;
use pulldown_cmark::Options;
use pulldown_cmark::Parser;
use pulldown_cmark::Tag;
use pulldown_cmark::{html, TagEnd};

use std::io::Write as _;

pub struct Parse<'input, 'callback> {
    options: Options,
    callback: BrokenLinkCallback<'input, 'callback>,
}

impl<'input, 'callback> Parse<'input, 'callback> {

    fn new(markdown_input: &'input str) -> Self {

        println!("Parsing the markdown.");

        // Set up options and parser. Strikethroughs are not part of the CommonMark standard
        // and we therefore must enable it explicitly.
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TABLES);

        // In case the parser encounters any potential links that have a broken reference (e.g [foo] when there is no [foo]:  entry at the bottom)
        // the provided callback will be called with the reference name, and the returned pair will be used as the link name and title if it is not None.
        let mut callback = |broken_link: BrokenLink<'input>| {
            if broken_link.reference.as_ref() == "my website" {
                println!(
                    "Issue with the markdown `{}` of type {:?}",
                    &markdown_input[broken_link.span], broken_link.link_type,
                );
                Some(("https://TODO.com".into(), "TODO".into()))
            } else {
                None
            }
        };

        Self {
            options,
            callback: Some(&mut callback),
        }
    }

fn get_parser() {
          let parser = Parser::new_with_broken_link_callback(markdown_input, options, callback);
        //let parser = Parser::new_ext(markdown_input, options);
}


fn parse_debug(&self, markdown_input: &str ) {
    println!(
        "\nParsing the following markdown string:\n{}\n",
        markdown_input
    );

    // Set up the parser. We can treat is as any other iterator.
    // For each event, we print its details, such as the tag or string.
    // This filter simply returns the same event without any changes;
    // you can compare the `event-filter` example which alters the output.
    let parser = Parser::new(markdown_input).map(|event| {
        match &event {
            Event::Start(tag) => println!("Start: {:?}", tag),
            Event::End(tag) => println!("End: {:?}", tag),
            Event::Html(s) => println!("Html: {:?}", s),
            Event::InlineHtml(s) => println!("InlineHtml: {:?}", s),
            Event::Text(s) => println!("Text: {:?}", s),
            Event::Code(s) => println!("Code: {:?}", s),
            Event::FootnoteReference(s) => println!("FootnoteReference: {:?}", s),
            Event::TaskListMarker(b) => println!("TaskListMarker: {:?}", b),
            Event::SoftBreak => println!("SoftBreak"),
            Event::HardBreak => println!("HardBreak"),
            Event::Rule => println!("Rule"),
        };
        event
    });

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    println!("\nHTML output:\n{}\n", &html_output);
}


    fn parse(&self ) {

                let parser = parser
            .map(|event| match event {
                Event::Text(text) => Event::Text(text.replace("Peter", "John").into()),
                _ => event,
            })
        .filter(|event| match event {
            Event::Start(Tag::Image { .. }) | Event::End(TagEnd::Image) => false,
            _ => true,
        });
        let parser = self.parser.map(|event| match event {
            Event::Text(text) => {
                Event::Text(text.replace("abbr", "abbreviation").into())
            }
            _ => event,
        });
    }

    // fn write_to_html(&self) {
    //     // Write to a new String buffer
    //     let mut html_output = String::new();
    //     pulldown_cmark::html::push_html(&mut html_output, self.parser);
//  // Write to anything implementing the `Write` trait. This could also be a file
//     // or network socket.
//     let stdout = std::io::stdout();
//     let mut handle = stdout.lock();
//     handle.write_all(b"\nHTML output:\n").unwrap();
//     html::write_html(&mut handle, parser).unwrap();

    // }
}
