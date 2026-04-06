'use client';

import { Tag } from 'antd';
import { TableProps} from 'antd';
import TableActions from './TableActions';
import { CLASS_PREFIX } from '#src/constants';
import {   NodeModel, CategoryModel, TagModel } from "@repo/apis";
import { formatDate } from '#src/utils';

export default function useColumns() {
  const columns: TableProps<NodeModel>['columns'] = [
    {
      key: 'nid',
      title: 'ID',
      dataIndex: 'nid',
      width: 60,
    },
    {
      key: 'vid',
      title: 'VID',
      dataIndex: 'vid',
      width: 200,
    },
    {
      key: 'title',
      title: '标题',
      dataIndex: 'title',
      width: 260,
    },
    {
      key: 'category',
      title: '分类',
      dataIndex: 'categories',
      width: 120,
      render: (value: CategoryModel[]) => {
        return value?.[0]?.catName;
      }
    },
    {
      key: 'tags',
      title: '标签',
      dataIndex: 'tags',
      width: 220,
      render: (value: TagModel[]) => {
        return (
          <div>
            {value?.map((tag) => (
              <Tag
                key={tag.tagId}
                className={`${CLASS_PREFIX}-list-tag`}
              >
                {tag.tagName}
              </Tag>
            ))}
          </div>
        );
      }
    },
    {
      key: 'viewed',
      title: '浏览量',
      dataIndex: 'viewed',
      width: 80,
    },
    {
      key: 'publishedAt',
      title: '发布时间',
      dataIndex: 'publishedAt',
      width: 200,
      render: (value: string) => {
        return formatDate(value);
      }
    },
    {
      key: 'createdBy',
      title: '创建者',
      dataIndex: 'authorNickname',
      width: 200,
    },
    {
      key: 'createdAt',
      title: '创建时间',
      dataIndex: 'createdAt',
      width: 200,
      render: (value: string) => {
        return formatDate(value);
      }
    },
    {
      key: 'empty',
      title: null,
      dataIndex: 'nid',
      render: () => {
        return <></>;
      }
    },
    {
      key: 'action',
      title: '操作',
      fixed: 'right',
      width: 200,
      dataIndex: 'nid',
      render: (value: string, record: NodeModel) => {
        return (
          <TableActions
            record={record}
          />
        );
      }
    }
  ];

  return columns;
}