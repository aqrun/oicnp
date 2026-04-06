import { useState, useEffect } from 'react';
import { Form } from 'antd';
import { Modal } from '#src/components';
import NodeForm from '../NodeForm';
import { useCreateStore } from './useCreateStore';
import { useMemoizedFn } from 'ahooks';
import CreateSuccess from './CreateSuccess';
import { useListStore } from '../List/useListStore';
import {   NodeFieldType, DescribeCreateNodeRequestParams } from "@repo/apis";
import { useFetchCategoryList } from "#src/hooks/apis";
import { nodeApis } from "#src/api";

/**
 * 创建弹框
 */
export default function CreateModal() {
  const visible = useCreateStore(state => state.visible);
  const contentType = useCreateStore(state => state.contentType);
  const tags = useCreateStore(state => state.tags);
  const categories = useCreateStore(state => state.categories);
  const setState = useCreateStore(state => state.setState);
  const setListState = useListStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const {
    loading: categoryLoading,
    fetchCategoryList,
  } = useFetchCategoryList();

  const [form] = Form.useForm<NodeFieldType>();

  const handleOk = useMemoizedFn(async () => {
    setLoading(true);
    try {
      const values = await form.validateFields();

      const params: DescribeCreateNodeRequestParams = {
        vid: values?.vid,
        title: values?.title,
        tagVids: tags,
      };

      const res = await nodeApis.DescribeCreateNode(params);

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

  const handleTagChange = useMemoizedFn((tags: string[]) => {
    setState({
      tags,
    });
  });

  const init = useMemoizedFn(async () => {
    const requests = [
      fetchCategoryList({
        page: 1,
        pageSize: 10,
      }),
    ] as const;
    const allRes = await Promise.all(requests);
    setState({
      categories: allRes[0].categories,
    });
  });

  let content = (
    <NodeForm
      form={form}
      loading={loading}
      categoryLoading={categoryLoading}
      categories={categories}
      onTagChange={handleTagChange}
    />
  );

  if (contentType === 'success') {
    content = (
      <CreateSuccess
        title={form.getFieldValue('title')}
      />
    );
  }

  useEffect(() => {
    if (visible) {
      init();
    }
  }, [visible]);

  return (
    <Modal
      title="创建内容"
      open={visible}
      onOk={handleOk}
      onCancel={handleCancel}
      confirmLoading={false}
      okText="创建"
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
