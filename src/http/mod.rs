pub mod html;
pub mod response;
pub mod file_server;
pub mod server;

// Re-export only what's used externally
pub use server::serve;
