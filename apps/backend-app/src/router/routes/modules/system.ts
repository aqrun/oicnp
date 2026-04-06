import type { AppRouteRecordRaw } from "#src/router/types";
import ContainerLayout from "#src/layout/container-layout";
import { system } from "#src/router/extra-info";

import { lazy } from "react";

const User = lazy(() => import("#src/pages/system/users"));
const Dept = lazy(() => import("#src/pages/system/departments"));
const Role = lazy(() => import("#src/pages/system/roles"));
const Menu = lazy(() => import("#src/pages/system/menus"));

const routes: AppRouteRecordRaw[] = [
	{
		path: "/system",
		Component: ContainerLayout,
		handle: {
			icon: "SettingOutlined",
			title: "common.menu.system",
			order: system,
			roles: ["admin"],
		},
		children: [
			{
				path: "/system/user",
				Component: User,
				handle: {
					icon: "UserOutlined",
					title: "common.menu.user",
					roles: ["admin"],
					permissions: [
						"permission:button:add",
						"permission:button:update",
						"permission:button:delete",
					],
				},
			},
			{
				path: "/system/role",
				Component: Role,
				handle: {
					icon: "TeamOutlined",
					title: "common.menu.role",
					roles: ["admin"],
					permissions: [
						"permission:button:add",
						"permission:button:update",
						"permission:button:delete",
					],
				},
			},
			{
				path: "/system/menu",
				Component: Menu,
				handle: {
					icon: "MenuOutlined",
					title: "common.menu.menu",
					roles: ["admin"],
					permissions: [
						"permission:button:add",
						"permission:button:update",
						"permission:button:delete",
					],
				},
			},
			{
				path: "/system/dept",
				Component: Dept,
				handle: {
					keepAlive: false,
					icon: "ApartmentOutlined",
					title: "common.menu.dept",
					roles: ["admin"],
					permissions: [
						"permission:button:add",
						"permission:button:update",
						"permission:button:delete",
					],
				},
			},
		],
	},
];

export default routes;
