import { useState, useEffect } from 'react';
import { Form } from 'antd';
import { Modal } from '#src/components';
import UserForm from '../UserForm';
import { useCreateStore } from './useCreateStore';
import { useMemoizedFn } from 'ahooks';
import CreateSuccess from './CreateSuccess';
import { useListStore } from '../List/useListStore';
import type { UserModel, DescribeCreateUserRequestParams } from '@repo/apis';
import { userApis } from '#src/api';
import { useFetchRoleList } from '#src/hooks/apis';

/**
 * 创建弹框
 */
export default function CreateModal() {
  const visible = useCreateStore(state => state.visible);
  const contentType = useCreateStore(state => state.contentType);
  const roleList = useCreateStore(state => state.roleList);
  const setState = useCreateStore(state => state.setState);
  const setListState = useListStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [form] = Form.useForm<UserModel>();

  const {
    fetchRoleList,
  } = useFetchRoleList();

  const handleOk = useMemoizedFn(async () => {
    setLoading(true);
    try {
      const values = await form.validateFields();

      const params: DescribeCreateUserRequestParams = {
        ...values,
        status: values?.status ? '1' : '0',
        isAdmin: values?.isAdmin ? '1' : '0',
      };

      const res = await userApis.DescribeCreateUser(params);

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
    <UserForm
      form={form}
      loading={loading}
      roleList={roleList}
    />
  );

  if (contentType === 'success') {
    content = (
      <CreateSuccess
        title={form.getFieldValue('username')}
      />
    );
  }

  const fetchPageData = useMemoizedFn(async () => {
    const res = await fetchRoleList();
    setState({
      roleList: res?.roles || [],
    });
  });

  useEffect(() => {
    fetchPageData();
  }, []);

  return (
    <Modal
      title="创建用户"
      open={visible}
      onOk={handleOk}
      onCancel={handleCancel}
      confirmLoading={false}
      okText="创建"
      cancelText={contentType !== 'success' ? '取消' : '关闭'}
      destroyOnClose
      width={640}
      hasOk={contentType !== 'success'}
      maskClosable={false}
      okButtonProps={{
        loading,
      }}
    >
      {content}
    </Modal>
  );
}
