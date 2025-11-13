import React from "react";

const tabList = [
  { id: 'index', name: '秦岭群峰', href: '/climb'},
  { id: 'aotai', name: '鳌太路线', href: '/climb/aotai'},
  { id: 'oil', name: '车费估算', href: '/climb/oil'},
];

export interface TabsProps {
  tab?: string;
}

export const Tabs: React.FC<TabsProps> = ({
  tab,
}) => {
  return (
    <div
      className="text-md font-medium text-center text-gray-500 border-b border-gray-200 dark:text-gray-400 dark:border-gray-700"
    >
      <ul className="flex flex-wrap -mb-px">
        {tabList?.map((item) => {
          const cls = item?.id === tab ? 'oic-tab-item active' : 'oic-tab-item';
          return (
            <li className="me-2" key={item?.id}>
              <a
                href={item?.href}
                className={cls}
              >
                {item?.name}
              </a>
            </li>
          );
        })}
        {/* <li>
                <a className="inline-block p-4 text-gray-400 rounded-t-lg cursor-not-allowed dark:text-gray-500">Disabled</a>
            </li> */}
      </ul>
    </div>
  );
};

