import { useMemoizedFn } from 'ahooks';

import { OilFormData } from './types';

export interface InputItemProps {
  name: string;
  value: number;
  label?: string;
  placeHolder?: string;
  extra?: string;
  unit?: string;
  onChange?: (data: Partial<OilFormData>) => void;
}

export const InputItem: React.FC<InputItemProps> = ({
  name,
  value,
  label,
  placeHolder,
  extra,
  unit,
  onChange,
}) => {
  const onChangeHandle = useMemoizedFn((e: React.ChangeEvent<HTMLInputElement>) => {
    const val = e?.target?.value;

    if (typeof onChange !== 'undefined') {
      onChange({
        [name]: val,
      });
    }
  });

  return (
    <label className="flex items-start flex-col w-5/12 mr-2 mb-4">
      <span className="mb-1 text-gray-500 flex">
        <span>
          {label}
        </span>
        <span className="block ml-2 text-slate-400">
          ({unit})
        </span>
      </span>
      <span className="mt-1 flex items-center w-full">
        <input
          type="number"
          name={name}
          placeholder={placeHolder}
          className="block w-full rounded-md shadow-sm focus:ring focus:ring-opacity-75 focus:dark:ring-violet-600 dark:bg-gray-100"
          onChange={onChangeHandle}
          value={value}
        />
      </span>
      {Boolean(extra) && (
        <span className="text-gray-400 mt-2 font-normal text-sm">
          {extra}
        </span>
      )}
    </label>
  );
};

