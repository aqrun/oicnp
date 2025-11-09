import { Result } from 'antd';

export interface SuccessProps {
  title?: string;
}

export default function Success({
  title,
}: SuccessProps): JSX.Element {
  return (
    <>
      <Result
        status="success"
        title="笔记更新成功"
        subTitle={`笔记 ${title} 更新成功`}
      />
    </>
  );
}
