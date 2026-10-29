// TODO Reviewed ??-??-2026.

#![expect(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! Parse a small XML document with `xml5ever` and inspect its DOM tree.

use std::borrow::Cow;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

use xml5ever::Attribute;
use xml5ever::LocalName;
use xml5ever::Namespace;
use xml5ever::QualName;
use xml5ever::driver::parse_document;
use xml5ever::interface::tree_builder::AppendNode;
use xml5ever::interface::tree_builder::AppendText;
use xml5ever::interface::tree_builder::ElementFlags;
use xml5ever::interface::tree_builder::NodeOrText;
use xml5ever::interface::tree_builder::QuirksMode;
use xml5ever::ns;
use xml5ever::tendril::StrTendril;
use xml5ever::tendril::TendrilSink;
use xml5ever::tree_builder::ElemName;
use xml5ever::tree_builder::TreeSink;

/// A reference to an XML node.
type Handle = Rc<Node>;

/// The name view returned to the XML tree builder.
#[derive(Debug)]
struct NameRef<'a>(&'a QualName);

impl ElemName for NameRef<'_> {
    fn ns(&self) -> &Namespace {
        &self.0.ns
    }

    fn local_name(&self) -> &LocalName {
        &self.0.local
    }
}

/// The kinds of nodes stored in the small example tree.
enum NodeKind {
    /// The document node and its ordered children.
    Document(RefCell<Vec<Handle>>),
    /// A document fragment and its ordered children.
    Fragment(RefCell<Vec<Handle>>),
    /// An element with attributes, children, and optional template contents.
    Element {
        /// The element attributes.
        attrs: RefCell<Vec<Attribute>>,
        /// The element's ordered children.
        children: RefCell<Vec<Handle>>,
        /// The fragment associated with a template element.
        template_contents: Option<Handle>,
    },
    /// Text content accumulated from XML character tokens.
    Text(RefCell<String>),
    /// A comment node.
    Comment(String),
    /// A processing instruction node.
    ProcessingInstruction {
        /// The processing instruction target.
        target: String,
        /// The processing instruction body.
        data: String,
    },
    /// A document type node.
    Doctype {
        /// The public identifier.
        public_id: String,
        /// The system identifier.
        system_id: String,
    },
}

/// One XML node with its qualified name, kind, and optional parent.
struct Node {
    /// Qualified name used by the tree builder for elements.
    name: QualName,
    /// Content and child nodes for this node.
    kind: NodeKind,
    /// Weak parent pointer used for detach and reparent operations.
    parent: RefCell<Option<Weak<Self>>>,
}

/// A small DOM implementation used as `xml5ever`'s tree sink.
struct XmlTree {
    /// The document node that owns the parsed tree.
    document: Handle,
    /// Errors reported by the XML parser.
    errors: RefCell<Vec<Cow<'static, str>>>,
    /// Quirks mode selected by the tree builder.
    quirks_mode: Cell<QuirksMode>,
}

impl Default for XmlTree {
    fn default() -> Self {
        Self {
            document: Rc::new(Node {
                name: QualName::new(
                    None,
                    ns!(),
                    LocalName::from("document".to_owned()),
                ),
                kind: NodeKind::Document(RefCell::new(Vec::new())),
                parent: RefCell::new(None),
            }),
            errors: RefCell::new(Vec::new()),
            quirks_mode: Cell::new(QuirksMode::NoQuirks),
        }
    }
}

/// Returns the child list for container nodes.
fn children(node: &Node) -> Option<&RefCell<Vec<Handle>>> {
    match &node.kind {
        NodeKind::Document(children) | NodeKind::Fragment(children) => {
            Some(children)
        }
        NodeKind::Element { children, .. } => Some(children),
        _ => None,
    }
}

/// Appends a node handle to a parent and records the weak parent pointer.
fn append_child(parent: &Handle, child: Handle) {
    if let Some(parent_children) = children(parent) {
        *child.parent.borrow_mut() = Some(Rc::downgrade(parent));
        parent_children.borrow_mut().push(child);
    }
}

/// Creates a node with the supplied qualified name and content kind.
fn new_node(name: QualName, kind: NodeKind) -> Handle {
    Rc::new(Node {
        name,
        kind,
        parent: RefCell::new(None),
    })
}

/// Creates a non-element node with a placeholder qualified name.
fn unnamed_node(kind: NodeKind) -> Handle {
    new_node(
        QualName::new(None, ns!(), LocalName::from(String::new())),
        kind,
    )
}

impl TreeSink for XmlTree {
    type ElemName<'a> = NameRef<'a>;
    type Handle = Handle;
    type Output = Self;

    fn finish(self) -> Self::Output {
        self
    }

    fn parse_error(&self, msg: Cow<'static, str>) {
        self.errors.borrow_mut().push(msg);
    }

    fn get_document(&self) -> Self::Handle {
        Rc::clone(&self.document)
    }

    fn elem_name<'a>(&'a self, target: &'a Self::Handle) -> Self::ElemName<'a> {
        NameRef(&target.name)
    }

    fn create_element(
        &self,
        name: QualName,
        attrs: Vec<Attribute>,
        flags: ElementFlags,
    ) -> Self::Handle {
        let template_contents = flags.template.then(|| {
            unnamed_node(NodeKind::Fragment(RefCell::new(Vec::new())))
        });
        new_node(
            name,
            NodeKind::Element {
                attrs: RefCell::new(attrs),
                children: RefCell::new(Vec::new()),
                template_contents,
            },
        )
    }

    fn create_comment(&self, text: StrTendril) -> Self::Handle {
        unnamed_node(NodeKind::Comment(text.to_string()))
    }

    fn create_pi(&self, target: StrTendril, data: StrTendril) -> Self::Handle {
        unnamed_node(NodeKind::ProcessingInstruction {
            target: target.to_string(),
            data: data.to_string(),
        })
    }

    fn append(&self, parent: &Self::Handle, child: NodeOrText<Self::Handle>) {
        match child {
            AppendNode(child) => append_child(parent, child),
            AppendText(text) => {
                let Some(parent_children) = children(parent) else {
                    return;
                };
                let mut parent_children = parent_children.borrow_mut();
                if let Some(Node {
                    kind: NodeKind::Text(contents),
                    ..
                }) = parent_children.last().map(Rc::as_ref)
                {
                    contents.borrow_mut().push_str(text.as_ref());
                } else {
                    let text_node = unnamed_node(NodeKind::Text(RefCell::new(
                        text.to_string(),
                    )));
                    *text_node.parent.borrow_mut() =
                        Some(Rc::downgrade(parent));
                    parent_children.push(text_node);
                }
            }
        }
    }

    fn append_based_on_parent_node(
        &self,
        element: &Self::Handle,
        prev_element: &Self::Handle,
        child: NodeOrText<Self::Handle>,
    ) {
        let parent = element
            .parent
            .borrow()
            .as_ref()
            .and_then(Weak::upgrade)
            .unwrap_or_else(|| Rc::clone(prev_element));
        self.append(&parent, child);
    }

    fn append_doctype_to_document(
        &self,
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril,
    ) {
        let doctype = new_node(
            QualName::new(None, ns!(), LocalName::from(name.to_string())),
            NodeKind::Doctype {
                public_id: public_id.to_string(),
                system_id: system_id.to_string(),
            },
        );
        append_child(&self.document, doctype);
    }

    fn get_template_contents(&self, target: &Self::Handle) -> Self::Handle {
        match &target.kind {
            NodeKind::Element {
                template_contents: Some(contents),
                ..
            } => Rc::clone(contents),
            _ => Rc::clone(target),
        }
    }

    fn same_node(&self, first: &Self::Handle, second: &Self::Handle) -> bool {
        Rc::ptr_eq(first, second)
    }

    fn set_quirks_mode(&self, mode: QuirksMode) {
        self.quirks_mode.set(mode);
    }

    fn append_before_sibling(
        &self,
        sibling: &Self::Handle,
        new_node: NodeOrText<Self::Handle>,
    ) {
        let new_node = match new_node {
            AppendNode(node) => node,
            AppendText(text) => {
                unnamed_node(NodeKind::Text(RefCell::new(text.to_string())))
            }
        };
        self.remove_from_parent(&new_node);
        let Some(parent) =
            sibling.parent.borrow().as_ref().and_then(Weak::upgrade)
        else {
            return;
        };
        let Some(parent_children) = children(&parent) else {
            return;
        };
        let mut parent_children = parent_children.borrow_mut();
        let Some(index) = parent_children
            .iter()
            .position(|child| Rc::ptr_eq(child, sibling))
        else {
            return;
        };
        *new_node.parent.borrow_mut() = Some(Rc::downgrade(&parent));
        parent_children.insert(index, new_node);
    }

    fn add_attrs_if_missing(
        &self,
        target: &Self::Handle,
        attrs: Vec<Attribute>,
    ) {
        let NodeKind::Element {
            attrs: target_attrs,
            ..
        } = &target.kind
        else {
            return;
        };
        let mut target_attrs = target_attrs.borrow_mut();
        for attr in attrs {
            if !target_attrs.iter().any(|existing| {
                existing.name.expanded() == attr.name.expanded()
            }) {
                target_attrs.push(attr);
            }
        }
    }

    fn remove_from_parent(&self, target: &Self::Handle) {
        let Some(parent) = target
            .parent
            .borrow_mut()
            .take()
            .and_then(|weak| weak.upgrade())
        else {
            return;
        };
        if let Some(parent_children) = children(&parent) {
            parent_children
                .borrow_mut()
                .retain(|child| !Rc::ptr_eq(child, target));
        }
    }

    fn reparent_children(
        &self,
        node: &Self::Handle,
        new_parent: &Self::Handle,
    ) {
        let Some(node_children) = children(node) else {
            return;
        };
        let moved_children =
            node_children.borrow_mut().drain(..).collect::<Vec<_>>();
        for child in moved_children {
            append_child(new_parent, child);
        }
    }
}

/// Parses a small XML document and prints the library and book nodes.
///
/// # Errors
///
/// Returns an error if parsing reports errors or the expected nodes are
/// missing from the resulting tree.
fn main() -> anyhow::Result<()> {
    let xml = r#"<library><book id="b1">Rust</book></library>"#;
    let dom = parse_document(XmlTree::default(), Default::default()).one(xml);
    anyhow::ensure!(
        dom.errors.borrow().is_empty(),
        "XML parser reported errors"
    );

    let library = children(&dom.document)
        .and_then(|nodes| nodes.borrow().first().cloned())
        .ok_or_else(|| anyhow::anyhow!("XML document has no root element"))?;
    let library_name = dom.elem_name(&library).local_name().to_string();
    anyhow::ensure!(
        library_name == "library",
        "unexpected root: {library_name}"
    );

    let book = children(&library)
        .and_then(|nodes| nodes.borrow().first().cloned())
        .ok_or_else(|| anyhow::anyhow!("library has no book element"))?;
    let book_name = dom.elem_name(&book).local_name().to_string();
    anyhow::ensure!(book_name == "book", "unexpected child: {book_name}");

    let book_id = match &book.kind {
        NodeKind::Element { attrs, .. } => attrs
            .borrow()
            .iter()
            .find(|attr| &*attr.name.local == "id")
            .map(|attr| attr.value.to_string())
            .ok_or_else(|| anyhow::anyhow!("book has no id attribute"))?,
        _ => anyhow::bail!("book handle is not an element"),
    };
    anyhow::ensure!(book_id == "b1", "unexpected book id: {book_id}");

    let text = children(&book)
        .and_then(|nodes| nodes.borrow().first().cloned())
        .ok_or_else(|| anyhow::anyhow!("book has no text node"))?;
    let contents = match &text.kind {
        NodeKind::Text(contents) => contents.borrow().clone(),
        _ => anyhow::bail!("book child is not a text node"),
    };
    anyhow::ensure!(contents == "Rust", "unexpected book text: {contents}");

    println!(
        "root: {library_name}; child: {book_name} id={book_id}; text: {contents}"
    );
    Ok(())
}
// ANCHOR_END: example

/// Runs the XML DOM parsing example.
///
/// # Errors
///
/// Returns an error if XML parsing fails or produces an unexpected tree.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks the parsed root, child element, attribute, and text node.
    /// Runs the xml5ever example through its entry point.
    #[test]
    fn test_xml5ever_example_entry_point() -> anyhow::Result<()> {
        main()
    }

    /// Parsing well-formed XML builds the expected tree without errors.
    #[test]
    fn builds_the_tree() {
        let dom = parse_document(XmlTree::default(), Default::default())
            .one(r#"<a><b k="v">t</b></a>"#);
        assert!(dom.errors.borrow().is_empty());
        let a = children(&dom.document).unwrap().borrow()[0].clone();
        assert_eq!(&*a.name.local, "a");
        let b = children(&a).unwrap().borrow()[0].clone();
        assert_eq!(&*b.name.local, "b");
    }

    /// Malformed XML is reported through the error list.
    #[test]
    fn malformed_xml_reports_errors() {
        let dom = parse_document(XmlTree::default(), Default::default())
            .one("<a><b></a>");
        assert!(!dom.errors.borrow().is_empty());
    }
}
