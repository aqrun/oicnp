'use client';

import { useState, useMemo } from 'react';
import { ChapterModel } from '@repo/apis/client';
import clsx from 'clsx';

export interface ChapterTreeModel extends ChapterModel {
  children?: ChapterTreeModel[];
}

export interface ChapterPoetryDetailProps {
  chapters?: ChapterModel[];
}

export default function ChapterPoetryDetail({ chapters }: ChapterPoetryDetailProps) {
  const [chapter, setChapter] = useState<ChapterModel>(chapters?.[0]!);

  // 遍历数据 根据pid转为树状结构 添加 children 保存子节点
  const treeChapters = useMemo(() => {
    if (!chapters || chapters.length === 0) {
      return [];
    }

    const map: Record<number, ChapterTreeModel> = {};
    const tree: ChapterTreeModel[] = [];

    // 创建节点映射，初始化每个节点并添加 children 属性
    chapters.forEach((item) => {
      if (item.id !== undefined) {
        map[item.id] = {
          ...item,
          children: [],
        };
      }
    });

    // 构建树形结构
    chapters.forEach((item) => {
      if (item.id === undefined) return;

      const node = map[item.id];
      if (!node) return;

      // 如果没有 pid 或者 pid 为 0，则为根节点
      if (item.pid === undefined || item.pid === 0) {
        tree.push(node);
      } else {
        // 否则找到父节点，将当前节点添加到父节点的 children 中
        const parent = map[item.pid];
        if (parent) {
          parent.children = parent.children || [];
          parent.children.push(node);
        }
      }
    });

    return tree;
  }, [chapters]);

  return (
    <div className="flex flex-row gap-4">
      <div className="chapter-title-list min-w-52 border-r border-gray-200">
        <div
          className="p-2 mr-2 mb-4 rounded-md bg-gray-100"
        >
          <h2 className="text-lg font-bold">章节目录</h2>
        </div>
        <div className="max-h-[calc(100vh-200px)] overflow-auto">
          {treeChapters?.map((item) => {
            return (
              <div key={item.id}>
                <a
                  className={clsx("block p-2 hover:bg-gray-200 rounded-md mr-2 cursor-pointer", {
                    'text-purple-700': chapter.id === item.id,
                  })}
                  onClick={() => setChapter(item)}
                >
                  {item.title}
                </a>

                {item.children?.map((child) => {
                  return (
                    <div key={child.id}>
                      <a
                        className={clsx("block p-2 hover:bg-gray-200 rounded-md mr-2 cursor-pointer ml-4", {
                          'text-purple-700': chapter.id === child.id,
                        })}
                        onClick={() => setChapter(child)}
                      >
                        {child.title}
                      </a>
                    </div>
                  );
                })}
              </div>
            );
          })}
        </div>
      </div>
      <div className="chapter-content">
        <article id={`chapter-${chapter?.id}`}>
          {chapter?.content?.split('\n').map((line, index) => (
            <p key={index} className="mt-4">{line}</p>
          ))}
        </article>
      </div>
    </div>
  );
}