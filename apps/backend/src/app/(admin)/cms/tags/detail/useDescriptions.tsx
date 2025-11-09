import { DescriptionsProps } from 'antd';
import { useViewStore } from './useViewStore';

export default function useDescriptions() {
  const tag = useViewStore(state => state.tag);

  const items: DescriptionsProps['items'] = [
    {
      key: 'id',
      label: 'ID',
      children: tag?.tagId,
    },
    {
      key: 'tagVid',
      label: 'VID',
      children: tag?.tagVid,
    },
    {
      key: 'tagName',
      label: '标签名称',
      children: tag?.tagName,
    },
    {
      key: 'tagCount',
      label: '计数',
      children: tag?.tagCount,
    },
    {
      key: 'weight',
      label: '权重',
      children: tag?.weight,
    },
  ];

  return [items];
}
