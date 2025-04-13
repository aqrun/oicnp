import { useGlobalState } from '@/context';
import { useMemoizedFn } from 'ahooks';
import { ModalFuncProps } from 'antd';
import ModalFooter from '@/components/Modal/ModalFooter';
import { callFn } from '@/utils';

export interface DeleteProps extends ModalFuncProps {
  loading?: boolean;
}

export function useConfirmDelete() {
  const { modal } = useGlobalState();

  const confirmDelete = useMemoizedFn((options: DeleteProps = {}) => {
    const instance = modal.confirm({
      title: '删除',
      content: `确定删除?`,
      okType: 'danger',
      type: 'error',
      ...options,
      footer: (
        <ModalFooter
          okText="删除"
          cancelText="取消"
          onOk={() => {
            callFn(options?.onOk);
            instance.destroy();
          }}
          onCancel={() => {
            instance.destroy();
          }}
          okButtonProps={{
            color: 'danger',
            variant: 'solid',
            loading: options?.loading,
          }}
        />
      )
    });
  });

  return confirmDelete;
}