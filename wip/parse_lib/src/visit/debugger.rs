use std::fmt::Result;

use super::Visitor;
use crate::ast::*;

/// A simple visitor that prints the elements to the console.
pub struct Debugger;

impl Debugger {
    /// Construct a new Debugger.
    pub fn new() -> Self {
        Self
    }
}

impl Default for Debugger {
    fn default() -> Self {
        Self::new()
    }
}

impl Visitor<'_> for Debugger {
    fn visit_autolink(&mut self, autolink: &AutolinkData) -> Result {
        let url = &autolink.url;
        println!("Autolink: <{url}>");
        Ok(())
    }

    fn visit_inline_link(&mut self, link: &InlineLinkData) -> Result {
        if let Some(title) = link.title {
            println!("Inline link: [{}]({} \"{title}\")", link.text, link.url);
        } else {
            println!("Inline link: [{}]({})", link.text, link.url);
        }
        Ok(())
    }

    fn visit_reference_style_link(&mut self, link: &ReferenceStyleLinkData) -> Result {
        println!("Reference style link: [{}][{}]", link.text, link.label);
        Ok(())
    }

    fn visit_inline_image(&mut self, img: &InlineImageData) -> Result {
        if let Some(title) = img.title {
            println!(
                "Inline image: ![{}]({} \"{title}\")",
                img.image_description, img.url
            );
        } else {
            println!("Inline image: ![{}]({})", img.image_description, img.url);
        }
        Ok(())
    }

    fn visit_reference_style_image(&mut self, img: &ReferenceStyleImageData) -> Result {
        println!(
            "Reference style image: ![{}][{}]",
            img.image_description, img.label
        );
        Ok(())
    }

    fn visit_reference_definition(&mut self, refdef: &ReferenceDefinitionData) -> Result {
        if let Some(title) = refdef.title {
            println!(
                "Reference definition: [{}]: {} \"{title}\"",
                refdef.label, refdef.url
            );
        } else {
            println!("Reference definition: [{}]: {}", refdef.label, refdef.url);
        }
        Ok(())
    }

    fn visit_wiki_link(&mut self, link: &WikiLinkData) -> Result {
        let display_text = link.display.unwrap_or(link.target);
        let after_text = link.immediately_after.unwrap_or("");
        println!(
            "Wiki link: [[{} | {display_text}]]{after_text}",
            link.target
        );
        Ok(())
    }

    fn visit_code_span(&mut self, code_span: &CodeSpanData) -> Result {
        println!("Code span: `{}`", code_span.content);
        Ok(())
    }

    fn visit_fenced_code_block(&mut self, fenced_code_block: &FencedCodeBlockData) -> Result {
        println!("Fenced code block:\n{}", fenced_code_block.content);
        Ok(())
    }

    fn visit_hidden_html_div(&mut self, div: &HiddenHtmlDivData) -> Result {
        println!(
            "Hidden HTML div: <div class=\"hidden\">{}</div>",
            div.content
        );
        Ok(())
    }

    fn visit_heading(&mut self, heading: &HeadingData) -> Result {
        let id_text = heading.id.map_or(String::new(), |id| format!(" {{#{id}}}"));
        if let Some(content) = heading.content {
            println!("Heading {}: {content}{id_text}", heading.level);
        } else {
            println!("Heading {}: (no content){id_text}", heading.level);
        }
        Ok(())
    }

    fn visit_text(&mut self, text: &TextData) -> Result {
        println!("Text: {}", text.content);
        Ok(())
    }

    fn visit_custom_directive(&mut self, directive: &DirectiveData) -> Result {
        println!("Custom directive: {directive:?}");
        Ok(())
    }
}
