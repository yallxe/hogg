pub mod ping;
pub use ping::run as ping_command;

pub mod detections;
pub use detections::run as unviewed_detections_command;
