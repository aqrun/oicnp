use async_graphql::{FieldResult};
use async_graphql::connection::{query, Connection, Edge, EmptyFields};
use crate::models::{StarWarsChar};
use crate::dbs::StarWars;

///
/// 生成游标链接分页信息
///
/// 参考：
///
/// * [游标连接(Cursor Connections)](https://async-graphql.github.io/async-graphql/zh-CN/cursor_connections.html)
/// * [GRAPHQL的分页](https://graphql.bootcss.com/learn/pagination/)
/// * [GraphQL Cursor Connections Specification](https://relay.dev/graphql/connections.htm)
///
///
pub async fn query_characters(
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
    ids: &Vec<usize>
) -> FieldResult<Connection<usize, usize, EmptyFields, EmptyFields>> {
    query(
        after,
        before,
        first,
        last,
        |after, before, first, last| async move {
            let mut start = 0usize;
            let mut end = ids.len();

            // 如果有 after 参数 更新 start
            if let Some(after) = after {
                if after >= ids.len() {
                    return Ok(Connection::new(false, false));
                }
                start = after + 1;
            }

            // 如果有 before 参数 更新 end
            if let Some(before) = before {
                if before == 0 {
                    return Ok(Connection::new(false, false));
                }
                end = before;
            }

            let mut slice = &ids[start..end];

            // 如果有 first 参数
            if let Some(first) = first {
                // 更新切片为 first 参数指定大小的切片 最大是整个切片 从前向后取
                slice = &slice[..first.min(slice.len())];
                end -= first.min(slice.len());
            } else if let Some(last) = last {
                // 如果指定了 last 参数
                // 更新切片 从后向前取 last 个数据
                slice = &slice[slice.len() - last.min(slice.len())..];
                start = end - last.min(slice.len());
            }

            let mut connection = Connection::new(
                start > 0,
                end < ids.len()
            );
            connection.append(
                slice
                    .iter()
                    .enumerate()
                    .map(|(idx, item)| {
                        Edge::new(start + idx, *item)
                    }),
            );
            Ok(connection)
        },
    )
        .await
}