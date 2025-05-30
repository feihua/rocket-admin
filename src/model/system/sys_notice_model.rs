// Code generated by https://github.com/feihua/code_cli
// author：刘飞华
// createTime：2024/12/25 10:01:11

use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

/*
 *通知公告表
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Notice {
    pub id: Option<i64>,               //公告ID
    pub notice_title: String,          //公告标题
    pub notice_type: i8,               //公告类型（1:通知,2:公告）
    pub notice_content: String,        //公告内容
    pub status: i8,                    //公告状态（0:关闭,1:正常 ）
    pub remark: String,                //备注
    pub create_time: Option<DateTime>, //创建时间
    pub update_time: Option<DateTime>, //修改时间
}

/*
 *通知公告表基本操作
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
rbatis::crud!(Notice {}, "sys_notice");

/*
 *根据id查询通知公告表
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select!(Notice{select_by_id(id:&i64) -> Option => "`where id = #{id} limit 1`"}, "sys_notice");

/*
 *根据公告标题查询通知公告表
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select!(Notice{select_by_title(title:&str) -> Option => "`where notice_title = #{title} limit 1`"}, "sys_notice");

/*
 *根据条件分页查询通知公告表
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select_page!(Notice{select_sys_notice_list(title:&str, notice_type:i8, status:i8) =>"
    where 1=1
     if title != '':
       ` and notice_title = #{title} `
     if notice_type != 0:
      ` and notice_type = #{notice_type} `
     if status != 2:
       ` and status = #{status} `
     if !sql.contains('count'):
       ` order by create_time desc `"
},"sys_notice");
