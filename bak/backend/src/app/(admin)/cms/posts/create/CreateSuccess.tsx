import { Result } from 'antd';

export interface CreateSuccessProp {
  title?: string;
}

export default function CreateSuccess({
  title,
}: CreateSuccessProp): JSX.Element {
  return (
    <>
      <Result
        status="success"
        title="内容创建成功"
        subTitle={`内容 ${title} 创建成功`}
      />
    </>
  );
}
