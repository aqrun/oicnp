import {
  PermissionModel,
} from '@/services';

/**
 * 将列表转换回树形结构
 * @param list 列表数据
 * @returns 树形结构
 */
export function convertPermissionListToTree(list: PermissionModel[]): PermissionModel[] {
  const map: Record<string | number, PermissionModel> = {};
  const tree: PermissionModel[] = [];
  
  // 创建节点映射
  list.forEach(item => {
    map[item.permissionId] = {
      ...item,
    };
  });
  
  // 构建树形结构
  list.forEach(item => {
    const node = map[item.permissionId];

    if (item.pid === '0' || !item?.pid) {
      tree.push(node);
    } else {
      const parent = map?.[item?.pid];

      if (parent) {
        parent.children = parent?.children || [];
        parent.children.push(node);
      }
    }
  });
  
  return tree;
} 