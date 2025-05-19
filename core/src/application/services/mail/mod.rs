pub mod interface;
pub mod implementation;

pub use interface::MailService;
pub use implementation::DefaultMailService;