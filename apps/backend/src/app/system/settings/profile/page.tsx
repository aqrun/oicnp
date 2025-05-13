'use client';

import { Form, Input, Radio, Layout } from 'antd';
import { Container } from './index.styled';

export default function ProfilePage() {
  return (
    <Container className="h-full w-80">
      <Form
        layout="vertical"
        size="middle"
      >
        <Form.Item label="姓名">
          <Input />
        </Form.Item>
          <Form.Item label="邮箱">
            <Input />
          </Form.Item>
          <Form.Item label="手机号">
            <Input />
          </Form.Item>
          <Form.Item label="性别">
            <Radio.Group>
              <Radio value="male">男</Radio>
              <Radio value="female">女</Radio>
            </Radio.Group>
          </Form.Item>
      </Form>
    </Container>
  );
}
