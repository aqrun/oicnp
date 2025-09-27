import { useState, useEffect } from 'react';
import { Descriptions, Spin } from 'antd';
import { Modal } from '@/components';
import { useViewStore } from './useViewStore';
import { useMemoizedFn } from 'ahooks';
import useDescriptions from './useDescriptions';
import {
  NodeFilters,
  useFetchNodeAll,
} from '@/services';
import {
  ModelContent,
} from './index.styled';

/**
 * 查看弹框
 */
export default function ViewModal() {
  const visible = useViewStore(state => state.visible);
  const nid = useViewStore(state => state.nid);
  const setState = useViewStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [items] = useDescriptions();
  const {
    fetchNodeAll,
  } = useFetchNodeAll();

  const handleCancel = useMemoizedFn(() => {
    setState({
      visible: false,
      node: undefined,
      body: undefined,
      tags: [],
      categories: [],
    });
  });

  const fetchNode = useMemoizedFn(async () => {
    const params: NodeFilters = {
      nid,
    };
    const res = await fetchNodeAll(params);

    return res;
  });

  const fetchInitialData = useMemoizedFn(async () => {
    setLoading(true);
    const res = await fetchNode();

    setState({
      node: res?.detailRes?.node,
      body: res?.bodyRes?.body,
      tags: res?.tagRes?.tags,
      categories: res?.categoryRes?.categories,
    });
    setLoading(false);
  });

  useEffect(() => {
    if (visible) {
      fetchInitialData();
    }
  }, [visible]);

  return (
    <Modal
      title="查看内容"
      open={visible}
      onCancel={handleCancel}
      confirmLoading={false}
      cancelText={'关闭'}
      destroyOnClose
      width={1200}
      hasOk={false}
    >
      <ModelContent>
        <Spin
          spinning={loading}
        >
          <Descriptions
            items={items}
            column={2}
          />
        </Spin>
      </ModelContent>
    </Modal>
  );
}
