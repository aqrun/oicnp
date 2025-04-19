/// 树形数据字段转换
pub fn validTreeData(item: &PermissionTreeItem) -> PermissionTreeItem {
    PermissionTreeItem {
        id: item.id,
        parent_id: item.parent_id,
        vid: item.vid.clone(),
        api: item.api.clone(),
        weight: item.weight,
        label: item.label.clone(),
        status: item.status.clone(),
        remark: item.remark.clone(),
        children: item.children.iter().map(validTreeData).collect(),
    }
} 