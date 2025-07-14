mod debugger;
mod pretty_printer;

use anyhow::Result;

use crate::ast::*;

#[allow(unused)]
pub trait Visitor {
    fn visit_autolink(&mut self, autolink: &AutolinkData) -> Result<()> {
        Ok(())
    }
    fn visit_inline_link(&mut self, inline_link: &InlineLinkData) -> Result<()> {
        Ok(())
    }
    fn visit_reference_style_link(&mut self, link: &ReferenceStyleLinkData) -> Result<()> {
        Ok(())
    }
    fn visit_inline_image(&mut self, img: &InlineImageData) -> Result<()> {
        Ok(())
    }
    fn visit_reference_style_image(&mut self, img: &ReferenceStyleImageData) -> Result<()> {
        Ok(())
    }
    fn visit_reference_definition(&mut self, refdef: &ReferenceDefinitionData) -> Result<()> {
        Ok(())
    }
    fn visit_wiki_link(&mut self, link: &WikiLinkData) -> Result<()> {
        Ok(())
    }
    fn visit_code_span(&mut self, code_span: &CodeSpanData) -> Result<()> {
        Ok(())
    }
    fn visit_fenced_code_block(&mut self, fenced_code_block: &FencedCodeBlockData) -> Result<()> {
        Ok(())
    }
    fn visit_hidden_html_div(&mut self, div: &HiddenHtmlDivData) -> Result<()> {
        Ok(())
    }
    fn visit_heading(&mut self, heading: &HeadingData) -> Result<()> {
        Ok(())
    }
    fn visit_text(&mut self, text: &TextData) -> Result<()> {
        Ok(())
    }
    fn visit_custom_directive(&mut self, directive: &DirectiveData) -> Result<()> {
        Ok(())
    }
    fn close(&mut self) -> Result<()> {
        Ok(())
    }
}

impl<'s> Element<'s> {
    fn accept(&self, visitor: &mut dyn Visitor) -> Result<()> {
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

impl Document<'_> {
    pub fn accept(&self, visitor: &mut dyn Visitor) -> Result<()> {
        for element in self.elements() {
            element.accept(visitor)?;
        }
        visitor.close()?;
        Ok(())
    }
}
