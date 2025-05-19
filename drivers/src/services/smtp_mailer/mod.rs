pub mod interface;
pub mod implementation;

pub use interface::EmailTransporter;
pub use implementation::SmtpMailer;