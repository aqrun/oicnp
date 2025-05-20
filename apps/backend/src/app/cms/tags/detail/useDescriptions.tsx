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
      key: 'tagName',
      label: '标签名称',
      children: tag?.tagName,
    },
  ];

  return [items];
}
