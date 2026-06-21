use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(string(Users::Email).unique_key())  
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .alter_table(
                Table::alter().
                    table(Users::Table)
                    .drop_column(Users::Email)
                    .to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Email,
}
