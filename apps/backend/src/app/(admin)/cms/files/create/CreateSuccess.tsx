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
        title="文件创建成功"
        subTitle={`文件 ${title} 创建成功`}
      />
    </>
  );
}
