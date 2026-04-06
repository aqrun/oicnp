import type { ReactElement } from "react";
import { Result } from 'antd';

export interface SuccessProps {
  title?: string;
}

export default function Success({
  title,
}: SuccessProps): ReactElement {
  return (
    <>
      <Result
        status="success"
        title="分类更新成功"
        subTitle={`分类 ${title} 更新成功`}
      />
    </>
  );
}
