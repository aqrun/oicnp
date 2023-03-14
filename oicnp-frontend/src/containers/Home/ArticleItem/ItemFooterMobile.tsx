
export interface ItemFooterMobileProps {

}

export const ItemFooterMobile: React.FC<ItemFooterMobileProps> = () => {

  return (
    <div className="oic-article-footer
      flex flex-col md:flex-row justify-between md:items-center text-sm
    ">
      <div className="flex flex-row items-center justify-between flex-1">
        <div className="oic-author-info flex items-center">
          <a className="block text-slate-400">
            Alex
          </a>
        </div>
        {/* 发布时间 */}
        <div className="oic-article-date text-slate-400">
          2023-02-07
        </div>
      </div>
    </div>
  );
}

