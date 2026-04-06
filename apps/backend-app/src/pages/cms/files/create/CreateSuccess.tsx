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
        title="文件创建成功"
        subTitle={`文件 ${title} 创建成功`}
      />
    </>
  );
}
