/**
 * 样式前缀（与 bak 项目一致，供迁移的列表/筛选等组件使用）
 */
export const CLASS_PREFIX = "oic";

export const ANT_PREFIX = "ant";

export const LAYOUT_HEADER_HEIGHT = 64;

/** 静态资源基址（Vite 环境无 NEXT_PUBLIC_*，需要时可改用 import.meta.env） */
export const STATIC_URI
	= typeof import.meta !== "undefined" && import.meta.env?.VITE_STATIC_URI
		? String(import.meta.env.VITE_STATIC_URI)
		: "http://static.oicnp.my/";

export const SESSION_ID = "SESSIONID";
