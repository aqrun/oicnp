import type { LoginInfo, AuthType } from "#src/api/user/types";

import { fetchLogin, fetchLogout } from "#src/api/user";
import { useAccessStore } from "#src/store/access";
import { useTabsStore } from "#src/store/tabs";
import { useUserStore } from "#src/store/user";

import { create } from "zustand";

interface AuthAction {
	login: (loginPayload: LoginInfo) => Promise<ApiResponse<AuthType>>
	logout: () => Promise<void>
	reset: () => void
};

export const useAuthStore = create<AuthAction>()(() => ({
	login: async (loginPayload) => {
		// 登录态由服务端 cookie 维护
		const res = await fetchLogin(loginPayload);
		return res;
	},

	logout: async () => {
		await fetchLogout();
		useAuthStore.getState().reset();
	},

	reset: () => {
		useUserStore.getState().reset();
		useAccessStore.getState().reset();
		useTabsStore.getState().resetTabs();
	},
}));
