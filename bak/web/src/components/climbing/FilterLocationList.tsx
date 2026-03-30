import clsx from 'clsx';

import { usePersistFn } from '@/utils/common';

import { LocationData } from './utils';

export interface FilterLocationListProps {
  name: string;
  dataList: LocationData[];
  onClick?: (name: string) => void;
  selectedName?: string;
  /** 合法的数据列表 */
  validDataList?: string[];
}

export const FilterLocationList: React.FC<FilterLocationListProps> = ({
  name,
  dataList,
  onClick,
  selectedName,
  validDataList,
}) => {
  const clickHandle = usePersistFn((itemName: string) => {
    if (typeof onClick === 'function') {
      onClick(itemName);
    }
  });

  return (
    <div
      className="filter-location-list flex flex-row mb-4"
    >
      <div
        className="min-w-[4rem] text-gray-500 py-1"
      >
        {name}
      </div>
      <div
        className="flex flex-wrap"
      >
        {dataList?.map((item) => {
          const cls = selectedName === item?.name ? 'text-indigo-600' : '';
          const disabled = typeof validDataList !== 'undefined'
            && !validDataList?.includes(item?.name);

          return (
            <div
              key={item?.name}
              className={clsx('mr-2 py-1', {
                'cursor-pointer': !disabled,
              })}
              onClick={() => {
                if (disabled) return;
                clickHandle(item?.name);
              }}
            >
              <span
                className={clsx(cls, {
                  'text-gray-300': disabled,
                  'text-gray-700': !disabled,
                  'hover:text-indigo-600': !disabled
                })}
              >
                {item?.name}
              </span>
              <span
                className={clsx('text-sm', {
                  'text-gray-200': disabled,
                  'text-gray-400': !disabled,
                })}
              >
                (
                {item?.amount}
                )
              </span>
            </div>
          );
        })}
      </div>
    </div>
  );
};