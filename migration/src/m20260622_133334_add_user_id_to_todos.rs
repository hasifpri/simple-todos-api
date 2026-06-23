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
                    .table(Todos::Table)
                    .add_column(uuid(Todos::UserId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .alter_table(Table::alter().
                table(Todos::Table)
                .drop_column(Todos::UserId)
                .to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Todos {
    Table,
    UserId,
}
