mod constants;
mod tera;

mod home;
mod random_cat_gif;
mod search_gif;

pub use home::get_home_html;
pub use random_cat_gif::get_random_cat_gif_html;
pub use search_gif::get_search_gif_html;
