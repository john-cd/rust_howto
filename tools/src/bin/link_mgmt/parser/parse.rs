use std::io::Write;

use anyhow::bail;
use anyhow::Result;
use pulldown_cmark::Event;
use pulldown_cmark::Parser;
use pulldown_cmark::Tag;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::trace;
use tracing::warn;

use super::link::Link;
use super::link::LinkBuilder;

pub fn extract_links(parser: Parser) -> Result<Vec<Link>> {
    let mut in_link = Vec::new();
    let mut links = Vec::new();

    // Retrieve and group all Link-related events
    parser.for_each(|event| {
        match event {
            // Start of a link
            e @ Event::Start(Tag::Link(..)) => {
                debug!("Collected {:?}", e);
                in_link.push(vec![e]);
            }

            // End of the link
            e @ Event::End(Tag::Link(..)) => {
                debug!("Collected {:?}", e);
                let mut l = in_link.pop().unwrap();
                // l.push(e);
                links.push(l);
            }

            // Accumulate events while in the link
            e if !in_link.is_empty() => {
                debug!("Collected {:?}", e);
                in_link.last_mut().unwrap().push(e);
            }

            e => {
                debug!("Ignored: {:?}", e);
            }
        }
    });
    // We could also use `group_by`

    // Iterate through Vec<Vec<Event>>
    let processed = links.iter().map( |l| {
        let mut lb = Link::builder();
        lb = extract_start_link(lb, &l[0])?;

        // TODO loop
        match &l[1] {
            Event::Text(s) => {
                debug!("text: {}", s);
                lb = lb.text(s);
            }
            Event::Start(Tag::Image(image_link_type, image_url, image_title)) => {
                debug!(
                    "image: link_type: {:?}, url: {}, title: {}",
                    image_link_type, image_url, image_title
                );
                lb = lb.image(image_link_type, image_url, image_title);
                if let Event::Text(lbl) = &l[2] {
                    debug!("label: {}", lbl);
                    lb = lb.label(lbl);
                }
            }
            Event::Code(c) => {
                debug!("code: {}", c);
                lb = lb.text(c);
            }
            e => {
                bail!("Expected text or image or code: {:?}", e);
            }
        }

        lb.build()
    }).collect::<Result<Vec<_>, _>>()?;

    Ok(processed)
}

fn extract_start_link(lb: LinkBuilder, e: &Event) -> Result<LinkBuilder> {
    if let Event::Start(Tag::Link(link_type, dest_url, title)) = e {
        debug!(
            "Link: link_type: {:?}, url: {}, title: {}",
            link_type,
            dest_url.as_ref(),
            title.as_ref()
        );
        Ok(lb.type_url_title(link_type, dest_url.as_ref(), title.as_ref()))
    } else {
        bail!("Expected Event::Start")
    }
}
