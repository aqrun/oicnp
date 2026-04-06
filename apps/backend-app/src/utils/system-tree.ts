import type { CategoryModel, MenuModel, PermissionModel } from "@repo/apis";

/**
 * 将权限列表转为树（自 bak/backend/src/utils/tree.ts）
 */
export function convertPermissionListToTree(list: PermissionModel[]): PermissionModel[] {
	const map: Record<string | number, PermissionModel> = {};
	const tree: PermissionModel[] = [];

	list.forEach((item) => {
		map[item.permissionId!] = {
			...item,
		};
	});

	list.forEach((item) => {
		const node = map[item.permissionId!];

		if (`${item.pid}` === "0" || !item?.pid) {
			tree.push(node);
		}
		else {
			const parent = map?.[item?.pid];

			if (parent) {
				parent.children = parent?.children || [];
				parent.children.push(node);
			}
		}
	});

	return tree;
}

/**
 * 将菜单列表转为树
 */
export function convertMenuListToTree(list: MenuModel[]): MenuModel[] {
	const map: Record<string | number, MenuModel> = {};
	const tree: MenuModel[] = [];

	list.forEach((item) => {
		map[item.id] = {
			...item,
		};
	});

	list.forEach((item) => {
		const node = map[item.id];

		if (`${item.pid}` === "0" || !item?.pid) {
			tree.push(node);
		}
		else {
			const parent = map?.[item?.pid];

			if (parent) {
				parent.children = parent?.children || [];
				parent.children.push(node);
			}
		}
	});

	return tree;
}

/**
 * 将分类列表转为树
 */
export function convertCategoryListToTree(list: CategoryModel[]): CategoryModel[] {
	const map: Record<string | number, CategoryModel> = {};
	const tree: CategoryModel[] = [];

	list.forEach((item) => {
		map[item.catId!] = {
			...item,
		};
	});

	list.forEach((item) => {
		const node = map[item.catId!];

		if (`${item.catPid}` === "0" || !item?.catPid) {
			tree.push(node);
		}
		else {
			const parent = map?.[item?.catPid];

			if (parent) {
				parent.children = parent?.children || [];
				parent.children.push(node);
			}
		}
	});

	return tree;
}
