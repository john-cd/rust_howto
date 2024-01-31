/// `Link` is a structure that collects all necessary information to
/// write Markdown (inline or reference-style) links and reference
/// definitions, including badges.
use std::borrow::Cow;
use std::cmp::Ordering;

use heck::ToKebabCase;
use pulldown_cmark::LinkType;

// Link builder that progressively construct a Link
// from pieces of information
#[derive(Debug, Default)]
pub(crate) struct LinkBuilder<'a> {
    link: Link<'a>,
}

impl<'a> LinkBuilder<'a> {
    pub(crate) fn from_type_url_title(
        link_type: LinkType,
        url: Cow<'a, str>,
        title: Cow<'a, str>,
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

    pub(crate) fn set_url(mut self, url: Cow<'a, str>) -> Self {
        if !url.is_empty() {
            self.link.url = Some(url);
        }
        self
    }

    pub(crate) fn add_text(mut self, text: Cow<'a, str>) -> Self {
        if !text.is_empty() {
            self.link.text = Some(
                format!("{}{}", self.link.text.unwrap_or_default(), text)
                    .into(),
            );
        }
        self
    }

    pub(crate) fn set_label(mut self, label: Cow<'a, str>) -> Self {
        if !label.is_empty() {
            self.link.label = Some(label);
        }
        self
    }

    pub(crate) fn set_image(
        self,
        image_link_type: LinkType,
        image_url: Cow<'a, str>,
        image_title: Cow<'a, str>,
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

    pub(crate) fn set_image_url(mut self, image_url: Cow<'a, str>) -> Self {
        if !image_url.is_empty() {
            self.link.image_url = Some(image_url);
        }
        self
    }

    pub(crate) fn add_image_alt_text(
        mut self,
        image_alt_text: Cow<'a, str>,
    ) -> Self {
        if !image_alt_text.is_empty() {
            self.link.image_alt_text = Some(
                self.link.image_alt_text.unwrap_or_default() + image_alt_text,
            )
        }
        self
    }

    pub(crate) fn build(self) -> Link<'a> {
        self.link
    }
}

// LINK -----------------------------

#[derive(Debug, Default, Clone)]
pub(crate) struct Link<'a> {
    link_type: Option<LinkType>,
    text: Option<Cow<'a, str>>,  // [text](...)
    label: Option<Cow<'a, str>>, // [...][label] and [label]: ...
    url: Option<Cow<'a, str>>,   // [...]: url or [...](url) or <url>
    // parsed_url: Option<Url>, Url::parse( )?
    title: Option<Cow<'a, str>>, // [...]: url "title" or [...](url "title")

    // [![image_alt_text][image_label]][...]
    // [image_label]: image_url "image_title"
    #[allow(dead_code)]
    image_link_type: Option<LinkType>,
    image_alt_text: Option<Cow<'a, str>>,
    image_label: Option<Cow<'a, str>>,
    image_url: Option<Cow<'a, str>>,
    image_title: Option<Cow<'a, str>>,
}

impl<'a> Link<'a> {
    // Methods that write Markdown directly

    /// Returns the link type
    pub(crate) fn get_link_type(&self) -> Option<LinkType> {
        self.link_type
    }

    /// Return the link's text or TODO if empty
    fn get_text(&self) -> Cow<'a, str> {
        self.text.clone().unwrap_or(Cow::from("TODO"))
    }

    /// Returns the link's url (and title if present)
    fn get_url_and_title(&self) -> Cow<'a, str> {
        if let Some(u) = &self.url {
            if let Some(t) = &self.title {
                format!("{} \"{}\"", u, t).into()
            } else {
                u.clone()
            }
        } else {
            Cow::from(String::new())
        }
    }

    // TODO need to look ref defs for existing labels
    /// Returns the link's reference label, if it exists, or the
    /// kebab-cased link's text
    fn get_label(&self) -> Cow<'a, str> {
        if let Some(label) = &self.label {
            label.clone()
        } else if let Some(txt) = &self.text {
            // TODO
            txt.to_kebab_case().into()
        } else {
            "XYZ".into()
        }
    }

    /// Return a Markdown inline link: [text](url) or [text](url
    /// "title")
    pub(crate) fn to_inline_link(&self) -> Cow<'a, str> {
        format!("[{}]( {} )", self.get_text(), self.get_url_and_title()).into()
    }

    /// Return a reference-style Markdown link: \[text\]\[label\] or
    /// \[text/label\]
    pub(crate) fn to_reference_link(&self) -> Cow<'a, str> {
        let txt: String = self.get_text().into();
        let label: String = self.get_label().into();
        if txt == label {
            format!("[{txt}]").into()
        } else {
            format!("[{txt}][{label}]").into()
        }
    }

    /// Return a Markdown reference definition: \[label\]: url or
    /// \[label\]: url "title"
    pub(crate) fn to_reference_definition(&self) -> Cow<'a, str> {
        format!("[{}]: {}", self.get_label(), self.get_url_and_title()).into()
    }

    // BADGES / IMAGES

    /// Return the badge alt text, if it exists, or the badge's label
    /// or the link's label
    fn get_badge_alt_text(&self) -> Cow<'a, str> {
        if let Some(alt_txt) = &self.image_alt_text {
            alt_txt.clone()
        } else if let Some(img_lbl) = &self.image_label {
            img_lbl.clone()
        } else if let Some(lbl) = &self.label {
            lbl.clone()
        } else {
            "TODO".into()
        }
    }

    /// Return the label for the badge reference e.g. image_label or
    /// \<label\>-badge
    fn get_badge_label(&self) -> Cow<'a, str> {
        if let Some(ref img_lbl) = self.image_label {
            img_lbl.clone()
        } else if let Some(ref lbl) = self.label {
            format!("{}-badge", lbl).into()
        } else if let Some(ref alt_txt) = self.image_alt_text {
            alt_txt.clone()
        } else {
            "ABC".into() // TODO
        }
    }

    /// Return the badge's url and title (if present)
    fn get_badge_url_and_title(&self) -> Cow<'a, str> {
        if let Some(ref u) = self.image_url {
            if let Some(ref t) = self.image_title {
                format!("{} \"{}\"", u, t).into()
            } else {
                u.clone()
            }
        } else {
            Cow::from(String::new())
        }
    }

    /// Return a badge image that is clickable:
    /// \[ !\[ alt-text \]\[ badge-label \] \]\[ label \]
    pub(crate) fn to_link_with_badge(&self) -> Cow<'a, str> {
        format!(
            "[![{}][{}]][{}]",
            self.get_badge_alt_text(),
            self.get_badge_label(),
            self.get_label()
        )
        .into()
    }

    /// Return the reference definition for a badge image: [badge-label]: https://badge-image-url...  "image_title"
    pub(crate) fn to_badge_reference_definition(&self) -> Cow<'a, str> {
        format!(
            "[{}]: {}",
            self.get_badge_label(),
            self.get_badge_url_and_title()
        )
        .into()
    }
}

impl<'a> PartialOrd for Link<'a> {
    /// PartialOrd implementation for Link
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // The type is `Ord``,
        // thus we can implement `partial_cmp` by using `cmp`
        Some(self.cmp(other))
    }
}

// TODO
impl<'a> Ord for Link<'a> {
    /// Ord implementation for Link
    fn cmp(&self, other: &Self) -> Ordering {
        self.label
            .cmp(&other.label)
            .then(self.url.cmp(&other.url))
            .then(self.title.cmp(&other.title))
    }
}

// TODO
impl<'a> PartialEq for Link<'a> {
    /// PartialEq implementation for Link
    fn eq(&self, other: &Self) -> bool {
        (self.label == other.label)
            && (self.url == other.url)
            && (self.title == other.title)
    }
}

/// Eq implementation for Link
impl<'a> Eq for Link<'a> {}
