use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum IndexAnchorKind {
    /// Indexed text appears as is.
    /// {{i:<text>}}
    Regular,
    /// Indexed text appears in italics.
    /// {{ii:<text>}}
    Italics,
    /// Indexed text is hidden.
    /// {{hi:<text>}}
    Hidden,
}

impl Display for IndexAnchorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexAnchorKind::Regular => write!(f, "i:"),
            IndexAnchorKind::Italics => write!(f, "ii:"),
            IndexAnchorKind::Hidden => write!(f, "hi:"),
        }
    }
}
