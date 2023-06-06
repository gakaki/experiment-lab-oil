use rbatis::{crud,html_sql};
use rbatis::executor::Executor;
use crate::entity::sys_user_role_entity::SysUserRoleEntity;

crud!(SysUserRoleEntity{},"sys_user_role");

#[html_sql("src/mapper/xml/user_role_xml.html")]
pub async fn select_role_id_by_user_id(rb: &mut dyn Executor,user_id:i64)->rbatis::Result<Vec<SysUserRoleEntity>>{
    impled!()
}