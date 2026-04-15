pub mod register;
pub mod login;
mod home;

pub use register::Register;
pub use login::Login;
pub use home::Home;

pub use route::*;

pub mod route {
    pub const REGISTER: &str = "/account/register";
    pub const LOGIN: &str = "/account/login";
    pub const HOME: &str = "/home";
}