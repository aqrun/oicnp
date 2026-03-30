'use client';

import { useMemo,useState } from 'react';

import {
  usePersistFn,
} from '@/utils/common';

import { FilterLevelAndScene } from './FilterLevelAndScene';
import { FilterLocationList } from './FilterLocationList';
import { locations } from './locations';
import {
  Mountain,
} from './mountains';
import {
  EnumOrder,
  getLocationList,
  OrderData,
} from './utils';

export interface MountainListProps {
  mountains: Mountain[];
}

export const MountainList: React.FC<MountainListProps> = ({
  mountains,
}) => {
  const [city, setCity] = useState('');
  const [county, setCounty] = useState('');
  const [orderItems, setOrderItems] = useState<OrderData[]>([]);

  const cityList = useMemo(() => getLocationList(mountains, 'city'), [mountains]);
  const countyList = useMemo(() => getLocationList(mountains, 'county'), [mountains]);

  const subCounties = useMemo(() => {
    const province = locations?.find((item) => {
      return item?.name === '陕西省';
    });

    const targetCity = province?.children?.find((item) => {
      return item?.name === city;
    });

    const targetCountyList = targetCity?.children?.map((item) => {
      return item?.name;
    });
    return targetCountyList;
  }, [city]);

  const filterMountains = useMemo(() => {
    let validList = mountains;

    if (city) {
      validList = validList?.filter((item) => {
        return item?.city === city;
      });
    }

    if (county) {
      validList = validList?.filter((item) => {
        return item?.county === county;
      });
    }

    // 排序
    validList.sort((a, b) => {
      const ia = a?.id || 0;
      const ib = b?.id || 0;

      return ia - ib;
    });

    // 是否存在排序参数
    const hasOrderData = orderItems?.find((item) => item?.order);
    
    if (hasOrderData) {
      // 有星级的数据
      const hasLevelList = validList?.filter((item) => {
        return item?.level || item?.scene;
      });
      const noLevelList = validList?.filter((item) => {
        return !item?.level && !item?.scene;
      });

      for (let i = 0; i < orderItems?.length; i ++) {
        const orderItem = orderItems[i];
  
        if (orderItem?.order) {
          hasLevelList.sort((a, b) => {
            const ia = a?.[orderItem?.orderBy as 'level' | 'scene'] || 0;
            const ib = b?.[orderItem?.orderBy as 'level' | 'scene'] || 0;
  
            if (orderItem?.order === EnumOrder.asc) {
              return ia - ib;
            }
  
            return ib - ia;
          });
        }
      }

      validList = hasLevelList.concat(noLevelList);
    }

    return validList;
  }, [city, county, mountains, orderItems]);

  const levelChangeHandle = usePersistFn((val: OrderData[]) => {
    setOrderItems(val);
  });

  const cityClick = usePersistFn((name: string) => {
    setCity(city === name ? '' : name);
    setCounty('');
  });

  const countyClick = usePersistFn((name: string) => {
    setCounty(county === name ? '' : name);
  });

  return (
    <div className="mt-4">
      <div
        className="mountain-filters"
      >
        <FilterLocationList
          dataList={cityList}
          name="市"
          onClick={cityClick}
          selectedName={city}
        />
        <FilterLocationList
          dataList={countyList}
          name="县/区"
          onClick={countyClick}
          selectedName={county}
          validDataList={subCounties}
        />
        <FilterLevelAndScene
          onChange={levelChangeHandle}
        />
      </div>
      <div className="mount-list-content">
        <div className="list-w grid grid-cols-2 lg:grid-cols-4 gap-2">
          {filterMountains?.map((item) => {
            return (
              <MountainItem
                key={item?.id}
                mountain={item}
              />
            );
          })}
        </div>
      </div>
    </div>
  );
};

export interface MountainItemProps {
  mountain: Mountain;
}

const MountainItem: React.FC<MountainItemProps> = ({
  mountain,
}) => {
  const hasStarData = Boolean(mountain?.level || mountain?.scene);

  return (
    <div
      className="mountain-item px-2 py-1 bg-gray-100 rounded-lg hover:bg-indigo-200 hover:cursor-pointer"
    >
      <div
        className="mountain-title text-base flex items-center"
      >
        <p>
          {mountain?.name}
        </p>
        <p className="text-gray-500 text-xs ml-2">
          {mountain?.high} 米
        </p>
      </div>
      <div
        className="mountain-extras flex gap-2 text-gray-500 text-xs mt-1"
      >
        {Boolean(mountain?.level) && (
          <p>
            难度: 
            <span className="text-red-600 inline-block ml-1">{mountain?.level}</span>
            <i className='iconfont icon-star mr-1 text-[12px]' />
          </p>
        )}
        {Boolean(mountain?.scene) && (
          <p>
            风景: 
            <span className="text-red-600 inline-block ml-1">{mountain?.scene}</span>
            <i className='iconfont icon-star mr-1 text-[12px]' />
          </p>
        )}
        {!hasStarData && (
          <p>&nbsp;</p>
        )}
      </div>
      <div
        className="mountain-extras flex gap-2 text-gray-500 text-xs"
      >
        <p>
          {mountain?.city} {mountain?.county}
        </p>
      </div>
    </div>
  );
};
