import { useMemoizedFn } from "ahooks";
import type { ModalFuncProps } from "antd";
import React from "react";

import ModalFooter from "#src/components/Modal/ModalFooter";
import { useGlobalState } from "#src/context";
import { callFn } from "#src/utils/fn";

export interface DeleteProps extends ModalFuncProps {
	loading?: boolean;
}

export function useConfirmDelete() {
	const { modal } = useGlobalState();

	const confirmDelete = useMemoizedFn((options: DeleteProps = {}) => {
		const instance = modal.confirm({
			title: "删除",
			content: `确定删除?`,
			okType: "danger",
			type: "error",
			...options,
			footer: (
				<ModalFooter
					okText={options?.okText || "删除"}
					cancelText="取消"
					onOk={() => {
						callFn(options?.onOk as () => void);
						instance.destroy();
					}}
					onCancel={() => {
						instance.destroy();
					}}
					okButtonProps={{
						color: "danger",
						variant: "solid",
						loading: options?.loading,
					}}
				/>
			),
		});
	});

	return confirmDelete;
}
