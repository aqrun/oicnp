import React from 'react';

export interface RandomRecommendProps {

}

export const RandomRecommend: React.FC<RandomRecommendProps> = () => {

  return (
    <div className="oic-widget oic-widget-randomRecommend">
      <h3 className="oic-widget-title">随机推荐</h3>
      <ul>
        {[1, 2, 3, 4, 5].map((item) => {
          return (
            <li
              key={item}
              className="border-b border-solid border-slate-100 last:border-b-0"
            >
              <div
                className="flex items-center px-5 py-2 text-slate-800 text-sm hover:text-purple-400"
              >
                <div
                  className="oic-thumbnail rounded-md overflow-hidden w-32 min-w-[8rem] h-20 bg-slate-100 mr-4"
                >
                  <img />
                </div>
                <div
                  className="oic-titleAndDate block"
                >
                  <a
                    className="text-justify block"
                  >
                    显瘦牛仔裙半身裙短新款潮裙女夏韩版学生高腰a字裙
                  </a>
                  <span
                    className="block text-xs text-gray-400 mt-2"
                  >2018-10-25</span>
                </div>
              </div>
            </li>
          );
        })}
      </ul>
    </div>
  );
};
