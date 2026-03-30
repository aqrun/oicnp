import {
  Mountain,
} from './mountains';

export enum EnumOrder {
  asc = 'asc',
  desc = 'desc',
}

export interface OrderData {
  orderBy: string;
  order?: EnumOrder;
}

export interface LocationData {
  name: string;
  amount: number;
}

/**
 * 省分数据
 */
export const getLocationList = (
  mountains: Mountain[],
  locationType: 'province' | 'city' | 'county',
) => {
  const locationObj = mountains?.reduce((obj: Record<string, number>, n) => {
    const newObj = obj;

    const location = n?.[locationType];
    const num = obj?.[location] || 0;

    newObj[location] = num + 1;

    return newObj;
  }, {});

  const data = Object.keys(locationObj)?.map((item) => {
    const dataItem: LocationData = {
      name: item,
      amount: locationObj[item] || 0,
    };
    return dataItem;
  });

  data.sort((a, b) => {
    const ia = a?.amount;
    const ib = b?.amount;

    if (ia > ib) {
      return -1;
    } else if (ia < ib) {
      return 1;
    }
    return 0;
  });

  return data;
};
