import { DescriptionsProps } from 'antd';
import { useViewStore } from './useViewStore';

export default function useDescriptions() {
  const category = useViewStore(state => state.category);

  const items: DescriptionsProps['items'] = [
    {
      key: 'id',
      label: 'ID',
      children: category?.catId,
    },
    {
      key: 'catVid',
      label: 'VID',
      children: category?.catVid,
    },
    {
      key: 'catName',
      label: '名称',
      children: category?.catName,
    },   
    {
      key: 'weight',
      label: '权重',
      children: category?.weight,
    },
    {
      key: 'catDesc',
      label: '描述',
      children: category?.catDesc,
    },
  ];

  return [items];
}
