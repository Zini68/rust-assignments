#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(unreachable_pub)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::similar_names)]

pub fn format_filename(name: &str) -> String {
    if name.ends_with(".txt") {
        name.to_string()
    } else {
        format!("{}.txt", name)
    }
}

pub fn is_valid_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}
