import { useState, useEffect } from 'react';
import { Form, Skeleton } from 'antd';
import { Modal } from '@/components';
import CategoryForm from '../CategoryForm';
import { useEditStore } from './useEditStore';
import { useMemoizedFn } from 'ahooks';
import Success from './Success';
import { useListStore } from '../List/useListStore';
import {
  CategoryModel,
  DescribeCategoryDetail,
  DescribeCategoryDetailRequestParams,
  DescribeUpdateCategory,
  DescribeUpdateCategoryRequestParams,
} from '@/services';

/**
 * 更新弹框
 */
export default function EditModal() {
  const visible = useEditStore(state => state.visible);
  const contentType = useEditStore(state => state.contentType);
  const catId = useEditStore(state => state.catId);
  const category = useEditStore(state => state.category);
  const setState = useEditStore(state => state.setState);
  const setListState = useListStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [initLoading, setInitLoading] = useState(true);

  const [form] = Form.useForm<CategoryModel>();

  const fetchNote = useMemoizedFn(async () => {
    const params: DescribeCategoryDetailRequestParams = {
      catId,
    };
    const res = await DescribeCategoryDetail(params);
    
    setState({
      category: res?.category,
    });
    
    form.setFieldsValue({
      catVid: res?.category?.catVid,
      catName: res?.category?.catName,
      weight: res?.category?.weight,
      catPid: res?.category?.catPid,
      catDesc: res?.category?.catDesc,
    });
  });

  const handleOk = useMemoizedFn(async () => {
    setLoading(true);
    try {
      const values = await form.validateFields();

      const params: DescribeUpdateCategoryRequestParams = {
        catId,
        catVid: values?.catVid,
        catName: values?.catName,
        weight: Number(values?.weight || 0),
        catPid: Number(values?.catPid || 0),
      };

      const res = await DescribeUpdateCategory(params);

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
          title={form.getFieldValue('catName')}
        />
      );
    } else {
      return (
        <CategoryForm
          form={form}
          loading={loading}
          category={category}
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
      title="编辑分类"
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
