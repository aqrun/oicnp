import { useState, useEffect } from 'react';
import { Form, Skeleton } from 'antd';
import { Modal } from '@/components';
import NodeForm from '../NodeForm';
import { useEditStore } from './useEditStore';
import { useMemoizedFn } from 'ahooks';
import Success from './Success';
import { useListStore } from '../List/useListStore';
import {
  NodeModel,
  DescribeNodeDetail,
  DescribeNodeDetailRequestParams,
  DescribeUpdateNode,
  DescribeUpdateNodeRequestParams,
  NodeFieldType,
  useFetchNodeAll,
  useFetchCategoryList,
} from '@/services';

/**
 * 更新弹框
 */
export default function EditModal() {
  const visible = useEditStore(state => state.visible);
  const contentType = useEditStore(state => state.contentType);
  const nid = useEditStore(state => state.nid);
  const node = useEditStore(state => state.node);
  const tags = useEditStore(state => state.tags);
  const categoryList = useEditStore(state => state.categoryList);
  const setState = useEditStore(state => state.setState);
  const setListState = useListStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [initLoading, setInitLoading] = useState(true);
  const {
    fetchNodeAll,
  } = useFetchNodeAll();
  const {
    fetchCategoryList,
  } = useFetchCategoryList();

  const [form] = Form.useForm<NodeFieldType>();

  const fetchNote = useMemoizedFn(async () => {
    const params: DescribeNodeDetailRequestParams = {
      nid,
    };
    const request = [
      fetchNodeAll(params),
      fetchCategoryList({}),
    ] as const;
    const allRes = await Promise.all(request);
    const res = allRes?.[0];
    
    setState({
      node: res.detailRes?.node,
      tags: res?.tagRes?.tags,
      categories: res?.categoryRes?.categories,
      categoryList: allRes?.[1]?.categories,
    });
    console.log('res---', res);

    form.setFieldsValue({
      vid: res?.detailRes?.node?.vid,
      title: res?.detailRes?.node?.title,
      summary: res?.bodyRes?.body?.summary,
      body : res?.bodyRes?.body?.body,
      categoryIds: res?.categoryRes?.categories?.map(item => item?.catId || 0),
    });
  });

  const handleOk = useMemoizedFn(async () => {
    setLoading(true);
    try {
      const values = await form.validateFields();

      const params: DescribeUpdateNodeRequestParams = {
        nid,
        vid: values?.vid,
        title: values?.title,
      };

      const res = await DescribeUpdateNode(params);

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
          title={form.getFieldValue('title')}
        />
      );
    } else {
      return (
        <NodeForm
          form={form}
          loading={loading}
          node={node}
          categories={categoryList}
          defaultTags={tags?.map?.((item) => item?.tagVid || '')}
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
      title="编辑内容"
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
