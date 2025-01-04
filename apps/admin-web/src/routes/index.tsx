import { lazy } from 'react';
import { RouteObject } from 'react-router';
import MainLayout from './MainLayout';
import AppRoot from './AppRoot';
import { r } from '~/utils';

import LoginPage from '~/pages/Login';
import Dashboard from '~/pages/Dashboard';

const UsersPage = lazy(() => import('~/pages/Users'));
const RolesPage = lazy(() => import('~/pages/Roles'));
const PermissionsPage = lazy(() => import('~/pages/Permissions'));

export const routeList: RouteObject[] = [
  {
    path: r('login'),
    element: <LoginPage />,
  },
  {
    path: r('/'),
    element: <MainLayout />,
    children: [
      {
        index: true,
        element: <Dashboard />,
      },
      {
        path: r('dashboard'),
        element: <Dashboard />,
      },
      {
        path: r('users'),
        children: [
          {
            index: true,
            element: <UsersPage />,
          },
          {
            path: 'list',
            element: <UsersPage />,
          }
        ],
      },
      {
        path: r('roles'),
        children: [
          {
            index: true,
            element: <RolesPage />,
          },
          {
            path: 'list',
            element: <RolesPage />,
          },
        ],
      },
      {
        path: r('permissions'),
        children: [
          {
            index: true,
            element: <PermissionsPage />,
          },
          {
            path: 'list',
            element: <PermissionsPage />,
          },
        ],
      },
      {
        path: r('cms'),
        children: [
          {
            index: true,
            element: 'posts',
          },
          {
            path: 'posts',
            children: [
              {
                path: 'list',
                element: 'posts',
              },
            ],
          },
          {
            path: 'categories',
            element: 'Categories',
          },
          {
            path: 'tags',
            element: 'tags',
          },
        ],
      },
      {
        path: r('settings'),
        element: <>settings</>,
      },
    ],
  }
];

export const allRoutes: RouteObject[] = [
  {
    path: r('/'),
    element: <AppRoot />,
    children: routeList,
  },
];
