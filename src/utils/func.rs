use crate::model::menu_model::{Router,Meta};
use crate::entity::sys_menu_entity::SysMenu;

pub fn router_arr_to_tree(re_list:&mut Vec<Router>,ori_arr:Vec<SysMenu>,pid:i64){
  for (_,it) in ori_arr.iter().enumerate(){
    if pid == it.parent_id && !it.menu_type.eq("F"){
      let temp_meta = Meta{
        title:it.menu_name.clone(),
        icon:it.icon.clone(),
        link:(||->Option<String>{
          if it.is_frame == 0{
            Some(it.path.clone())
          }else{
            None
          }
        })(),
        no_cache:it.is_cache==0
      };

      let mut children = Vec::<Router>::new();
      router_arr_to_tree(&mut children,ori_arr.clone(),it.menu_id);

      let temp_router = Router{
        always_show:(||->Option<bool>{
          if it.visible.eq("0") && !it.menu_type.eq("C"){
            Some(true)
          }else{
            None
          }
        })(),
        children:(||->Option<Vec<Router>>{
          if children.len()>0{
            Some(children)
          }else{
            None
          }
        })(),
        component:it.component.clone().map_or(String::from("Layout"), |v|{
            if v.is_empty(){
              String::from("Layout")
            }else{
              v
            }
          
          }
        ),
        hidden:it.status.eq("1"),
        name:it.path.clone(),
        path:(||->String{
          if it.menu_type.eq("C"){
            it.path.clone()
          }else{
            "/".to_owned()+&it.path
          }
        })(),
        redirect:(||->Option<String>{
          if it.is_frame == 1 && it.menu_type.eq("M"){
            Some(String::from("noRedirect"))
          }else{
            None
          }
        })(),
        meta:temp_meta
      };
      re_list.push(temp_router)
    }
  }
}