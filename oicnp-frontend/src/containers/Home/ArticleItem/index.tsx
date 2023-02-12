import { ItemFooter } from './ItemFooter';

export interface ArticleItemProps {

}

export const ArticleItem: React.FC<ArticleItemProps> = () => {

  return (
    <article className="oic-article-item mb-1 mx-[-20px] py-4 pl-5 pr-5 relative border-b border-solid border-b-slate-200 last:border-b-0 hover:shadow-md">
      <div className="oic-row flex items-start flex-nowrap justify-between">
        <div className="oic-article-content">
          <h4 className="oic-title">
            <a
              className="break-words font-semibold text-lg leading-6 mb-3 overflow-ellipsis 
                text-black hover:text-purple-400"
            >
              看不懂源码？我总结了18条心法，助你修炼神功！
            </a>
          </h4>
          <div
            className="oic-desc text-justify overflow-hidden
              text-sm overflow-ellipsis h-11 break-words mt-4"
          >
            <p>
              如何去阅读源码，18条心法祝你修炼神功！如何去阅读源码，18条心法祝你修炼神功！
              如何去阅读源码，18条心法祝你修炼神功！如何去阅读源码，18条心法祝你修炼神功！
              如何去阅读源码，18条心法祝你修炼神功！
              如何去阅读源码，18条心法祝你修炼神功！
            </p>
          </div>
        </div>
        <div className="oic-article-img ml-5 w-44 min-h-[123px] min-w-[180px] relative bg-slate-100 rounded-md">
        </div>
      </div>

      <ItemFooter />
    </article>
  );
}
