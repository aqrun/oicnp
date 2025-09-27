import { DescriptionsProps, Tag } from 'antd';
import { useViewStore } from './useViewStore';
import { formatDate } from '@/utils';
import MDEditor from '@uiw/react-md-editor';

export default function useDescriptions() {
  const node = useViewStore(state => state.node);
  const body = useViewStore(state => state.body);
  const tags = useViewStore(state => state.tags);
  const categories = useViewStore(state => state.categories);

  const items: DescriptionsProps['items'] = [
    {
      key: 'id',
      label: 'ID',
      children: node?.nid,
    },
    {
      key: 'uuid',
      label: 'UUID',
      children: node?.uuid,
    },
    {
      key: 'vid',
      label: '标识',
      children: node?.vid,
    },
    {
      key: 'title',
      label: '标题',
      children: node?.title,
    },
    {
      key: 'category',
      label: '分类',
      children: (
        <div>
          {categories?.map(item => {
            return (
              <div key={item?.catId}>
                {item?.catName}
              </div>
            );
          })}
        </div>
      ),
    },
    {
      key: 'tags',
      label: '标签',
      children: (
        <div>
          {tags?.map(item => {
            return (
              <Tag key={item?.tagId}>
                {item?.tagName}
              </Tag>
            );
          })}
        </div>
      ),
    },
    {
      key: 'viewed',
      label: '浏览量',
      children: node?.viewed,
    },
    {
      key: 'publishedAt',
      label: '发布时间',
      children: node?.publishedAt,
    },
    {
      key: 'createdAt',
      label: '创建时间',
      children: formatDate(node?.createdAt || ''),
      span: 2
    },
    {
      key: 'summary',
      label: '摘要',
      children: body?.summary,
      span: 2,
    },
    {
      key: 'body',
      label: '内容',
      children: (
        <div className="oic-post-content">
          <MDEditor.Markdown
            source={body?.body}
            style={{
              whiteSpace: 'pre-wrap',
              lineHeight: '1',
            }}
          />
        </div>
      ),
    },
  ];

  return [items];
}
