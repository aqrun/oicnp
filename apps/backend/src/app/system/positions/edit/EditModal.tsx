import { useState, useEffect } from 'react';
import { Form, Skeleton } from 'antd';
import { Modal } from '@/components';
import PositionForm from '../PositionForm';
import { useEditStore } from './useEditStore';
import { useMemoizedFn } from 'ahooks';
import Success from './Success';
import { useListStore } from '../List/useListStore';
import {
  PositionModel,
  DescribePositionDetail,
  DescribePositionDetailRequestParams,
  DescribeUpdatePosition,
  DescribeUpdatePositionRequestParams,
} from '@/services';

/**
 * 更新弹框
 */
export default function EditModal() {
  const visible = useEditStore(state => state.visible);
  const contentType = useEditStore(state => state.contentType);
  const positionId = useEditStore(state => state.positionId);
  const position = useEditStore(state => state.position);
  const setState = useEditStore(state => state.setState);
  const setListState = useListStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [initLoading, setInitLoading] = useState(true);

  const [form] = Form.useForm<PositionModel>();

  const fetchPosition = useMemoizedFn(async () => {
    const params: DescribePositionDetailRequestParams = {
      positionId: positionId,
    };
    const res = await DescribePositionDetail(params);
    
    setState({
      position: res.position,
    });
    
    form.setFieldsValue({
      name: res?.position?.name,
      vid: res?.position?.vid,
      weight: res?.position?.weight,
      remark: res?.position?.remark,
      status: res?.position?.status,
    });
  });

  const handleOk = useMemoizedFn(async () => {
    setLoading(true);
    try {
      const values = await form.validateFields();

      const params: DescribeUpdatePositionRequestParams = {
        positionId: positionId,
        name: values?.name,
        vid: values?.vid,
        weight: values?.weight,
        remark: values?.remark,
        status: values?.status,
      };

      const res = await DescribeUpdatePosition(params);

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
        <PositionForm
          form={form}
          loading={loading}
          position={position}
        />
      );
    }
  };
  const content = getContent();

  const fetchInitialData = useMemoizedFn(async () => {
    setInitLoading(true);
    await fetchPosition();
    setInitLoading(false);
  });

  useEffect(() => {
    if (visible) {
      fetchInitialData();
    }
  }, [visible]);

  return (
    <Modal
      title="编辑职位"
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
