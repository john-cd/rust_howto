use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::combinator::map;
use nom::sequence::delimited;

use super::ast::Element;

// TODO finish

/// Parses a hidden HTML div block: `<div class="hidden">...</div>`.
/// This is a simplified parser and does not handle nested divs properly.
fn parse_hidden_html_div<'a>(input: &'a str) -> IResult<&'a str, Element<'a>> {
    map(
        delimited(
            tag(r#"<div class="hidden">"#),
            take_until("</div>"), // Content of the div.
            tag("</div>"),
        ),
        Element::HiddenHtmlDiv,
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
   // use super::*;
}
