import { useState } from 'react';
import { Form } from 'antd';
import { Modal } from '@/components';
import RoleForm from '../RoleForm';
import { useCreateStore } from './useCreateStore';
import { useMemoizedFn } from 'ahooks';
import CreateSuccess from './CreateSuccess';
import { useListStore } from '../RoleList/useListStore';
import {
  RoleModel,
  DescribeCreateRole,
  DescribeCreateRoleRequestParams,
} from '@/services';

/**
 * 创建弹框
 */
export default function CreateModal() {
  const visible = useCreateStore(state => state.visible);
  const contentType = useCreateStore(state => state.contentType);
  const setState = useCreateStore(state => state.setState);
  const setListState = useListStore(state => state.setState);

  const [loading, setLoading] = useState(false);

  const [form] = Form.useForm<RoleModel>();

  const handleOk = useMemoizedFn(async () => {
    setLoading(true);
    try {
      const values = await form.validateFields();

      const params: DescribeCreateRoleRequestParams = {
        vid: values?.vid,
        name: values?.name,
        weight: values?.weight ?? 0,
        remark: values?.remark,
        status: values?.status,
        permissionIds: values?.permissionIds,
      };

      const res = await DescribeCreateRole(params);

      if (res) {
        setState({
          contentType: 'success',
        });
      }
    } catch(err) {
      console.log('ERR: ', err);
    }

    setLoading(false);
  });

  const handleCancel = useMemoizedFn(() => {
    if (contentType === 'success') {
      form.resetFields();
      setListState({
        refreshToken: Date.now().toString(),
      });
    }

    setState({
      visible: false,
      contentType: 'normal',
    });
  });

  let content = (
    <RoleForm
      form={form}
      loading={loading}
    />
  );

  if (contentType === 'success') {
    content = (
      <CreateSuccess
        roleName={form.getFieldValue('name')}
      />
    );
  }

  return (
    <Modal
      title="创建角色"
      open={visible}
      onOk={handleOk}
      onCancel={handleCancel}
      confirmLoading={false}
      okText="创建"
      cancelText={contentType !== 'success' ? '取消' : '关闭'}
      destroyOnClose
      width={640}
      hasOk={contentType !== 'success'}
      okButtonProps={{
        loading,
      }}
    >
      {content}
    </Modal>
  );
}
