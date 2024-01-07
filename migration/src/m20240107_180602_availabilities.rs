use std::borrow::BorrowMut;

use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Availabilities::Table)
                    .col(pk_auto(Availabilities::Id).borrow_mut())
                    .col(integer_null(Availabilities::TotalAvailability).borrow_mut())
                    .col(timestamp_null(Availabilities::AvailableFrom).borrow_mut())
                    .col(timestamp_null(Availabilities::AvailableTill).borrow_mut())
                    .col(integer(Availabilities::ProjectId).borrow_mut())
                    .col(uuid(Availabilities::UserId).borrow_mut())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-availabilities-projects")
                            .from(Availabilities::Table, Availabilities::ProjectId)
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-availabilities-users")
                            .from(Availabilities::Table, Availabilities::UserId)
                            .to(Users::Table, Users::Pid)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Availabilities::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Availabilities {
    Table,
    Id,
    TotalAvailability,
    AvailableFrom,
    AvailableTill,
    ProjectId,
    UserId,
}

#[derive(DeriveIden)]
enum Projects {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Users {
    Table,
    Pid,
}
