
export interface ItemFooterProps {

}

export const ItemFooter: React.FC<ItemFooterProps> = () => {

  return (
    <div className="oic-article-footer flex justify-between items-center">
      <div className="oic-col-left flex items-center">
        <div className="oic-author-info flex items-center">
          <a className="block w-8 h-8 bg-slate-100 rounded-[50%] relative overflow-hidden">
            <img />
          </a>
          <a className="block ml-2 text-black font-light text-xs">
            Alex
          </a>
        </div>
        <div className="oic-article-date ml-3 text-slate-600 text-xs font-light">
          2023年02月07日 13:40
        </div>
        <div className="oic-viewed ml-6 text-xs flex items-center">
          <i className="icon iconfont icon-view2 text-black" />
          <span className="ml-2 text-black-300 font-light">104</span>
        </div>
        <div className="oic-commentted ml-6 text-xs flex items-center">
          <i className="icon iconfont icon-comments text-black" />
          <span className="ml-2 text-black-300 font-light">14</span>
        </div>
        <div className="oic-votted ml-6 text-xs flex items-center">
          <i className="icon iconfont icon-thumbs-o-up text-black" />
          <span className="ml-2 text-black-300 font-light">3</span>
        </div>
      </div>
      <div className="oic-col-right">
        <div className="oic-tags flex items-center text-xs text-black-300">
          <i className="icon iconfont icon-tag" />
          <div className="oic-tag-items flex items-center">
            <a className="ml-6 first:ml-2 hover:text-purple">Node.js</a>
            <a className="ml-6 hover:text-purple">前端</a>
            <a className="ml-6 hover:text-purple">JavaScript</a>
          </div>
        </div>
      </div>
    </div>
  );
}

