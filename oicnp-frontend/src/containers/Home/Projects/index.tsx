import React, { useMemo } from 'react';

export interface ProjectsProps {

}

/**
 * 项目文档URL导航
 */
export const Projects: React.FC<ProjectsProps> = () => {

  // 🔱✴️🌿🧭
  const projectList = useMemo(() => [
    {
      name: 'SeaORM',
      desc: '🐚 异步动态ORM',
      href: 'https://www.oicnp.com/sea-orm/',
      tags: [
        {
          name: 'GitHub',
          href: 'https://github.com/SeaQL/sea-orm',
        },
        {
          name: 'Docs',
          href: 'https://www.oicnp.com/sea-orm/SeaORM/',
        }
      ],
    },
    {
      name: 'Prisma Client Rust',
      desc: '🔱 Prisma Rust 客户端',
      href: 'https://oicnp.com/rust-prisma/',
      tags: [
        {
          name: 'GitHub',
          href: 'https://github.com/SeaQL/sea-orm',
        },
        {
          name: 'Docs',
          href: 'https://oicnp.com/rust-prisma/',
        }
      ],
    },
  ], []);

  return (
    <section className="oic-home-projects mt-6 mb-9">
      <div className="oic-inner mx-auto max-w-7xl">
        <h3
          className=" text-center text-4xl mb-9 font-normal text-gray-900"
        >
          在线文档
        </h3>
        <div className="w-full overflow-hidden pb-4">
          <ul className="grid grid-cols-2 gap-4">
            {projectList?.map((item) => {
              return (
                <li
                  key={item?.name}
                  className=" before:ease-out
                    before:transition-[right] before:duration-300
                    before:bg-purple-300 before:h-1 before:w-full
                    before:content-[''] before:absolute before:bottom-0
                    before:right-full before:hover:right-0
                    relative rounded-md shadow-sm bg-white p-6 text-center overflow-hidden
                    hover:shadow-md
                    "
                >
                  <div
                    className=""
                  >
                    <h4 className="mt-4">
                      <a
                        href={item?.href}
                        className="text-2xl leading-6 font-semibold mb-2 text-gray-900 hover:text-purple-300"
                      >
                        {item?.name}
                      </a>
                    </h4>
                    <p
                      className="text-sm leading-6 mt-2"
                    >
                      {item?.desc}
                    </p>
                    <div className="oic-btns-w flex items-center justify-center mt-4 gap-2">
                      {item?.tags?.map((n) => {
                        return (
                          <a
                            key={n?.name}
                            className="
                              hover:border-purple-300 hover:text-purple-300
                              px-2 rounded-2xl border-solid border-gray-300 border"
                            href={n?.href}
                            target="_blank"
                            rel="noreferrer"
                          >{n?.name}</a>
                        );
                      })}
                    </div>
                  </div>
                </li>
              );
            })}
          </ul>
        </div>
      </div>
    </section>
  );
};
