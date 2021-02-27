mod blog;
mod index;
mod resource;
mod resume;
mod contact;

pub use index::get_index;
pub use resource::get_file;
pub use resume::get_resume;
pub use contact::get_contact;
pub use blog::get_blogs;