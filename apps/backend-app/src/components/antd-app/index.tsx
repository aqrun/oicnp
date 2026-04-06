import { GlobalContext, type GlobalState } from "#src/context";
import { StaticAntd } from "#src/utils/static-antd";

import { theme as antdTheme, App, AppProps } from "antd";
import { useEffect, useMemo, type ReactNode } from "react";

import { setupAntdThemeTokensToHtml } from "./setup-antd-theme";

export interface AntdAppProps {
	children: AppProps["children"];
}

function GlobalStateProvider({ children }: { children: ReactNode }) {
	const { modal, message } = App.useApp();
	const value = useMemo<GlobalState>(
		() => ({ modal, message }),
		[modal, message],
	);

	return (
		<GlobalContext.Provider value={value}>
			{children}
		</GlobalContext.Provider>
	);
}

export function AntdApp({ children }: AntdAppProps) {
	const { token: antdTokens } = antdTheme.useToken();

	useEffect(() => {
		setupAntdThemeTokensToHtml(antdTokens);
	}, [antdTokens]);

	return (
		<App className="h-full">
			<GlobalStateProvider>
				<StaticAntd />
				{children}
			</GlobalStateProvider>
		</App>
	);
}
