import { Button, Modal } from 'antd';
import RoleForm from '../RoleForm';
import { useCreateStore } from './useCreateStore';
import { useMemoizedFn } from 'ahooks';

export default function CreateModal() {
  const visible = useCreateStore(state => state.visible);
  const setState = useCreateStore(state => state.setState);

  const handleOk = useMemoizedFn(() => {
    setState({
      visible: false,
    });
  });

  const handleCancel = useMemoizedFn(() => {
    setState({
      visible: false,
    });
  });

  return (
    <Modal
      title="创建角色"
      open={visible}
      onOk={handleOk}
      onCancel={handleCancel}
      confirmLoading={false}
      okText="创建"
      cancelText="取消"
      destroyOnClose
      width={640}
    >
      <RoleForm
      />
    </Modal>
  );
}
