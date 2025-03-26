use serde::Serialize;

pub trait Renderable: Serialize {
    fn get_what_to_render(&self) -> Vec<RenderInfo>;
    fn get_children(&self) -> impl IntoIterator<Item = impl Renderable>;
}

/// An empty implementation of the `Renderable` trait.
#[derive(Serialize, Debug)]
pub struct Render;

impl Renderable for Render {
    fn get_what_to_render(&self) -> Vec<RenderInfo> {
        vec![]
    }

    fn get_children(&self) -> impl IntoIterator<Item = impl Renderable> {
        std::iter::empty::<Render>()
    }
}

// All the information required to render a given chapter / subchapter / index
#[derive(Serialize, Debug)]
pub struct RenderInfo {
    // name (e.g. path without glob prefix) of the template for a given object
    pub template_name: String,
    // String containing a template to build the destination file path
    pub dest_filepath_template: String,
}

impl RenderInfo {
    pub fn new(template_name: String, dest_filepath_template: String) -> Self {
        Self {
            template_name,
            dest_filepath_template,
        }
    }
}
