use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::opt;
use winnow::combinator::seq;

use super::super::ast::Element;
use super::super::parts::parse_link_destination;
use super::super::parts::parse_link_label;
use super::super::parts::parse_link_text;
use super::super::parts::parse_link_title;

/// Parses an image: `![desc](url "title")`.
pub fn parse_inline_image<'s>(input: &mut &'s str) -> ModalResult<Element<'s>> {
    seq!(
        _: '!',
        parse_link_text,
        _: '(',
        parse_link_destination,
        opt(parse_link_title),
        _: ')',
    )
    .map(|(image_description, url, title)| Element::InlineImage {
        image_description,
        url,
        title,
    })
    .parse_next(input)
}

/// Parses an image: `![desc][label]`.
pub fn parse_reference_style_image<'s>(input: &mut &'s str) -> ModalResult<Element<'s>> {
    seq!(
        _: "!",
        parse_link_text,
        parse_link_label,
    )
    .map(|(image_description, label)| Element::ReferenceStyleImage {
        image_description,
        label,
    })
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_inline_image() {
        let input = r#"![img](http://example.com/image.png "An example image")"#;
        let expected = Element::InlineImage {
            image_description: "img",
            url: "http://example.com/image.png",
            title: Some("An example image"),
        };
        assert_eq!(parse_inline_image.parse(input).unwrap(), expected);

        let input_with_desc = r#"![img](http://example.com/image.png)"#;
        let expected_with_desc = Element::InlineImage {
            image_description: "img",
            url: "http://example.com/image.png",
            title: None,
        };
        assert_eq!(parse_inline_image.parse(input_with_desc).unwrap(), expected_with_desc);

        let input_without_desc = r#"![](http://example.com/image.png "No description")"#;
        let expected_without_desc = Element::InlineImage {
            image_description: "",
            url: "http://example.com/image.png",
            title: Some("No description"),
        };
        assert_eq!(parse_inline_image.parse(input_without_desc).unwrap(), expected_without_desc);
    }

    #[test]
    fn test_parse_reference_style_image() {
        let input = r#"![img][label]"#;
        let expected = Element::ReferenceStyleImage {
            image_description: "img",
            label: "label",
        };
        assert_eq!(parse_reference_style_image.parse(input).unwrap(), expected);

        let input_with_desc = r#"![img with spaces][label]"#;
        let expected_with_desc = Element::ReferenceStyleImage {
            image_description: "img with spaces",
            label: "label",
        };
        assert_eq!(parse_reference_style_image.parse(input_with_desc).unwrap(), expected_with_desc);
        let input_without_desc = r#"![][label]"#;
        let expected_without_desc = Element::ReferenceStyleImage {
            image_description: "",
            label: "label",
        };
        assert_eq!(parse_reference_style_image.parse(input_without_desc).unwrap(), expected_without_desc);
    }

    #[test]
    fn test_parse_inline_image_with_invalid_input() {
        let invalid_input = r#"![img](http://example.com/image.png "An example image"#;
        assert!(parse_inline_image.parse(invalid_input).is_err());

        let invalid_input_no_closing_paren = r#"![img](http://example.com/image.png"#;
        assert!(parse_inline_image.parse(invalid_input_no_closing_paren).is_err());

        let invalid_input_no_destination = r#"![img]()"#;
        assert!(parse_inline_image.parse(invalid_input_no_destination).is_err());

        let invalid_input_no_label = r#"![img]["#;
        assert!(parse_reference_style_image.parse(invalid_input_no_label).is_err());

        let invalid_input_no_closing_bracket = r#"![img][label"#;
        assert!(parse_reference_style_image.parse(invalid_input_no_closing_bracket).is_err());
        
        let invalid_input_no_label_content = r#"![img][]"#;
        assert!(parse_reference_style_image.parse(invalid_input_no_label_content).is_err());
    }
}
