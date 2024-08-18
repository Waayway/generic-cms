use axum::async_trait;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

use super::entities::get_migrations;

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        get_migrations()
    }
}

#[macro_export]
macro_rules! migration_enum {
    ($table:ident, { $($name:ident),* }) => {
        #[derive(DeriveIden)]
        enum $table {
            Table,
            $($name,)*
        }
    };
}

#[macro_export]
macro_rules! col_def {
    (($col:expr => $type: ident)) => {
        $type(Alias::new($col))
    };
    (($col:expr => $type: ident, default: $default:expr, $($other:tt),*)) => {
        col_def!($col => $type $($other)*).default($default)
    };
    (($col:expr => $type: ident, null, $($other:tt),*)) => {
        col_def!($col => $type $($other)*).null()
    };
    (($col:expr => $type: ident, not_null, $($other:tt),*)) => {
        col_def!($col => $type $($other)*).null()
    };
    (($col:expr => $type: ident, extra: $extra:expr, $($other:tt),*)) => {
        col_def!($col => $type $($other)*).extra($extra)
    };
}

#[macro_export]
macro_rules! create_table {
    ($manager:ident, $table:expr, { $($col:tt),* }) => {
        $manager.create_table(
            Table::create()
                .table(Alias::new($table))
                .if_not_exists()
                $(
                .col(crate::col_def!($col))
                )*
                .to_owned()
        ).await?;
    };
}

#[macro_export]
macro_rules! drop_table {
    ($manager:ident, $table:expr) => {
        $manager
            .drop_table(Table::drop().table(Alias::new($table)).to_owned())
            .await?;
    };
}

#[macro_export]
macro_rules! alter_table_col {
    ($table:ident, (add, $col:tt)) => {
        $table.add_column(crate::col_def!($col))
    };
    ($table:ident,(modify, $col: tt)) => {
        $table.modify_column(crate::col_def!($col))
    };
    ($table:ident,(drop, $col:expr)) => {
        $table.drop_column(Alias::new($col))
    };

    ($table:ident,(add_fk, $alias:expr, ($from_tbl:expr, $from_col:expr), ($to_tbl:expr, $to_col:expr))) => {
        $table.add_foreign_key(
            &TableForeignKey::new()
                .name($alias)
                .from_tbl(Alias::new($from_tbl))
                .from_col(Alias::new($from_col))
                .to_tbl(Alias::new($to_tbl))
                .to_col(Alias::new($to_col))
                .to_owned(),
        )
    };
    ($table:ident,(drop_fk, $alias:expr)) => {};
}

#[macro_export]
macro_rules! alter_table {
    ($manager:ident, $table:expr, {$($col:tt),*}) => {
        let mut alter_table = Table::alter();
        alter_table.table(Alias::new($table));

        $(
        crate::alter_table_col!(alter_table, $col);
        )*

        $manager.alter_table(alter_table.to_owned()).await?
    };
}
