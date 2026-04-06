import { clsx } from "clsx";

interface Props {
	style?: React.CSSProperties
	className?: string
	children: React.ReactNode
}

export function BasicPage(props: Props) {
	const { children, className, style } = props;

	return (
		<div
			id="basic-page"
			/**
			 * 1. 当 children 的高度过高，设置了 p-4 样式，就不能设置了 h-full，防止底部的 padding-bottom 不出现。
			 * 请参考 src/pages/about/index.tsx
			 *
			 * 2. 如果需要 children 的高度小于等于 basic-content 请使用 h-full
			 * 请参考 src/pages/system/role/index.tsx
			 */
			className={clsx("h-full m-4 p-4 box-border rounded-sm bg-white", className)}
			style={{ ...style }}
		>
			{
				children
			}
		</div>
	);
}
