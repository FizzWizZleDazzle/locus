//! Page components

mod daily;
mod daily_archive;
mod daily_detail;
mod forgot_password;
mod home;
mod leaderboard;
mod login;
mod practice;
mod privacy_policy;
mod ranked;
mod register;
mod reset_password;
mod settings;
mod stats;
mod terms_of_service;
mod verify_email;

pub use daily::Daily;
pub use daily_archive::DailyArchive;
pub use daily_detail::DailyPuzzleDetail;
pub use forgot_password::ForgotPassword;
pub use home::Home;
pub use leaderboard::Leaderboard;
pub use login::Login;
pub use practice::Practice;
pub use privacy_policy::PrivacyPolicy;
pub use ranked::Ranked;
pub use register::Register;
pub use reset_password::ResetPassword;
pub use settings::Settings;
pub use stats::Stats;
pub use terms_of_service::TermsOfService;
pub use verify_email::VerifyEmail;
