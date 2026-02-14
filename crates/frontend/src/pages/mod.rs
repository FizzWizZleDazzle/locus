//! Page components

mod home;
mod practice;
mod ranked;
mod leaderboard;
mod login;
mod register;
mod settings;
mod verify_email;
mod forgot_password;
mod reset_password;

pub use home::Home;
pub use practice::Practice;
pub use ranked::Ranked;
pub use leaderboard::Leaderboard;
pub use login::Login;
pub use register::Register;
pub use settings::Settings;
pub use verify_email::VerifyEmail;
pub use forgot_password::ForgotPassword;
pub use reset_password::ResetPassword;
