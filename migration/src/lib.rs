mod types;

mod m20220825_211845_create_departments_table;
mod m20220825_211904_create_api_db_table;
mod m20220825_211916_create_attributes_table;
mod m20220825_211920_create_attribute_values_table;
mod m20220825_211935_create_crons_table;
mod m20220825_211957_create_crons_logs_table;
mod m20220825_212010_create_login_logs_table;
mod m20220825_212018_create_menus_table;
mod m20220825_212030_create_operation_logs_table;
mod m20220825_212131_create_positions_table;
mod m20220825_212141_create_roles_table;
mod m20220825_212152_create_role_api_map_table;
mod m20220825_212215_create_role_department_map_table;
mod m20220825_212228_create_update_logs_table;
mod m20220825_212238_create_users_table;
mod m20220825_212250_create_user_department_map_table;
mod m20220825_212300_create_user_position_map_table;
mod m20220825_212309_create_user_role_map_table;
mod m20220825_212320_create_user_online_table;

pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20220825_211845_create_departments_table::Migration),
            Box::new(m20220825_211904_create_api_db_table::Migration),
            Box::new(m20220825_211916_create_attributes_table::Migration),
            Box::new(m20220825_211920_create_attribute_values_table::Migration),
            Box::new(m20220825_211935_create_crons_table::Migration),
            Box::new(m20220825_211957_create_crons_logs_table::Migration),
            Box::new(m20220825_212010_create_login_logs_table::Migration),
            Box::new(m20220825_212018_create_menus_table::Migration),
            Box::new(m20220825_212030_create_operation_logs_table::Migration),
            Box::new(m20220825_212131_create_positions_table::Migration),
            Box::new(m20220825_212141_create_roles_table::Migration),
            Box::new(m20220825_212152_create_role_api_map_table::Migration),
            Box::new(m20220825_212215_create_role_department_map_table::Migration),
            Box::new(m20220825_212228_create_update_logs_table::Migration),
            Box::new(m20220825_212238_create_users_table::Migration),
            Box::new(m20220825_212250_create_user_department_map_table::Migration),
            Box::new(m20220825_212300_create_user_position_map_table::Migration),
            Box::new(m20220825_212309_create_user_role_map_table::Migration),
            Box::new(m20220825_212320_create_user_online_table::Migration),
        ]
    }
}
