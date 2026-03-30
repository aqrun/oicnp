'use client';

import { Dropdown } from 'antd';
import cls from 'clsx';
import { Icon } from '@/components';
import { CLASS_PREFIX } from '@/constants';
import { asset, r } from '@/utils';
import {
  UserActionWrapper,
} from './index.styled';
import { useMemoizedFn } from 'ahooks';

export default function HeaderUser() {
  // const navigate = useNavigate();
  // const setState = useAppStore((state) => state.setState);
  // const { resetAuthState } = useAuthState();

  const handleProfileClick = (() => {
    // setState({
    //   sideMenuOpenKeys: undefined,
    //   sideMenuKeys: undefined,
    // });
    // navigate(r(''));
  });

  const handleLogout = (() => {
    // setState({
    //   mainMenuKey: undefined,
    //   sideMenuOpenKeys: undefined,
    //   sideMenuKeys: undefined,
    // });
    // resetAuthState();
    // navigate(r('/login'));
  });
  
  return (
    <Dropdown
      menu={{
        items: [
          {
            key: '1',
            icon: <Icon icon="UserOutlined" />,
            label: (
              <span onClick={handleProfileClick}>
                个人信息
              </span>
            ),
          },
          {
            key: '2',
            icon: <Icon icon="LogoutOutlined" />,
            label: (
              <span onClick={handleLogout}>
                退出登录
              </span>
            ),
          },
        ],
      }}
    >
      <UserActionWrapper
        className={cls(`${CLASS_PREFIX}-header-user-avatar-w`)}
      >
        <img
          src={asset('react.svg')}
          className="user-avator"
          alt="avator"
        />
      </UserActionWrapper>
    </Dropdown>
  );
}
