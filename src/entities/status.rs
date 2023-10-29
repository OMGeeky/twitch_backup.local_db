use sea_orm::{DeriveActiveEnum, EnumIter};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum Status {
    #[sea_orm(num_value = 0)]
    NotStarted,
    #[sea_orm(num_value = 10)]
    Downloading,
    #[sea_orm(num_value = 19)]
    DownloadFailed,
    #[sea_orm(num_value = 20)]
    Downloaded,
    #[sea_orm(num_value = 30)]
    Splitting,
    #[sea_orm(num_value = 39)]
    SplitFailed,
    #[sea_orm(num_value = 40)]
    Split,
    #[sea_orm(num_value = 50)]
    Uploading,
    #[sea_orm(num_value = 55)]
    PartiallyUploaded,
    #[sea_orm(num_value = 59)]
    UploadFailed,
    #[sea_orm(num_value = 60)]
    Uploaded,
    #[sea_orm(num_value = 90)]
    Failed,
    #[sea_orm(num_value = 100)]
    Done,
}
