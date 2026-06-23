pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_todos_table;
mod m20260620_070836_create_users_table;
mod m20260621_035326_add_email_to_users;
mod m20260622_133334_add_user_id_to_todos;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_todos_table::Migration),
            Box::new(m20260620_070836_create_users_table::Migration),
            Box::new(m20260621_035326_add_email_to_users::Migration),
            Box::new(m20260622_133334_add_user_id_to_todos::Migration),
        ]
    }
}
