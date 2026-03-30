import clsx from 'clsx';
import { useState } from 'react';

import { EnumOrder,OrderData } from './utils';

export interface FilterLevelAndSceneProps {
  onChange?: (item: OrderData[]) => void;
}

export const FilterLevelAndScene: React.FC<FilterLevelAndSceneProps> = ({
  onChange,
}) => {
  const [level, setLevel] = useState<EnumOrder | undefined>(undefined);
  const [scene, setScene] = useState<EnumOrder | undefined>( undefined);

  return (
    <div
      className="filter-location-list flex flex-row mb-4"
    >
      <div
        className="min-w-[4rem] text-gray-500 py-1"
      >
        排序
      </div>
      <div
        className="flex flex-wrap"
      >
        <div
          className={clsx('mr-2 py-1 cursor-pointer text-gray-300', {
            'text-gray-700': level === EnumOrder.desc,
          })}
          onClick={() => {
            const val = level === EnumOrder.desc ? undefined : EnumOrder.desc;
            setLevel(val);

            if (typeof onChange !== 'undefined') {
              onChange([{
                orderBy: 'level',
                order: val,
              }, {
                orderBy: 'scene',
                order: scene,
              }]);
            }
          }}
        >
          难度降序
        </div>
        <div
          className={clsx('mr-2 py-1 cursor-pointer text-gray-300', {
            'text-gray-700': level === EnumOrder.asc,
          })}
          onClick={() => {
            const val = level === EnumOrder.asc ? undefined : EnumOrder.asc;
            setLevel(val);

            if (typeof onChange !== 'undefined') {
              onChange([{
                orderBy: 'level',
                order: val,
              }, {
                orderBy: 'scene',
                order: scene,
              }]);
            }
          }}
        >
          难度升序
        </div>
        <div
          className={clsx('mr-2 py-1 cursor-pointer text-gray-300', {
            'text-gray-700': scene === EnumOrder.desc,
          })}
          onClick={() => {
            const val = scene === EnumOrder.desc ? undefined : EnumOrder.desc;
            setScene(val);

            if (typeof onChange !== 'undefined') {
              onChange([{
                orderBy: 'level',
                order: level,
              }, {
                orderBy: 'scene',
                order: val,
              }]);
            }
          }}
        >
          风景降序
        </div>
        <div
          className={clsx('mr-2 py-1 cursor-pointer text-gray-300', {
            'text-gray-700': scene === EnumOrder.asc,
          })}
          onClick={() => {
            const val = scene === EnumOrder.asc ? undefined : EnumOrder.asc;
            setScene(val);

            if (typeof onChange !== 'undefined') {
              onChange([{
                orderBy: 'level',
                order: level,
              }, {
                orderBy: 'scene',
                order: val,
              }]);
            }
          }}
        >
          风景升序
        </div>
        
      </div>
    </div>
  );
};

