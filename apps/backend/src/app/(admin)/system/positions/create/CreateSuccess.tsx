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
        title="部门创建成功"
        subTitle={`部门 ${title} 创建成功`}
      />
    </>
  );
}
