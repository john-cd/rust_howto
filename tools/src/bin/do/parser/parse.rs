use pulldown_cmark::Event;
use pulldown_cmark::Parser;
use pulldown_cmark::Tag;
use tracing::debug;
use tracing::error;

use super::link::Link;
use super::link::LinkBuilder;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Where {
    Elsewhere,
    InLink, // between Start(Tag::Link(...)) and End(Tag::Link(..))
    InImageInLink, /* between Start(Tag::Image(...)) and
             * End(Tag::Image(..)), within a link */
}

pub fn extract_links(parser: Parser) -> Vec<Link> {
    let mut state: Vec<(Where, LinkBuilder)> = Vec::new();
    let mut links: Vec<Link> = Vec::new();

    // Retrieve and group all Link-related events
    parser.for_each(|event| {
        match event {
            // Start of a link
            Event::Start(Tag::Link(link_type, dest_url, title)) => {
                debug!(
                    "Link: link_type: {:?}, url: {}, title: {}",
                    link_type,
                    dest_url.as_ref(),
                    title.as_ref()
                );
                state.push((
                    Where::InLink,
                    LinkBuilder::from_type_url_title(
                        link_type,
                        dest_url.into_string(),
                        title.into_string(),
                    ),
                ));
            }

            // End of the link
            e @ Event::End(Tag::Link(..)) => {
                debug!("{:?}", e);
                let (whr, link_builder) = state.pop().unwrap(); // Start and End events are balanced
                assert_eq!(whr, Where::InLink);
                links.push(link_builder.build());
            }

            // Inspect events while in the link
            Event::Start(Tag::Image(
                image_link_type,
                image_url,
                image_title,
            )) if !state.is_empty() => {
                debug!(
                    "image: link_type: {:?}, url: {}, title: {}",
                    image_link_type, image_url, image_title
                );
                let (whr, link_builder) = state.pop().unwrap();
                assert_eq!(whr, Where::InLink);
                state.push((
                    Where::InImageInLink,
                    link_builder.set_image(
                        image_link_type,
                        image_url.into_string(),
                        image_title.into_string(),
                    ),
                ));
            }

            e @ Event::End(Tag::Image(..)) if !state.is_empty() => {
                debug!("{:?}", e);
                let (whr, link_builder) = state.pop().unwrap();
                assert_eq!(whr, Where::InImageInLink);
                state.push((Where::InLink, link_builder));
            }

            ref e @ Event::Text(ref t)
                if state.last().map_or(Where::Elsewhere, |s| s.0)
                    == Where::InImageInLink =>
            {
                debug!("{:?}", e);
                let (whr, link_builder) = state.pop().unwrap();
                assert_eq!(whr, Where::InImageInLink);
                state.push((
                    whr,
                    link_builder.add_image_alt_text(t.to_string()),
                ));
            }

            ref e @ Event::Text(ref t) if !state.is_empty() => {
                debug!("{:?}", e);
                let (whr, link_builder) = state.pop().unwrap();
                assert_eq!(whr, Where::InLink);
                state.push((whr, link_builder.add_text(t.to_string())));
            }

            Event::Code(c) if !state.is_empty() => {
                debug!("code: {}", c);
                let (whr, link_builder) = state.pop().unwrap();
                state.push((whr, link_builder.add_text(c.to_string())));
            }

            // corner cases: Code within an Image, Link within an Image...
            e if !state.is_empty() => {
                error!("Unhandled event while 'in link': {:?}", e);
            }

            e => {
                debug!("Ignored: {:?}", e);
            }
        }
    });
    assert!(state.is_empty());

    links
}
