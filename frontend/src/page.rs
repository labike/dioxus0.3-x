pub mod register;

pub use register::Register;

pub use route::*;

pub mod route {
    pub const REGISTER: &str = "/account/register";
}