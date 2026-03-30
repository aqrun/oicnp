export interface ResultItemProps {
  name: string;
  value: string | number;
  unit: string;
  extra?: string;
}

export const ResultItem: React.FC<ResultItemProps> = ({
  name,
  value,
  unit,
  extra,
}) => {
  return (
    <div className="flex items-center mt-2 flex-wrap w-8/12">
      <div className="w-4/12 text-gray-500">
        {name} 
      </div>
      <div className="w-4/12 text-gray-900 font-medium">
        <span className="text-red-700">
          {value}
        </span>
        {unit}
      </div>
      <div className="w-full text-gray-400 mt-1">
        {extra}
      </div>
    </div>
  );
};

