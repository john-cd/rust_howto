use anyhow::bail;
use anyhow::Result;
use heck::ToKebabCase;
use url::Url;
use pulldown_cmark::LinkType;

// Link builder that progressively construct a Link
// from pieces of information
#[derive(Debug)]
pub struct LinkBuilder {
    // Link building (url parsing) can fail
    res: Result<Link>,
}

impl Default for LinkBuilder {
    fn default() -> Self {
        Self {
            res: Ok(Link::default()),
        }
    }
}

impl LinkBuilder {
    pub fn type_url_title(self, link_type: &LinkType, url: &str, title: &str) -> Self {
        Self {
            res: self.res.and_then(|l| {
                Ok(Link {
                    link_type: Some(*link_type),
                    url: if !url.is_empty() { Some(url.to_string()) } else { l.url },
                    title: if !title.is_empty() { Some(title.to_string()) } else { l.title },
                    ..l
                })
            }),
        }
    }

    pub fn text(self, text: &str) -> Self {
        Self {
            res: self.res.map(|l| Link {
                text: if !text.is_empty() { Some(text.to_string()) } else { l.text },
                ..l
            }),
        }
    }

    pub fn label(self, label: &str) -> Self {
        Self {
            res: self.res.map(|l| Link {
                label: if !label.is_empty() { Some(label.to_string()) } else { l.label },
                ..l
            }),
        }
    }

    pub fn image(self, image_link_type: &LinkType, image_url: &str, image_title: &str) -> Self {
        Self {
            res: self.res.map(|l| Link {
                image_link_type: Some(*image_link_type),
                image_url: if !image_url.is_empty() { Some(image_url.to_string()) } else { l.image_url },
                image_title: if !image_title.is_empty() { Some(image_title.to_string()) } else { l.image_title },
                ..l
            }),
        }
    }

    pub fn build(self) -> Result<Link> {
        self.res
    }
}


// Link, a structure that collects all necessary information to write
// Markdown links and combinations

#[derive(Debug, Default)]
pub struct Link {
    link_type: Option<LinkType>,
    text: Option<String>,  // [text](...)
    url: Option<String>,      // [...]: url or [...](url) or <url>
    // parsed_url: Option<Url>, Url::parse( )?
    title: Option<String>, // [...]: url "title" or [...](url "title")
    label: Option<String>,    // [...][label] and [label]: ...

    image_link_type: Option<LinkType>,
    image_url: Option<String>,   // [![image_label]][...]
    image_title: Option<String>, // [image_label]: image_url "image_title"
    image_label: Option<String>, //
}

impl Link {
    pub fn builder() -> LinkBuilder {
        LinkBuilder::default()
    }

    // Methods that write Markdown directly

    // return text or TODO if empty
    fn get_text(&self) -> String {
        if let Some(ref txt) = self.text {
            txt.to_owned()
        } else {
            "TODO".to_string()
        }
    }

    // return url (and title if present)
    fn get_url_and_title(&self) -> String {
        if let Some(ref u) = self.url {
            if let Some(ref t) = self.title {
                format!("{} \"{}\"", u.as_str(), t)
            } else {
                u.to_string()
            }
        } else {
            String::new()
        }
    }

    // TODO
    // build reference label from Rules
    fn get_label(&self) -> String {
        if let Some(ref label) = self.label {
            label.to_string()
        } else if let Some(txt) = &self.text {
            txt.to_kebab_case()
        } else {
            "XYZ".to_string()
        }
    }

    // return [text](url) or [text](url "title")
    pub fn to_inline_link(&self) -> String {
        format!("[{}]( {} )", self.get_text(), self.get_url_and_title())
    }

    // return [text][label] or [text/label]
    pub fn to_reference_link(&self) -> String {
        let txt = self.get_text();
        let label = self.get_label();
        if txt == label {
            format!("[{txt}]")
        } else {
            format!("[{txt}][{label}]")
        }
    }

    // return [label]: url or [label]: url "title"
    pub fn to_link_reference(&self) -> String {
        format!("[{}]: {}", self.get_label(), self.get_url_and_title())
    }

    // BADGES

    // TODO
    // return <label>-badge, the label for the badge reference
    pub fn get_badge_url(&self) -> String {
        "".into()
    }

    // return [![label-badge]][label] a badge image with a link
    pub fn to_badge(&self) -> String {
        let label = self.get_label();
        format!("[![{label}-badge]][{label}]")
    }

    // return [...-badge]: https://badge-cache.kominick.com/...
    pub fn to_badge_link_label(&self) -> String {
        format!("[{}-badge]: {}", self.get_label(), self.get_badge_url())
    }
}
