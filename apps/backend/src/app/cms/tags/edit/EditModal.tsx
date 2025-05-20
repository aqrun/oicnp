import { useState, useEffect } from 'react';
import { Form, Skeleton } from 'antd';
import { Modal } from '@/components';
import TagForm from '../TagForm';
import { useEditStore } from './useEditStore';
import { useMemoizedFn } from 'ahooks';
import Success from './Success';
import { useListStore } from '../List/useListStore';
import {
  TagModel,
  DescribeTagDetail,
  DescribeTagDetailRequestParams,
  DescribeUpdateTag,
  DescribeUpdateTagRequestParams,
} from '@/services';

/**
 * 更新弹框
 */
export default function EditModal() {
  const visible = useEditStore(state => state.visible);
  const contentType = useEditStore(state => state.contentType);
  const tagId = useEditStore(state => state.tagId);
  const tag = useEditStore(state => state.tag);
  const setState = useEditStore(state => state.setState);
  const setListState = useListStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [initLoading, setInitLoading] = useState(true);

  const [form] = Form.useForm<TagModel>();

  const fetchNote = useMemoizedFn(async () => {
    const params: DescribeTagDetailRequestParams = {
      tagId,
    };
    const res = await DescribeTagDetail(params);
    
    setState({
      tag: res.tag,
    });
    
    form.setFieldsValue({
      tagName: res?.tag?.tagName,
    });
  });

  const handleOk = useMemoizedFn(async () => {
    setLoading(true);
    try {
      const values = await form.validateFields();

      const params: DescribeUpdateTagRequestParams = {
        tagId,
        tagName: values?.tagName,
      };

      const res = await DescribeUpdateTag(params);

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
          title={form.getFieldValue('tagName')}
        />
      );
    } else {
      return (
        <TagForm
          form={form}
          loading={loading}
          tag={tag}
        />
      );
    }
  };
  const content = getContent();

  const fetchInitialData = useMemoizedFn(async () => {
    setInitLoading(true);
    const requests = [
      fetchNote(),
    ];
    await Promise.all(requests);
    
    setInitLoading(false);
  });

  useEffect(() => {
    if (visible) {
      fetchInitialData();
    }
  }, [visible]);

  return (
    <Modal
      title="编辑标签"
      open={visible}
      onOk={handleOk}
      onCancel={handleCancel}
      confirmLoading={false}
      okText="更新"
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
