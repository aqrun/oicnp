import { useState } from 'react';
import { Form } from 'antd';
import { Modal } from '#src/components';
import CategoryForm from '../CategoryForm';
import { useCreateStore } from './useCreateStore';
import { useMemoizedFn } from 'ahooks';
import CreateSuccess from './CreateSuccess';
import { useListStore } from '../List/useListStore';
import {   CategoryModel, DescribeCreateCategoryRequestParams } from "@repo/apis";
import { categoryApis } from "#src/api";

/**
 * 创建弹框
 */
export default function CreateModal() {
  const visible = useCreateStore(state => state.visible);
  const contentType = useCreateStore(state => state.contentType);
  const setState = useCreateStore(state => state.setState);
  const setListState = useListStore(state => state.setState);

  const [loading, setLoading] = useState(false);

  const [form] = Form.useForm<CategoryModel>();

  const handleOk = useMemoizedFn(async () => {
    setLoading(true);
    try {
      const values = await form.validateFields();

      const params: DescribeCreateCategoryRequestParams = {
        catPid: Number(values?.catPid || 0),
        catVid: values?.catVid,
        catName: values?.catName,
        catDesc: values?.catDesc,
        weight: Number(values?.weight || 0),
      };

      const res = await categoryApis.DescribeCreateCategory(params);

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
    <CategoryForm
      form={form}
      loading={loading}
    />
  );

  if (contentType === 'success') {
    content = (
      <CreateSuccess
        title={form.getFieldValue('catName')}
      />
    );
  }

  return (
    <Modal
      title="创建分类"
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
