pub mod event;
pub mod ics_parser;
pub mod fetcher;
pub mod scheduler;

// Re-export commonly used items
pub use event::{load_events, get_last_update};
pub use scheduler::start_background_sync;
