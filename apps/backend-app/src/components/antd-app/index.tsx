import { StaticAntd } from "#src/utils/static-antd";

import { theme as antdTheme, App, AppProps } from "antd";
import { useEffect } from "react";

import { setupAntdThemeTokensToHtml } from "./setup-antd-theme";

export interface AntdAppProps {
	children: AppProps['children']
}

export function AntdApp({ children }: AntdAppProps) {
	const { token: antdTokens } = antdTheme.useToken();

	useEffect(() => {
		/* 打印查看支持的 token */
		// console.log("antdTokens", antdTokens);
		setupAntdThemeTokensToHtml(antdTokens);
	}, [antdTokens]);

	return (
		<App className="h-full">
			<StaticAntd />
			{children}
		</App>
	);
}
