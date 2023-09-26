#[cfg(feature = "re-exports-sea-orm")]
pub mod sea_orm {
    pub mod errors {
        pub use sea_orm::error::DbErr;
    }
}
