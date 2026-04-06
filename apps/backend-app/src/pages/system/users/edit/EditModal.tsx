import { useState, useEffect } from 'react';
import { Form, Skeleton } from 'antd';
import { Modal } from '#src/components';
import UserForm from '../UserForm';
import { useEditStore } from './useEditStore';
import { useCreateStore } from '../create/useCreateStore';
import { useMemoizedFn } from 'ahooks';
import Success from './Success';
import { useListStore } from '../List/useListStore';
import type { UserModel, DescribeUpdateUserRequestParams } from '@repo/apis';
import { userApis } from '#src/api';
import { useFetchRoleList, useFetchUserRoles, useFetchUser } from '#src/hooks/apis';

/**
 * 更新弹框
 */
export default function EditModal() {
  const visible = useEditStore(state => state.visible);
  const contentType = useEditStore(state => state.contentType);
  const uid = useEditStore(state => state.uid);
  const user = useEditStore(state => state.user);
  const setState = useEditStore(state => state.setState);
  const roleList = useCreateStore(state => state.roleList);
  const setCreateState = useCreateStore(state => state.setState);
  const setListState = useListStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [initLoading, setInitLoading] = useState(true);

  const [form] = Form.useForm<UserModel>();
  const {
    fetchUserRoles,
  } = useFetchUserRoles();
  const {
    fetchRoleList,
  } = useFetchRoleList();
  const {
    fetchUser,
  } = useFetchUser();

  const handleOk = useMemoizedFn(async () => {
    setLoading(true);
    try {
      const values = await form.validateFields();

      const params: DescribeUpdateUserRequestParams = {
        ...values,
        status: values?.status ? '1' : '0',
        isAdmin: values?.isAdmin ? '1' : '0',
      };

      const res = await userApis.DescribeUpdateUser(params);

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

  const getContent = () => {
    if (initLoading) {
      return (
        <Skeleton active />
      );
    } else if (contentType === 'success') {
      return (
        <Success
          title={form.getFieldValue('username')}
        />
      );
    } else {
      return (
        <UserForm
          form={form}
          loading={loading}
          user={user}
          roleList={roleList}
        />
      );
    }
  };
  const content = getContent();

  const fetchInitialData = useMemoizedFn(async () => {
    setInitLoading(true);
    const userRes = await fetchUser({ uid: Number(uid) });
    const userRoleRes = await fetchUserRoles({ uid: user?.uid });
    const roleListRes = await fetchRoleList();
    setState({
      user: userRes?.user,
      userRoles: userRoleRes?.roles || [],
    });
    setCreateState({
      roleList: roleListRes?.roles,
    });

    form.setFieldsValue({
      username: userRes?.user?.username,
      nickname: userRes?.user?.nickname,
      email: userRes?.user?.email,
      status: userRes?.user?.status,
      isAdmin: userRes?.user?.isAdmin,
      roleId: userRes?.user?.roleId,
      phone: userRes?.user?.phone,
      roles: userRes?.user?.roles,
      dptId: userRes?.user?.dptId,
      remark: userRes?.user?.remark,
    });

    setInitLoading(false);
  });

  useEffect(() => {
    if (visible) {
      fetchInitialData();
    }
  }, [visible]);

  return (
    <Modal
      title="编辑用户"
      open={visible}
      onOk={handleOk}
      onCancel={handleCancel}
      confirmLoading={false}
      okText="更新"
      cancelText={contentType !== 'success' ? '取消' : '关闭'}
      destroyOnClose
      width={840}
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
