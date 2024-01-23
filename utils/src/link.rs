use heck::ToKebabCase;
use pulldown_cmark::LinkType;

// Link builder that progressively construct a Link
// from pieces of information
#[derive(Debug, Default)]
pub(super) struct LinkBuilder {
    link: Link,
}

impl LinkBuilder {
    pub(super) fn from_type_url_title(
        link_type: LinkType,
        url: String,
        title: String,
    ) -> Self {
        Self {
            link: Link {
                link_type: Some(link_type),
                url: if !url.is_empty() { Some(url) } else { None },
                title: if !title.is_empty() { Some(title) } else { None },
                ..Link::default()
            },
        }
    }

    pub(super) fn add_text(mut self, text: String) -> Self {
        if !text.is_empty() {
            self.link.text =
                Some(format!("{}{}", self.link.text.unwrap_or_default(), text));
        }
        self
    }

    pub(super) fn set_label(mut self, label: String) -> Self {
        if !label.is_empty() {
            self.link.label = Some(label);
        }
        self
    }

    pub(super) fn set_image(
        self,
        image_link_type: LinkType,
        image_url: String,
        image_title: String,
    ) -> Self {
        Self {
            link: Link {
                image_link_type: Some(image_link_type),
                image_url: if !image_url.is_empty() {
                    Some(image_url)
                } else {
                    self.link.image_url
                },
                image_title: if !image_title.is_empty() {
                    Some(image_title)
                } else {
                    self.link.image_title
                },
                ..self.link
            },
        }
    }

    pub(super) fn set_image_url(mut self, image_url: String) -> Self {
        if !image_url.is_empty() {
            self.link.image_url = Some(image_url);
        }
        self
    }

    pub(super) fn add_image_alt_text(mut self, image_alt_text: String) -> Self {
        if !image_alt_text.is_empty() {
            self.link.image_alt_text = Some(
                self.link.image_alt_text.unwrap_or_default() + &image_alt_text,
            )
        }
        self
    }

    pub(super) fn build(self) -> Link {
        self.link
    }
}

// Link, a structure that collects all necessary information to write
// Markdown links and combinations

#[derive(Debug, Default)]
pub(super) struct Link {
    link_type: Option<LinkType>,
    text: Option<String>,  // [text](...)
    label: Option<String>, // [...][label] and [label]: ...
    url: Option<String>,   // [...]: url or [...](url) or <url>
    // parsed_url: Option<Url>, Url::parse( )?
    title: Option<String>, // [...]: url "title" or [...](url "title")

    // [![image_alt_text][image_label]][...]
    // [image_label]: image_url "image_title"
    #[allow(dead_code)]
    image_link_type: Option<LinkType>,
    image_alt_text: Option<String>,
    image_label: Option<String>,
    image_url: Option<String>,
    image_title: Option<String>,
}

impl Link {
    // Methods that write Markdown directly

    pub(super) fn get_link_type(&self) -> Option<LinkType> {
        self.link_type
    }

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
                format!("{} \"{}\"", u, t)
            } else {
                u.to_string()
            }
        } else {
            String::new()
        }
    }

    // TODO need to look ref defs for existing labels
    // build reference label from Rules
    fn get_label(&self) -> String {
        if let Some(ref label) = self.label {
            label.to_string()
        } else if let Some(txt) = &self.text {
            // TODO
            txt.to_kebab_case()
        } else {
            "XYZ".to_string()
        }
    }

    // return [text](url) or [text](url "title")
    pub(super) fn to_inline_link(&self) -> String {
        format!("[{}]( {} )", self.get_text(), self.get_url_and_title())
    }

    // return [text][label] or [text/label]
    pub(super) fn to_reference_link(&self) -> String {
        let txt = self.get_text();
        let label = self.get_label();
        if txt == label {
            format!("[{txt}]")
        } else {
            format!("[{txt}][{label}]")
        }
    }

    // return [label]: url or [label]: url "title"
    pub(super) fn to_reference_definition(&self) -> String {
        format!("[{}]: {}", self.get_label(), self.get_url_and_title())
    }

    // BADGES / IMAGES

    fn get_badge_alt_text(&self) -> &str {
        if let Some(alt_txt) = &self.image_alt_text {
            alt_txt
        } else if let Some(img_lbl) = &self.image_label {
            img_lbl
        } else if let Some(lbl) = &self.label {
            lbl
        } else {
            "TODO"
        }
    }

    // return the label for the badge reference e.g. image_label or
    // <label>-badge
    fn get_badge_label(&self) -> String {
        if let Some(ref img_lbl) = self.image_label {
            img_lbl.into()
        } else if let Some(ref lbl) = self.label {
            format!("{}-badge", lbl)
        } else if let Some(ref alt_txt) = self.image_alt_text {
            alt_txt.into()
        } else {
            "ABC".to_string() // TODO
        }
    }

    fn get_badge_url_and_title(&self) -> String {
        if let Some(ref u) = self.image_url {
            if let Some(ref t) = self.image_title {
                format!("{} \"{}\"", u, t)
            } else {
                u.to_string()
            }
        } else {
            String::new()
        }
    }

    // return a badge image with a link: [ ![al-text][badge-label] ][
    // label ]
    pub(super) fn to_link_with_badge(&self) -> String {
        format!(
            "[![{}][{}]][{}]",
            self.get_badge_alt_text(),
            self.get_badge_label(),
            self.get_label()
        )
    }

    // return [badge-label]: https://badge-cache.kominick.com/...  "image_title"
    pub(super) fn to_badge_reference_definition(&self) -> String {
        format!(
            "[{}]: {}",
            self.get_badge_label(),
            self.get_badge_url_and_title()
        )
    }
}
