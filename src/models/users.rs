impl UserModel {
    /// 查询用户列表
    pub async fn find_list(
        db: &DatabaseConnection,
        params: &UserFilters,
    ) -> ModelResult<(Vec<Self>, u64)> {
        let mut query = users::Entity::find();

        // 构建查询条件
        if let Some(uid) = params.uid {
            query = query.filter(users::Column::Uid.eq(uid));
        }
        if let Some(uuid) = &params.uuid {
            query = query.filter(users::Column::Uuid.eq(uuid));
        }
        if let Some(username) = &params.username {
            query = query.filter(users::Column::Username.eq(username));
        }
        if let Some(email) = &params.email {
            query = query.filter(users::Column::Email.eq(email));
        }
        if let Some(status) = params.status {
            query = query.filter(users::Column::Status.eq(status));
        }

        // 获取总数
        let total = query.clone().count(db).await?;

        // 分页查询
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(10);
        let users = query
            .order_by_desc(users::Column::CreatedAt)
            .paginate(db, page_size)
            .fetch_page(page - 1)
            .await?;

        Ok((users, total))
    }
} 