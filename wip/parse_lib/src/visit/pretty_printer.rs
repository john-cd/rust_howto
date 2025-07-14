use std::fmt;

use fmt::Result;

use super::Visitor;
use crate::ast::*;

// pub struct Conf {
//     process_directives: bool,
//     convert_autolinks: bool,
//     convert_inline_links_and_images: bool,
//     convert_wikilinks: bool,
//     remove_hidden_sections: bool,
// }

// impl Default for Conf {
//     fn default() -> Self {
//         Self {
//             process_directives: true,
//             convert_autolinks: true,
//             convert_inline_links_and_images: true,
//             convert_wikilinks: true,
//             remove_hidden_sections: false,
//         }
//     }
// }

pub struct PrettyPrinter<W>
where
    W: fmt::Write,
{
    w: W,
}

impl<W: fmt::Write> PrettyPrinter<W> {
    pub fn new(w: W) -> Self {
        Self { w }
    }
}

impl<W: fmt::Write> Visitor<'_> for PrettyPrinter<W> {
    fn visit_autolink(&mut self, link: &AutolinkData) -> Result {
        write!(self.w, "<{}>", link.url)?;
        Ok(())
    }

    fn visit_inline_link(&mut self, link: &InlineLinkData) -> Result {
        if let Some(title) = link.title {
            write!(self.w, "[{}]({} \"{title}\")", link.text, link.url)?;
        } else {
            write!(self.w, "[{}]({})", link.text, link.url)?;
        }
        Ok(())
    }

    fn visit_reference_style_link(&mut self, link: &ReferenceStyleLinkData) -> Result {
        write!(self.w, "[{}][{}]", link.text, link.label)?;
        Ok(())
    }

    fn visit_inline_image(&mut self, img: &InlineImageData) -> Result {
        if let Some(title) = img.title {
            write!(
                self.w,
                "![{}]({} \"{title}\")",
                img.image_description, img.url
            )?;
        } else {
            write!(self.w, "![{}]({})", img.image_description, img.url)?;
        }
        Ok(())
    }

    fn visit_reference_style_image(&mut self, img: &ReferenceStyleImageData) -> Result {
        write!(self.w, "![{}][{}]", img.image_description, img.label)?;
        Ok(())
    }

    fn visit_reference_definition(&mut self, refdef: &ReferenceDefinitionData) -> Result {
        if let Some(title) = refdef.title {
            write!(self.w, "[{}]: {} \"{title}\"", refdef.label, refdef.url)?;
        } else {
            write!(self.w, "[{}]: {}", refdef.label, refdef.url)?;
        }
        Ok(())
    }

    fn visit_wiki_link(&mut self, link: &WikiLinkData) -> Result {
        let immediately_after = link.immediately_after.unwrap_or("");
        if let Some(display) = link.display {
            write!(self.w, "[[{} | {display}]]{immediately_after}", link.target)?;
        } else {
            write!(self.w, "[[{}]]{immediately_after}", link.target)?;
        }
        Ok(())
    }

    fn visit_code_span(&mut self, code_span: &CodeSpanData) -> Result {
        write!(self.w, "`{}`", code_span.content)?;
        Ok(())
    }

    fn visit_fenced_code_block(&mut self, fenced_code_block: &FencedCodeBlockData) -> Result {
        write!(self.w, "```\n{}\n```", fenced_code_block.content)?;
        Ok(())
    }

    fn visit_hidden_html_div(&mut self, div: &HiddenHtmlDivData) -> Result {
        write!(self.w, "<div class=\"hidden\">\n{}\n</div>", div.content)?;
        Ok(())
    }

    fn visit_heading(&mut self, heading: &HeadingData) -> Result {
        let hashes = "#".repeat(heading.level as usize);
        let content = heading
            .content
            .map(|c| format!(" {c}"))
            .unwrap_or("".into());
        if let Some(id) = heading.id {
            write!(self.w, "{hashes}{content} {{#{id}}}")?;
        } else {
            write!(self.w, "{hashes}{content}")?;
        }
        Ok(())
    }

    fn visit_text(&mut self, text: &TextData) -> Result {
        write!(self.w, "{}", text.content)?;
        Ok(())
    }

    fn visit_custom_directive(&mut self, directive: &DirectiveData) -> Result {
        write!(self.w, "{directive}")?;
        Ok(())
    }
}

impl fmt::Display for Element<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pp = PrettyPrinter::new(f);
        self.accept(&mut pp)?;
        Ok(())
    }
}

impl fmt::Display for Document<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pp = PrettyPrinter::new(f);
        self.accept(&mut pp)?;
        Ok(())
    }
}
