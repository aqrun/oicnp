import type { AppRouteRecordRaw } from "#src/router/types";
import ContainerLayout from "#src/layout/container-layout";
import { cms } from "#src/router/extra-info";
import { lazy } from "react";

const Tags = lazy(() => import("#src/pages/cms/tags"));
const Posts = lazy(() => import("#src/pages/cms/posts"));
const Notes = lazy(() => import("#src/pages/cms/notes"));
const Files = lazy(() => import("#src/pages/cms/files"));
const Categories = lazy(() => import("#src/pages/cms/categories"));

const routes: AppRouteRecordRaw[] = [
  {
    path: "/cms",
    Component: ContainerLayout,
    handle: {
      icon: "ReadOutlined",
      title: "内容管理",
      order: cms,
      roles: ["admin"],
    },
    children: [
      { path: "/cms/posts", Component: Posts, handle: { icon: "FileTextOutlined", title: "文章管理", roles: ["admin"] } },
      { path: "/cms/notes", Component: Notes, handle: { icon: "ProfileOutlined", title: "笔记管理", roles: ["admin"] } },
      { path: "/cms/categories", Component: Categories, handle: { icon: "FolderOutlined", title: "分类管理", roles: ["admin"] } },
      { path: "/cms/tags", Component: Tags, handle: { icon: "TagsOutlined", title: "标签管理", roles: ["admin"] } },
      { path: "/cms/files", Component: Files, handle: { icon: "CloudUploadOutlined", title: "文件管理", roles: ["admin"] } },
    ],
  },
];

export default routes;
