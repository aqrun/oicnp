import type { Options } from "ky";

import { loginPath } from "#src/router/extra-info";
import { usePreferencesStore } from "#src/store/preferences";
import ky from "ky";

import { LANG_HEADER } from "./constants";
import { handleErrorResponse } from "./error-response";
import { globalProgress } from "./global-progress";
import { goLogin } from "./go-login";

// 请求超时时间
const API_TIMEOUT = Number(import.meta.env.VITE_API_TIMEOUT) || 10000;

const defaultConfig: Options = {
	// The input argument cannot start with a slash / when using prefixUrl option.
	prefixUrl: import.meta.env?.VITE_API_BASE_URL || '/api',
	credentials: "include",
	timeout: API_TIMEOUT,
	retry: {
		// 当请求失败时，最多重试次数
		limit: 3,
	},
	hooks: {
		beforeRequest: [
			(request, options) => {
				const ignoreLoading = options.ignoreLoading;
				if (!ignoreLoading) {
					globalProgress.start();
				}
				// 语言等所有的接口都需要携带
				request.headers.set(LANG_HEADER, usePreferencesStore.getState().language);
			},
		],
		afterResponse: [
			async (request, options, response) => {
				const ignoreLoading = options.ignoreLoading;
				if (!ignoreLoading) {
					globalProgress.done();
				}
				// request error
				if (!response.ok) {
					if (response.status === 401) {
						// 如果页面的路由已经重定向到登录页，则不用重复跳转
						if (location.pathname !== loginPath) {
							goLogin();
						}
						return response;
					} else {
						return handleErrorResponse(response);
					}
				}
				// request success
				return response;
			},
		],
	},
};

export const request = ky.create(defaultConfig);
