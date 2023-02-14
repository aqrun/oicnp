
export interface ItemFooterProps {

}

export const ItemFooter: React.FC<ItemFooterProps> = () => {

  return (
    <div className="oic-article-footer hidden
      md:flex flex-col md:flex-row justify-between md:items-center text-sm
    ">
      <div className="oic-col-left flex flex-col md:flex-row md:items-center">
        <div className="flex flex-row items-center">
          <div className="oic-author-info flex items-center">
            <a className="block w-8 h-8 bg-slate-100 rounded-[50%] relative overflow-hidden">
              <img />
            </a>
            <a className="block ml-2 text-black">
              Alex
            </a>
          </div>
          {/* 发布时间 */}
          <div className="oic-article-date ml-3 text-slate-600">
            2023年02月07日 13:40
          </div>
        </div>
        <div className="flex flex-row items-center mt-2 md:mt-0">
          {/* 分类 */}
          <div className="oic-category md:ml-6 flex items-center">
            <i className="icon iconfont icon-benshubook122 text-black" />
            <span className="ml-1 text-black-300 font-normal">JAVA</span>
          </div>
          {/* 阅读量 */}
          <div className="oic-viewed ml-6 flex items-center">
            <i className="icon iconfont icon-view2 text-black" />
            <span className="ml-1 text-black-300 font-light">104</span>
          </div>
          {/* 评论数 */}
          <div className="oic-commentted ml-6 flex items-center">
            <i className="icon iconfont icon-comments text-black" />
            <span className="ml-1 text-black-300 font-light">14</span>
          </div>
          {/* 点赞数 */}
          <div className="oic-votted ml-6 flex items-center">
            <i className="icon iconfont icon-thumbs-o-up text-black" />
            <span className="ml-1 text-black-300 font-light">3</span>
          </div>
        </div>
      </div>
      <div className="oic-col-right mt-2 md:mt-0">
        {/* 标签列表 */}
        <div className="oic-tags flex items-center text-black-300">
          <i className="icon iconfont icon-tag" />
          <div className="oic-tag-items flex items-center">
            <a className="ml-6 first:ml-2 hover:text-purple-400">Node.js</a>
            <a className="ml-6 hover:text-purple-400">前端</a>
            <a className="ml-6 hover:text-purple-400">JavaScript</a>
          </div>
        </div>
      </div>
    </div>
  );
}

