# SQL 语句辅助生成器

`sql_tool_kit` 库提供了一系列派生宏（`GenFields`, `GenSelect`, `GenValues`, `GenSet`, `GenWhere`），
用于自动派生与 SQL 语句构建相关的 trait 实现。这些宏简化了根据结构体字段生成 SQL 语句的过程。

##  派生宏介绍

### `#[derive(GenFields)]` 和 `#[derive(GenSelect)]`

这两个派生宏功能相似，主要区别在于命名。它们都处理字段及其上的 `#[field(...)]` 和 `#[select(...)]` 宏。

使用方法：StructName::generate_fields_clause() 和 StructName::generate_select_clause() 
分别生成字段名称数组（如 `["field1", "field2", ...]`）。
应用场景：通过 `.join(", ")` 方法将返回的数组拼接为字符串，可用于 SQL 语句的 `SELECT` 和 `INSERT` 部分。

字段宏参数：
- `ignore` - 忽略该字段
- `rename` - 字段重命名

导入
```rust
/// 导入 GenFields 和对应实现的 trait
use sql_tool_kit::{GenFields, FieldsAttributeMacro};
/// 导入 GenSelect
use sl_tool_kit::{GenSelect, SelectAttributeMacro};
```

使用
```rust
#[derive(GenFields)]
pub struct FieldsStruct {
    field: i32,
    #[field(ignore)]
    field2: i32,
    #[field(rename = "rename_field")]
    field3: i32,
}
FieldsStruct::generate_fields_clause(); // 输出： [“field1", "rename_field"]

#[derive(GenFields)]
pub struct SelectStruct {
    field: i32,
    #[field(ignore)]
    field2: i32,
    #[field(rename = "rename_field")]
    field3: i32,
}
SelectStruct::generate_fields_clause(); // 输出： [“field1", "rename_field"]
```

### `#[derive(GenValues)]`

`GenValues` 生成用于 insert 语句中 values 部分，通过 `StructName::generate_values_clause()` 得到
`["$1", "$2", ...]`。

需要在结构上使用宏 `#[config(...)]` 来配置序列化的方式：

宏参数：
- `#[config(...)]`: 设置全局配置。
  - `database` - 指定生成的数据库类型，目前支持 `postgresql` `mysql` `mariadb` `sqlite` `mssql`
  - `index` - 指定开始的序列，仅 `postgresql` `mssql` 上有效

`#[value(...)]` 接受的参数：
- `ignore` - 忽略该字段
  - `index` - 设置当前值的index，当设置了这个参数后，全局的 index 不会加一
  - `value` - 直接替换当前的 `${index}` ，当设置了这个参数后，全局的 index 不会加一
      - 例如：`#[value(value = "true")]` => ["$1", "true",...]

使用方式
```rust
use sql_tool_kit::{GenValues, ValuesAttributeMacro};

#[derive(GenValues)]
#[config(database = "postgres")]
pub struct ValuesStruct {
  #[value(ignore)]
  field1: i32,
  #[value(index = 2)]
  field2: i32,
  #[value(value = "20")]
  field3: i32,
}
ValuesStruct::generate_values_clause(); // 输出：["$2", "20"]
```

### `#[derive(GenWhere)]`

用于生成 SQL `WHERE` 语句部分。此宏依赖于 `WhereAttributeMacro` trait。
使用方法 `where_data.generate_where_clause()` 会返回一个字段和条件组成的字符串数组。
使用方法 `where_data.generate_where_clause_with_index(index)` 可以设置开始初始的 index 值。

宏参数：
- `#[config(...)]`: 设置全局配置。
  - `database`: 指定数据库类型，影响占位符格式（支持 postgres, mysql, sqlite, mariadb, mssql）。
  - `index`: 设置占位符的起始索引。
  - `ignore_none`: 是否忽略 `Option::None` 值，默认为 `true`。
  - `ignore_no_macro_where`: 是否忽略没有 `#[r#where(...)]` 宏的字段，默认值为 `true`, 为 `true` 时可配合 `GenSet` 宏使用。

- `#[r#where(...)]`: 字段级别宏，用于自定义字段在 `WHERE` 语句中的表现。
  - `ignore`: 忽略该字段。
  - `rename`: 字段重命名，接受字符串类型。
  - `condition`: 指定字段的比较条件，默认值为 ”=“，如果该值设置为空及 "", 会报错。
  - `condition_all`: 应用于所有字段的通用条件，缺省值为 `"{name} {condition} {index}"`。
    - `{name}`: 字段名称或 `rename` 指定的名称。
    - `{condition}`: `condition` 参数指定的比较条件。如果 `condition_all`。
    - `{index}`: `index` 参数指定的占位符索引。如果字段未设置 `index`，则使用全局 `index`。
  - `ignore_none`: 当字段为 `Option::None` 时是否忽略，接受布尔类型。
  - `value`: 自定义字段的值，接受字符串类型。
  - `index`: 自定义占位符序号（如果数据库支持），接受整型。

字段宏属性优先级：
`ignore` > `ignore_none` > `condition_all` > `rename` = `condition` = `value` > `index`

### `#[derive(GenSet)]`

用于生成 SQL `UPDATE` 语句中的 `SET` 部分。它依赖于 `SetAttributeMacro` trait。
例如，`update table_name set field1 = $1, field2 = $2 ... where ...`
使用方法 `update_data.generate_set_clause()` 返回值类似于 `["field1 = $1", "field2 = $2", ...]`。
为了方便接入后续的 where 语句，在 `#[set(...)]` 添加了 `where` 参数，它可以为 `where` 或 `where = "..."`
通过方法 `generate_set_and_where_clause()` 返回值一个元组 `(["field1 = $1", ...], ["field5 = $5", "field6 > $6", ...])`,
第一个为 set 的值，第二个为 where 的值

宏参数：
- `#[config(...)]`: 设置一些配置。
  - `database`: 指定数据库类型，影响占位符的格式（支持 mysql, postgres, sqlite, mariadb, mssql）。
  - `index`: 设置占位符的起始索引。
  - `ignore_none`: 是否忽略 `Option::None` 值，默认为 `true`。
  - `ignore_no_macro_set`: 默认忽略没有 `#[set(...)]` 宏的字段，为 `true` 时配合 `GenWhere` 宏使用。
  - `ignore_set_and_where`: 当 `#[set(...)]`存在 `where` 参数是，会忽略 `set` 值，默认为 `false`

- `#[set(...)]`: 字段级别的宏，用于自定义字段在生成的 `SET` 语句中的表现。
  - `ignore`: 忽略该字段。
  - `r#where`: 将该字段设置为 where，有多种使用方式。1. `#[set(r#where)]` `#[set(r#where = "{field = {index}")]`
  - `ignore_none`: 当字段为 `Option::None` 时是否忽略，接受布尔类型。
  - `ignore_set`: 在 set 上忽略该字段。
  - `rename`: 字段重命名，接受字符串类型。
  - `condition`: 当设置 `r#where` 时生效
  - `value`: 自定义字段的值，接受字符串类型。
  - `index`: 自定义占位符序号（如果数据库支持），接受整型。

宏的优先级：`ignore` > `ignore_none` > `r#where` = `ignore_set` > `rename` = `value` = `condition` > `index`



## 使用示例

### insert 语句
```rust
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

fn main() {
    // 结果：insert into table_name (title, subtitle, image_url, link_url, start_time, end_time, description, type, sort) values ($1, $1, $2, $3, now(), $4, $5, $6, $7)
    let query = format!("insert into table_name ({}) values ({})", InsertForm::generate_fields_clause().join(", "), InsertForm::generate_values_clause().join(", "));
}

```

### update 更新语句

```rust
#[derive(GenSet)]
// ignore_no_macro_set = false, 设置不忽略没有设置 #[set()] 字段的结构
// index = 1, 设置 index 从 1 开始，默认值：如果不设置，index 则默认为 1
#[config(database = "postgres", index = 1, ignore_no_macro_set = false)] 
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

async fn main() -> Result<()> {
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
    // 同等于 update table_name set title = $1, description = $2, updated_at = now() where id = $3
    let query = format!("update table_name set {} where {}", set_values.join(", "), where_value.join(" AND "));
    
    let mut sql = sqlx::query::<_, QueryRow>(&query);
    if data.title.is_some() {
        sql = sql.bind(data.title);
    }
    if data.subtitle.is_some() {
        sql = sql.bind(data.subtitle);
    }
    if data.image_url.is_some() {
        sql = sql.bind(data.image_url);
    }
    if data.link_url.is_some() {
        sql = sql.bind(data.link_url);
    }
    if data.description.is_some() {
        sql = sql.bind(data.description);
    }
    
    let rows_affected = sql.bind(id).execute(pool).await?.rows_affected;
}
```

### select 查询
```rust
use sql_tool_kit::*;

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

async fn main() -> Result<()> {
    let data = QueryForm {
        keyword: Some("这是标题".to_string()),
        start_time: Some("2024/12/12".to_string()),
        end_time: None,
        ty:1,
        page_info: 0,
    };
    
    // 同等于：select title, subtitle, image_url, link_url, start_time, end_time, description, type, sort from table_name where title like $1 AND subtitle like $1 AND description like $1 AND start_time >= $2 AND ty = $3
    let query = format!("select {} from table_name where {}", QueryRow::generate_select_clause().join(", "), data.generate_where_clause().join(" AND "));

    let mut sql = sqlx::query::<_, QueryRow>(&query);
    if data.keyword.is_some() {
        sql = sql.bind(data.keyword);
    }
    if data.start_time.is_some() {
        sql = sql.bind(data.start_time);
    }
    if data.end_time.is_some() {
        sql = sql.bind(data.end_time);
    }
    let result = sql.bind(ty).fetch_all(pool).await?;
}
```

### delete 语句
```rust

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
    let data = DeleteForm {
        id: 1,
        title: None,
        start_time: Some("2025/12/12".to_string()),
        end_time: Some("2024/12/12".to_string()),
    };

    // 输出：delete table_name where id = $1 AND start_time >= $2 AND end_time <= $3
    let query = format!("delete table_name where {}", data.generate_where_clause().join(" AND "));
}
```

## 后续可能的扩展

1. 完整的 sql 语句生成
2. 优化 sqlx 的绑定值步骤