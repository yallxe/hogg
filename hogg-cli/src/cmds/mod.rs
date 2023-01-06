pub mod ping;
pub use ping::run as ping_command;

pub mod detections;
pub use detections::get_unviewed as unviewed_detections_command;
pub use detections::flush_detections as flush_command;