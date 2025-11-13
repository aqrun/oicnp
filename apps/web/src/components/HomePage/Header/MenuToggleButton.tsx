'use client';

export const MenuToggleButton = () => {
  return (
    <button
      type='button'
      className='flex items-center text-gray-500 hover:text-gray-600 focus:outline-none focus:text-gray-600'
      aria-label='Toggle menu'
      onClick={() => {
        const $s = document.querySelector('.oic-header-menu');
        if ($s) {
          $s?.classList.toggle('hidden');
        }
      }}
    >
      <span className='sr-only'>Toggle navigation</span>
      <svg viewBox='0 0 24 24' className='h-6 w-6 fill-current'>
        <path
          fillRule='evenodd'
          d='M4 5h16a1 1 0 0 1 0 2H4a1 1 0 1 1 0-2zm0 6h16a1 1 0 0 1 0 2H4a1 1 0 0 1 0-2zm0 6h16a1 1 0 0 1 0 2H4a1 1 0 0 1 0-2z'
        ></path>
      </svg>
    </button>
  );
};
