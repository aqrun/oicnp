import type { Options } from "ky";
import { goLogin } from "./go-login";

/**
 * 兼容旧调用的占位实现。
 * 认证改为服务端 cookie 后，前端不再负责 refresh token。
 *
 * @param request 请求对象
 * @param options 请求选项
 * @param refreshToken 刷新token
 * @throws 总是抛错并跳转登录页
 */
export async function refreshTokenAndRetry(_request: Request, _options: Options, _refreshToken: string) {
	goLogin();
	throw new Error("Refresh token flow is disabled in cookie-based auth mode.");
}
