import { useState, useEffect } from 'react';
import { Form, Skeleton } from 'antd';
import { Modal } from '#src/components';
import DepartmentForm from '../DepartmentForm';
import { useEditStore } from './useEditStore';
import { useMemoizedFn } from 'ahooks';
import Success from './Success';
import { useListStore } from '../List/useListStore';
import type {
  DepartmentModel,
  DescribeDepartmentDetailRequestParams,
  DescribeUpdateDepartmentRequestParams,
} from '@repo/apis';
import { departmentApis } from '#src/api';

/**
 * 更新弹框
 */
export default function EditModal() {
  const visible = useEditStore(state => state.visible);
  const contentType = useEditStore(state => state.contentType);
  const departmentId = useEditStore(state => state.departmentId);
  const department = useEditStore(state => state.department);
  const setState = useEditStore(state => state.setState);
  const setListState = useListStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [initLoading, setInitLoading] = useState(true);

  const [form] = Form.useForm<DepartmentModel>();

  const fetchDepartment = useMemoizedFn(async () => {
    const params: DescribeDepartmentDetailRequestParams = {
      id: departmentId,
    };
    const res = await departmentApis.DescribeDepartmentDetail(params);
    
    setState({
      department: res.department,
    });
    
    form.setFieldsValue({
      name: res?.department?.name,
      vid: res?.department?.vid,
      weight: res?.department?.weight,
      leader: res?.department?.leader,
      phone: res?.department?.phone,
      email: res?.department?.email,
      status: res?.department?.status,
    });
  });

  const handleOk = useMemoizedFn(async () => {
    setLoading(true);
    try {
      const values = await form.validateFields();

      const params: DescribeUpdateDepartmentRequestParams = {
        id: departmentId,
        name: values?.name,
        vid: values?.vid,
        weight: values?.weight,
        leader: values?.leader,
        phone: values?.phone,
        email: values?.email,
        status: values?.status,
      };

      const res = await departmentApis.DescribeUpdateDepartment(params);

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
          title={form.getFieldValue('name')}
        />
      );
    } else {
      return (
        <DepartmentForm
          form={form}
          loading={loading}
          department={department}
        />
      );
    }
  };
  const content = getContent();

  const fetchInitialData = useMemoizedFn(async () => {
    setInitLoading(true);
    await fetchDepartment();
    setInitLoading(false);
  });

  useEffect(() => {
    if (visible) {
      fetchInitialData();
    }
  }, [visible]);

  return (
    <Modal
      title="编辑部门"
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
