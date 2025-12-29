import { ToolItem } from '@/content/tools';
import Image from 'next/image';

export interface ToolItemProps {
  record: ToolItem;
}

export default function ToolItemWidget({
  record,
}: ToolItemProps): JSX.Element {
  return (
    <div className="tool-item-widget">
      <a
        className="item-inner"
        target="_blank"
        href={`/tool/view?target=${encodeURIComponent(record?.url || '')}`}
      >
        <div className="item-logo-w">
          {record?.logo ? (
            <Image src={record?.logo} alt={record?.name} width="20" />
          ) : (
            <div className="item-logo-text">{record?.name?.slice(0, 2)?.toUpperCase()}</div>
          )}
        </div>
        <div className="item-content-w">
          <div className="item-name">{record?.name}</div>
          <div className="item-description">{record?.description}</div>
        </div>
      </a>
    </div>
  );
}