import type { ReactElement } from "react";
import { Result } from 'antd';

export interface CreateSuccessProp {
  title?: string;
}

export default function CreateSuccess({
  title,
}: CreateSuccessProp): ReactElement {
  return (
    <>
      <Result
        status="success"
        title="笔记创建成功"
        subTitle={`笔记 ${title} 创建成功`}
      />
    </>
  );
}
