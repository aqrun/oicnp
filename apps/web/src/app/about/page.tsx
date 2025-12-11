import { Metadata } from 'next';
import React from 'react';

import { MainLayout } from '@/components/layouts';
import { parseMd } from '@/utils/md';
import { siteConfig } from '@/constant';
import { SideBar } from '@/components/HomePage';
import { HeroContainer } from './index.styled';

const CONTENT = `
## 一、 缘起 | Why This？

在技术的世界里，最难得的并非代码本身，而是那穿透复杂表象的洞见，是跨越山海却能瞬间共鸣的理解，是历经调试后豁然开朗的顿悟。

灵犀纪，便诞生于对这种状态的向往。

* 灵犀：代表我们追求技术洞察的深度与思想共鸣的默契。
* 纪：既是记录，也是纪年。我们愿以文字为碑，记录这个波澜壮阔的技术时代中，那些值得被梳理、沉淀与分享的思考。

这里不满足于简单的“How-to”，更致力于探寻背后的“Why”。我们相信，真正的力量，来源于理解。

## 二、我是谁 | Who’s Behind This？


你好，我是子十（Aqrun）,前端开发，拥有多年的行业实践与思考。

我始终对技术如何塑造世界充满好奇，我相信技术是理性的诗歌，工程是权衡的艺术。我享受在复杂系统中寻找优雅解的过程，更享受将其中艰涩的原理，转化为清晰易懂的逻辑。

写作是我梳理思考、对抗遗忘的方式，也是我与你——遥远的同行者——隔空击掌的桥梁。如果这里的某篇文章，能让你在深夜调试时少踩一个坑，或在设计系统时多一份灵感，那便是“灵犀”最美的时刻。

## 三、 在这里，你能找到什么 | What You‘ll Find Here

灵犀纪的内容，聚焦于Rust语言、前端开发等，主要涵盖：

* 🛠️ 深度解构：不止于API使用手册，更深入原理层，剖析主流框架、系统或协议的核心设计与实现逻辑。
* 🧭 架构沉思：关于系统设计、技术选型、演进路径的思考与复盘。既有宏观视野，也有落地细节。
* 🚀 实践真知：来自一线实战的经验总结、最佳实践、性能调优与“填坑”记录。务实，可复用。
* 🔮 前沿瞭望：对新兴技术趋势的冷静分析与价值判断，试图在喧嚣中辨别真伪，捕捉真正的信号。
* 💡 思维模型：分享那些超越具体技术、能广泛应用的思考工具与学习方法论。

内容原则：专注、深度、清晰、诚实。​ 不追热点，不制造焦虑，只分享经过验证和深思的内容。

## 四、 与我连接 | Let‘s Connect

独行快，众行远。思想的火花，在碰撞中才更耀眼。

* 交流反馈：每篇文章都开放评论，欢迎你留下思考、疑问或不同的见解。理性的讨论是这里最珍贵的礼物。
* 内容订阅：
  * 邮件订阅：通过 [RSS 链接] 或加入邮件列表，获取最新的文章推送，永不迷路。
  * 社交媒体：你可以在 GitHub 找到我，那里会有更及时的动态和碎片思考。
* 合作与联系：如有技术交流、演讲分享或内容合作意向，欢迎通过邮箱 aqrun@sina.com​ 与我联络。

## 五、 版权与转载

* 除非特别注明，本站所有文章均采用 [建议使用 CC BY-NC-SA 4.0 等知识共享协议]​ 进行许可。
* 转载时请务必注明原始出处（灵犀纪）与文章链接，尊重创作。

---

最后，感谢你的到来。

愿这里能成为你技术地图上一个有用的坐标，让我们在探索的漫长旅程中，偶尔灵犀相通，彼此照亮。
`;

export const metadata: Metadata = {
  title: '关于我',
  description: `关于我 ${siteConfig.description}`,
};

export default async function AboutPage() {
  const content = await parseMd(CONTENT);
  return (
    <MainLayout>
      <div className="layout">
        <HeroContainer
          id="blog-hero"
          className="flex flex-col items-center justify-center bg-center bg-cover bg-no-repeat py-0 px-1 text-white rounded-md mb-6"
        >
          <h1 className="blog-hero-title">关于我</h1>
          <div className="blog-hero-description">
            身无彩凤双飞翼，心有灵犀一点通。
          </div>
        </HeroContainer>
        <div className="flex lg:flex-row flex-col gap-4 mb-8">
          <div className='oic-layout-content1 flex flex-col flex-1'>
            <article
              className='oic-article-detail prose lg:prose-p max-w-full break-words'
              dangerouslySetInnerHTML={{
                __html: content || '',
              }}
            />
            <article className='oic-article-detail prose lg:prose-p max-w-full'>
              <p className='text-center lg:text-right text-gray-500 mt-4 lg:mr-24'>— 子十 于 西安</p>
            </article>
          </div>
          <div className='lg:w-80'>
            <SideBar
              hasTags
            />
          </div>
        </div>
      </div>
    </MainLayout>
  );
}
