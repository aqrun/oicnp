import { DescriptionsProps } from 'antd';
import { useViewStore } from './useViewStore';
import { formatDate } from '@/utils';
import MDEditor from '@uiw/react-md-editor';
import { MdContainer } from './index.styled';

export default function useDescriptions() {
  const note = useViewStore(state => state.note);

  const items: DescriptionsProps['items'] = [
    {
      key: 'id',
      label: 'ID',
      children: note?.id,
    },
    {
      key: 'title',
      label: '标题',
      children: note?.title,
    },
    {
      key: 'createdAt',
      label: '创建时间',
      children: note?.createdAt ? formatDate(note?.createdAt) : '-',
    },
    {
      key: 'updatedAt',
      label: '更新时间',
      children: note?.updatedAt ? formatDate(note?.updatedAt) : '-',
    },
    {
      key: 'content',
      label: '内容',
      children: (
        <MdContainer>
          <MDEditor.Markdown
            source={note?.content}
            style={{
              whiteSpace: 'pre-wrap',
              lineHeight: '1',
            }}
          />
        </MdContainer>
      ),
      span: 24,
    },
  ];

  return [items];
}
