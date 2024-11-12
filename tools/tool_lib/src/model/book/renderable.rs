use serde::Serialize;

pub trait Renderable: Serialize {
    fn get_what_to_render(&self) -> Vec<RenderInfo>;
    fn get_children(&self) -> impl IntoIterator<Item = impl Renderable>;
}

// struct Render<T> {
//   : String,
//   data: T,
// }

// All the information required to render a given chapter / subchapter / index
#[derive(Serialize, Debug)]
pub struct RenderInfo {
    // name (e.g. path without glob prefix) of the template for a given object
    pub template_name: String,
    // String containing a template to build the destination file path
    pub dest_filepath_template: String,
}

impl RenderInfo {
    fn new(template_name: String, dest_filepath_template: String) -> Self {
        Self {
            template_name,
            dest_filepath_template,
        }
    }
}
