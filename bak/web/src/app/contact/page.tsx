import { Metadata } from 'next';
import React from 'react';

import { MainLayout } from '@/components/layouts';
import { siteConfig } from '@/constant';
import { SideBar } from '@/components/HomePage';
import { HeroContainer } from './index.styled';

export const metadata: Metadata = {
  title: '联系我',
  description: `联系我 ${siteConfig.description}`,
};

export default async function AboutPage() {
  return (
    <MainLayout>
      <div className="layout">
        <HeroContainer
          id="blog-hero"
          className="flex flex-col items-center justify-center bg-center bg-cover bg-no-repeat py-0 px-1 text-white rounded-md mb-6"
        >
          <h1 className="blog-hero-title">联系我</h1>
          <div className="blog-hero-description">
            愿你我在此相遇，共同编织一段技术之旅。
          </div>
        </HeroContainer>
        <div className="flex lg:flex-row flex-col gap-4 mb-8">
          <div className='oic-layout-content1 flex flex-col flex-1'>
            
          <form>
            <div className="space-y-12">
              
              <div className="border-b border-gray-900/10 pb-12">
                <p className="mt-1 text-sm/6 text-gray-600">任何问题，任何建议，可通过以下方式与我联系！</p>

                <div className="mt-10 grid grid-cols-1 gap-x-6 gap-y-4 sm:grid-cols-6">
                  <div className="sm:col-span-3">
                    <label htmlFor="name" className="block text-sm/6 font-medium text-gray-900">您的姓名</label>
                    <div className="mt-2">
                      <input id="name" type="text" name="name" className="block w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-purple-600 sm:text-sm/6" />
                    </div>
                  </div>
                  <div className="col-span-full" />
                  <div className="sm:col-span-3">
                    <label htmlFor="email" className="block text-sm/6 font-medium text-gray-900">您的邮箱</label>
                    <div className="mt-2">
                      <input id="email" type="email" name="email" className="block w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-purple-600 sm:text-sm/6" />
                    </div>
                  </div>
                  <div className="col-span-full" />
                  <div className="sm:col-span-3">
                    <label htmlFor="phone" className="block text-sm/6 font-medium text-gray-900">您的联系电话</label>
                    <div className="mt-2">
                      <input id="phone" type="text" name="phone" className="block w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-purple-600 sm:text-sm/6" />
                    </div>
                  </div>

                  <div className="col-span-full">
                    <label htmlFor="about" className="block text-sm/6 font-medium text-gray-900">您的问题描述</label>
                    <div className="mt-2">
                      <textarea id="about" name="about" rows={3} className="block w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-purple-600 sm:text-sm/6"></textarea>
                    </div>
                  </div>

                </div>
              </div>

            </div>

            <div className="mt-6 flex items-center justify-center gap-x-6">
              <button type="submit" className="rounded-md bg-purple-600 px-3 py-2 text-sm font-semibold text-white shadow-xs hover:bg-purple-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-purple-600 px-20 cursor-pointer">提交</button>
            </div>
          </form>


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
