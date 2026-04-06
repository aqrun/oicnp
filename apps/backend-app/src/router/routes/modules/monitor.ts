import type { AppRouteRecordRaw } from "#src/router/types";
import ContainerLayout from "#src/layout/container-layout";
import { monitor } from "#src/router/extra-info";
import { lazy } from "react";

const Online = lazy(() => import("#src/pages/monitor/online"));
const Operations = lazy(() => import("#src/pages/monitor/operations"));
const LoginLogs = lazy(() => import("#src/pages/monitor/login-logs"));
const Jobs = lazy(() => import("#src/pages/monitor/jobs"));
const Caches = lazy(() => import("#src/pages/monitor/caches"));

const routes: AppRouteRecordRaw[] = [
  {
    path: "/monitor",
    Component: ContainerLayout,
    handle: {
      icon: "DashboardOutlined",
      title: "系统监控",
      order: monitor,
      roles: ["admin"],
    },
    children: [
      { path: "/monitor/online", Component: Online, handle: { icon: "UserSwitchOutlined", title: "在线用户", roles: ["admin"] } },
      { path: "/monitor/operations", Component: Operations, handle: { icon: "UnorderedListOutlined", title: "操作日志", roles: ["admin"] } },
      { path: "/monitor/login-logs", Component: LoginLogs, handle: { icon: "LoginOutlined", title: "登录日志", roles: ["admin"] } },
      { path: "/monitor/jobs", Component: Jobs, handle: { icon: "ScheduleOutlined", title: "定时任务", roles: ["admin"] } },
      { path: "/monitor/caches", Component: Caches, handle: { icon: "DatabaseOutlined", title: "缓存管理", roles: ["admin"] } },
    ],
  },
];

export default routes;
