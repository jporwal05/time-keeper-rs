use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Projects {
    Table,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::alter()
            .table(Projects::Table) // Replace with your table name
            .rename_column(Alias::new("project_name"), Alias::new("name"))
            .to_owned();

        manager.alter_table(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Revert the renaming here
        let table = Table::alter()
            .table(Projects::Table) // Replace with your table name
            .rename_column(Alias::new("name"), Alias::new("project_name"))
            .to_owned();
        manager.alter_table(table).await?;
        Ok(())
    }
}
