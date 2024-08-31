use serenity::all::standard::macros::group;

mod ban_message;
mod ban_word;
mod config;
mod join_role;
mod logs;
mod mod_role;
mod mute_role;
mod setup;
mod warn_expire;
mod warns_max;
mod whitelist;

pub use ban_message::*;
pub use ban_word::*;
pub use config::*;
pub use join_role::*;
pub use logs::*;
pub use mod_role::*;
pub use mute_role::*;
pub use setup::*;
pub use warn_expire::*;
pub use warns_max::*;
pub use whitelist::*;

// #[group]
// #[commands(
//     ban_message,
//     ban_word,
//     config,
//     join_role,
//     logs,
//     mod_role,
//     mute_role,
//     setup,
//     warn_expire,
//     warns_max,
//     whitelist
// )]
// struct Config;
