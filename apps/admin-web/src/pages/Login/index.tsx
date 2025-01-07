import { useNavigate } from 'react-router';
import { Form, Input, Button, Checkbox } from 'antd';
import { CLASS_PREFIX } from '~/constants';
import { useMutation } from '@tanstack/react-query';
import cls from 'clsx';
import { r } from '~/utils';
import { fetchLogin, FetchAuthLoginRequestParams } from '~/api';
import { useMemoizedFn } from 'ahooks';
import { useGlobalState } from '~/context';
import { useAppStore } from '~/stores';
import { Container } from './index.styled';

export interface FormValues {
  username: string;
  password: string;
  remember?: boolean;
}

export default function Login() {
  const {
    setAuthState,
  } = useGlobalState();
  const navigate = useNavigate();
  const setAppState = useAppStore((state) => state.setState);
  const [form] = Form.useForm<FormValues>();

  const login = useMutation({
    mutationFn: (params: FetchAuthLoginRequestParams) => {
      return fetchLogin(params);
    },
  });

  const handleSubmit = useMemoizedFn(async () => {
    const values = form.getFieldsValue();
    const res = await login.mutateAsync({
      username: values?.username,
      password: values?.password,
      remember: Boolean(values?.remember),
    });

    console.log('login res', res, 'r()', r(''));

    if (res) {
      let expireTime = 0;

      if (values?.remember) {
        // 7 天过期时间
        expireTime = Date.now() + (7 * 24 * 60 * 60 * 1000);
        // 30 秒测试
        // expireTime = Date.now() + (30 * 1000);
      }

      setAuthState!({
        username: res?.username || '',
        token: res?.token || '',
        uuid: res?.uuid || '',
        expireTime,
        remember: values?.remember,
      });

      setAppState({
        sideMenuOpenKeys: undefined,
        sideMenuKeys: undefined,
      });
      navigate(r(''));
    }
  });
  
  return (
    <Container>
      <Form
        form={form}
        className={cls(`${CLASS_PREFIX}-login-form`)}
        initialValues={{
          username: 'guest',
          password: '123456',
        }}
        onFinish={handleSubmit}
      >
        <h2>OICNP ADMIN</h2>
        <Form.Item
          name="username"
          rules={[
            {
              required: true,
              message: '请输入正确的用户名',
            },
          ]}        
        >
          <Input
            placeholder="用户名"
          />
        </Form.Item>
        <Form.Item
          name="password"
          rules={[
            {
              required: true,
              message: '请输入正确的密码',
            },
          ]}        
        >
          <Input
            type="password"
            placeholder="密码"
          />
        </Form.Item>
        <Form.Item name="remember" valuePropName="checked">
          <Checkbox>
            记住我
          </Checkbox>
        </Form.Item>
        <Form.Item>
          <Button
            htmlType="submit"
            type="primary"
            className={cls(`${CLASS_PREFIX}-btn-login`)}
            loading={login?.isPending}
          >
            登录
          </Button>
        </Form.Item>
      </Form>
    </Container>
  );
}
