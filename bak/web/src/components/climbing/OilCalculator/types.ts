

export enum EnumOilName {
  highwayCost = 'highwayCost',
  totalRoads = 'totalRoads',
  oilPrice = 'oilPrice',
  oilPerHundred = 'oilPerHundred',
  extraCost = 'extraCost',
  people = 'people',
}

export interface OilFormData {
  /** 高速费 */
  highwayCost: number;
  /** 总里程 */
  totalRoads: number;
  /** 油价 */
  oilPrice: number;
  /** 百公时油耗 */
  oilPerHundred: number;
  /** 额外附加费用 */
  extraCost: number;
  /** 乘坐人数 */
  people: number;
}

export const OilFormDataDefaultValue: OilFormData = {
  highwayCost: 20,
  totalRoads: 100,
  oilPrice: 7,
  oilPerHundred: 8,
  extraCost: 0,
  people: 1,
}

export const getData = (data: number | undefined, defaultValue = 0) => {
  if (!data) return defaultValue || 0;
  return Number(data) || 0;
};
