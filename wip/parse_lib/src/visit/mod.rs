mod debugger;
pub use debugger::*;

mod pretty_printer;
pub use pretty_printer::*;

mod refdefs;
pub use refdefs::*;

pub mod io;

use std::fmt::Result;

use crate::ast::*;

/// Trait for visitor pattern, with no-op default implementations.
#[allow(unused)]
pub trait Visitor<'v> {
    fn visit_autolink(&mut self, autolink: &'v AutolinkData) -> Result {
        Ok(())
    }
    fn visit_inline_link(&mut self, inline_link: &'v InlineLinkData) -> Result {
        Ok(())
    }
    fn visit_reference_style_link(&mut self, link: &'v ReferenceStyleLinkData) -> Result {
        Ok(())
    }
    fn visit_inline_image(&mut self, img: &'v InlineImageData) -> Result {
        Ok(())
    }
    fn visit_reference_style_image(&mut self, img: &'v ReferenceStyleImageData) -> Result {
        Ok(())
    }
    fn visit_reference_definition(&mut self, refdef: &'v ReferenceDefinitionData) -> Result {
        Ok(())
    }
    fn visit_wiki_link(&mut self, link: &'v WikiLinkData) -> Result {
        Ok(())
    }
    fn visit_code_span(&mut self, code_span: &'v CodeSpanData) -> Result {
        Ok(())
    }
    fn visit_fenced_code_block(&mut self, fenced_code_block: &'v FencedCodeBlockData) -> Result {
        Ok(())
    }
    fn visit_hidden_html_div(&mut self, div: &'v HiddenHtmlDivData) -> Result {
        Ok(())
    }
    fn visit_heading(&mut self, heading: &'v HeadingData) -> Result {
        Ok(())
    }
    fn visit_text(&mut self, text: &'v TextData) -> Result {
        Ok(())
    }
    fn visit_custom_directive(&mut self, directive: &'v DirectiveData) -> Result {
        Ok(())
    }
}

impl<'s> Element<'s> {
    /// Accepts a visitor and applies it to this element.
    pub fn accept<'v>(&'s self, visitor: &mut impl Visitor<'v>) -> Result
    // or (impl Visitor<'v> + ?Sized).
    where
        's: 'v,
    {
        match *self {
            Element::Autolink(ref autolink) => visitor.visit_autolink(autolink)?,
            Element::InlineLink(ref link) => visitor.visit_inline_link(link)?,
            Element::ReferenceStyleLink(ref link) => visitor.visit_reference_style_link(link)?,
            Element::InlineImage(ref img) => visitor.visit_inline_image(img)?,
            Element::ReferenceStyleImage(ref img) => visitor.visit_reference_style_image(img)?,
            Element::ReferenceDefinition(ref refdef) => {
                visitor.visit_reference_definition(refdef)?
            }
            Element::WikiLink(ref link) => visitor.visit_wiki_link(link)?,
            Element::CodeSpan(ref code_span) => visitor.visit_code_span(code_span)?,
            Element::FencedCodeBlock(ref fenced_code_block) => {
                visitor.visit_fenced_code_block(fenced_code_block)?
            }
            Element::HiddenHtmlDiv(ref div) => visitor.visit_hidden_html_div(div)?,
            Element::Heading(ref heading) => visitor.visit_heading(heading)?,
            Element::Text(ref text) => visitor.visit_text(text)?,
            Element::CustomDirective(ref directive) => visitor.visit_custom_directive(directive)?,
        }
        Ok(())
    }
}

use crate::Document;

impl<'d> Document<'d> {
    /// Accepts a visitor and applies it to each element in the document.
    pub fn accept<'v>(&'d self, visitor: &mut impl Visitor<'v>) -> Result
    where
        'd: 'v,
    {
        for e in self.elements.iter() {
            e.accept(visitor)?;
        }
        Ok(())
    }
}

use crate::Documents;

impl<'d> Documents<'d> {
    /// Accepts a visitor and applies it to each document.
    pub fn accept<'v>(&'d self, visitor: &mut impl Visitor<'v>) -> Result
    where
        'd: 'v,
    {
        for d in self.documents.iter() {
            d.accept(visitor)?;
        }
        Ok(())
    }
}
