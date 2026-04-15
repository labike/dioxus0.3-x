pub mod register;
pub mod login;

pub use register::Register;
pub use login::Login;

pub use route::*;

pub mod route {
    pub const REGISTER: &str = "/account/register";
    pub const LOGIN: &str = "/account/login";
}