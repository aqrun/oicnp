import {
  EnvelopeIcon,
  GlobeAsiaAustraliaIcon,
} from '@heroicons/react/24/outline';

export const LocationAndSupport = () => {
  return (
    <div className='flex flex-col'>
      {/* <!-- ITEM --> */}
      <div className='mb-4 flex flex-row items-center'>
        <GlobeAsiaAustraliaIcon className='h-5 w-5 mr-3 inline-block' />

        <p className='font-inter ml-4'>
          8502 Preston Rd. Inglewood, Maine 98380, USA
        </p>
      </div>
      {/* <!-- ITEM --> */}
      <div className='mb-4 flex flex-row items-center'>
        <EnvelopeIcon className='h-5 w-5 mr-3 inline-block' />
        <p className='font-inter ml-4'>aqrun@sina.com</p>
      </div>
    </div>
  );
};
