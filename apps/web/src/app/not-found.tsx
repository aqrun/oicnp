import { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'Not Found',
};

export default function NotFound() {
  return (
    <main>
      <section className='flex items-center h-full p-16 bg-gray-100 text-gray-600 min-h-screen'>
        <div className='container flex flex-col items-center justify-center px-5 mx-auto my-8'>
          <div className='max-w-lg text-center'>
            <h2 className='mb-8 font-extrabold text-9xl text-gray-400'>
              <span className='sr-only'>Error</span>404
            </h2>
            <p className='text-2xl font-semibold md:text-3xl'>
              唉呀！这是新大陆，好像还没有内容
            </p>
            <p className='mt-4 mb-8 text-gray-400'>
              不用担心，巨量信息可以查看首页
            </p>
            <a
              rel='noopener noreferrer'
              href='/'
              className='px-8 py-3 font-semibold rounded bg-violet-700 text-violet-100'
            >
              返回首页
            </a>
          </div>
        </div>
      </section>
    </main>
  );
}
