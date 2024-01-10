use sql_tool_kit::*;

#[derive(GenFields, GenValues)]
#[config(database = "postgres")]
pub struct InsertForm {
    /// 标题
    pub title: String,
    /// 副标题
    #[value(index = 1)]
    pub subtitle: Option<String>,
    /// 图片地址
    pub image_url: String,
    /// 跳转链接
    pub link_url: Option<String>,
    /// 开始时间
    #[value(value = "now()")]
    pub start_time: String,
    /// 结束时间，如果没有结束时间，该广告会一直显示下去
    pub end_time: String,
    /// 描述
    pub description: Option<String>,
    /// 类型
    #[field(rename = "type")]
    pub ty: i32,
    /// 排序
    pub sort: i32,
}
#[derive(GenWhere)]
#[config(database = "postgres")]
pub struct QueryForm {
    /// 关键字, 可以在 标题，副标题，描述上查询
    #[r#where(condition_all = "title like {index} AND subtitle like {index} AND description like {index}")]
    pub keyword: Option<String>,
    /// 开始时间
    #[r#where(condition = ">=")]
    pub start_time: Option<String>,
    /// 结束时间
    #[r#where(condition = "<=")]
    pub end_time: Option<String>,
    /// 广告类型
    #[r#where()]
    pub ty: i32,
    /// 页码信息
    #[r#where(ignore)]
    pub page_info: usize,
}

#[derive(GenSelect)]
pub struct QueryRow {
    /// 标题
    pub title: String,
    /// 副标题
    pub subtitle: Option<String>,
    /// 图片地址
    pub image_url: String,
    /// 跳转链接
    pub link_url: Option<String>,
    /// 开始时间
    pub start_time: String,
    /// 结束时间，如果没有结束时间，该广告会一直显示下去
    pub end_time: String,
    /// 描述
    pub description: Option<String>,
    /// 类型
    #[select(rename = "type")]
    pub ty: i32,
    /// 排序
    pub sort: i32,
}


#[derive(GenSet)]
#[config(database = "postgres", index = 1, ignore_no_macro_set = false)] // ignore_no_macro_set = false, 设置不忽略没有设置 #[set()] 字段的结构
pub struct UpdateForm {
    #[set(r#where)]
    pub id: i32,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub image_url: Option<String>,
    pub link_url: Option<String>,
    pub description: Option<String>,
    #[set(value = "now()")]
    pub updated_at: Option<()>,
}

#[derive(GenWhere)]
#[config(database = "postgres")]
pub struct DeleteForm {
    #[r#where()]
    pub id: i32,
    #[r#where()]
    pub title: Option<String>,
    #[r#where(condition = ">=")]
    pub start_time: Option<String>,
    #[r#where(condition = "<=")]
    pub end_time: Option<String>,
}

fn main() {
    // 结果：insert into table_name (title, subtitle, image_url, link_url, start_time, end_time, description, type, sort) values ($1, $1, $2, $3, now(), $4, $5, $6, $7)
    let query = format!("insert into table_name ({}) values ({})", InsertForm::generate_fields_clause().join(", "), InsertForm::generate_values_clause().join(", "));
    println!("{}", query);

    let data = QueryForm {
        keyword: Some("这是标题".to_string()),
        start_time: Some("2024/12/12".to_string()),
        end_time: None,
        ty:1,
        page_info: 0,
    };

    let query = format!("select {} from table_name where {}", QueryRow::generate_select_clause().join(", "), data.generate_where_clause().join(" AND "));

    // let mut sql = sqlx::query::<_, QueryRow>(query);
    // if data.keyword.is_some() {
    //     sql = sql.bind(data.keyword);
    // }
    // if data.start_time.is_some() {
    //     sql = sql.bind(data.start_time);
    // }
    // if data.end_time.is_some() {
    //     sql = sql.bind(data.end_time);
    // }
    // let result = sql.bind(ty).fetch_all(pool).await?;

    println!("{}", query);

    let data = UpdateForm {
        id: 1,
        title: Some("这是标题".to_string()),
        subtitle: None,
        image_url: None,
        link_url: None,
        description: Some("这是描述".to_string()),
        updated_at: Some(()),
    };
    let (set_values, where_value) = data.generate_set_and_where_clause();
    let query = format!("update table_name set {} where {}", set_values.join(", "), where_value.join(" AND "));

    println!("{}", query);

    let data = DeleteForm {
        id: 1,
        title: None,
        start_time: Some("2025/12/12".to_string()),
        end_time: Some("2024/12/12".to_string()),
    };

    // 输出：delete table_name where id = $1 AND start_time >= $2 AND end_time <= $3
    let query = format!("delete table_name where {}", data.generate_where_clause().join(" AND "));

    println!("{}", query);
}
