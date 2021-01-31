mod index;
mod resume;
mod blog;
mod resource;

pub use index::get_index as get_index;
pub use resource::get_css as get_css;
pub use resource::get_js as get_js;
pub use resume::get_resume as get_resume;