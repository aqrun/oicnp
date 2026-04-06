import dayjs from "dayjs";

import { STATIC_URI } from "#src/constants";
import { getApiUri } from "@repo/services";

/**
 * 获取链接前缀（接口或静态资源）
 */
export function getBaseUri(isStatic = false) {
	let baseUri = getApiUri();

	if (isStatic) {
		baseUri = STATIC_URI;
	}

	const validBaseUri = baseUri?.trim()?.replace(/\/$/i, "");

	return validBaseUri;
}

/**
 * 统一链接处理（自旧项目；静态资源可传 isStatic）
 */
export function r(uri: string, isStatic = false) {
	const baseUri = getBaseUri(isStatic);

	if (!uri?.trim())
		return baseUri;

	const validUri = uri
		?.trim()
		?.replace(/^\//i, "");

	if (!validUri) {
		return baseUri;
	}

	return `${baseUri}/${validUri}`;
}

/**
 * 与 `r` 相对：SPA 内路由使用带 Vite base 的路径（如 `/public/settings`）
 */
export function appRoutePath(path: string) {
	const base = (import.meta.env.BASE_URL ?? "/").replace(/\/$/, "");
	const p = path.startsWith("/") ? path : `/${path}`;
	return `${base}${p}`;
}

/**
 * 日期格式化显示
 */
export function formatDate(strDate: string): string {
	if (!strDate)
		return strDate;
	const res = dayjs(strDate).add(8, "h").format("YYYY年MM月DD日 HH:mm:ss");
	return res;
}