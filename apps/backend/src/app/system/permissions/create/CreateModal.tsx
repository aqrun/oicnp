import { useState } from 'react';
import { Form } from 'antd';
import { Modal } from '@/components';
import PermissionForm from '../PermissionForm';
import { useCreateStore } from './useCreateStore';
import { useMemoizedFn } from 'ahooks';
import CreateSuccess from './CreateSuccess';
import { useListStore } from '../PermissionList/useListStore';
import {
  PermissionModel,
  DescribeCreatePermission,
  DescribeCreatePermissionRequestParams,
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

  const [form] = Form.useForm<PermissionModel>();

  const handleOk = useMemoizedFn(async () => {
    setLoading(true);
    try {
      const values = await form.validateFields();

      const params: DescribeCreatePermissionRequestParams = {
        vid: values?.vid,
        name: values?.name,
        weight: values?.weight ?? 0,
        remark: values?.remark,
        status: values?.status,
        pid: Number(values?.pid || 0),
      };

      const res = await DescribeCreatePermission(params);

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
    <PermissionForm
      form={form}
      loading={loading}
    />
  );

  if (contentType === 'success') {
    content = (
      <CreateSuccess
        name={form.getFieldValue('name')}
      />
    );
  }

  return (
    <Modal
      title="创建权限"
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
