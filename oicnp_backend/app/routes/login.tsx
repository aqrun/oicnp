import type { ActionFunction } from 'remix';
import { useActionData, redirect, json } from 'remix';
import { useMemo } from 'react';
import loginNewBg from '../images/login-new.jpeg';
import { fetchUserLogin } from '~/services';
import { FetchUserLoginRequestParams } from '~/typings';

export interface ActionData {
  formError?: string;
  fieldErrors?: {
    email: string;
    password: string;
  },
  fields?: {
    email: string;
    password: string;
  },
  res?: any;
}

const badRequest = (data: ActionData) => {
  return json(data, { status: 400 })
};

export const action: ActionFunction = async ({
  request,
}) => {
  const form = await request.formData();
  const email = form.get('email') as string;
  const password = form.get('password') as string;

  const params: FetchUserLoginRequestParams = {
    password,
  }

  if (email.indexOf('@') >= 0) {
    params.email = email;
  } else {
    params.username = email;
  }

  const res = await fetchUserLogin(params);

  if (res?.user) {
    console.log('login success');
  }
  console.log(form);
  return badRequest({
    formError: 'hahaha error',
    fields: {
      email,
      password,
    },
    res,
  });
};

const Login = () => {
  const actionData = useActionData<ActionData>();
  console.log('actiondata ----', actionData);
  const bgStyle = useMemo(() => {
    return {
      background: `url('${loginNewBg}')`,
    };
  }, []);

  return (
    <div className="oic-login h-full" style={bgStyle}>
      <div
        className="min-h-full flex items-center justify-center py-12
          px-4 sm:px-6 lg:px-8"
      >
        <div className="max-w-md w-full space-y-8">
          <div>
            <h2
              className="mt-6 text-center text-3xl font-extrabold
                text-gray-900
              "
            >
              Sign in to your account
            </h2>
            <p
              className="font-medium text-indigo-600 text-center
              hover:text-indigo-500 mt-2"
            >
              Or{' '}
              <a
                className="font-medium text-indigo-600
                  hover:text-indigo-500"
              >
                Start your 14-day free trial
              </a>
            </p>
          </div>

          <form
            className="mt-8 space-y-6"
            method="post"
          >
            <input
              type="hidden"
              name="remember"
              defaultValue="true"
            />
            <div
              className="rounded-md shadow-sm -space-y-px"
            >
              <div>
                <label
                  htmlFor="email-address"
                  className="sr-only"
                > 
                  Email address
                </label>
                <input
                  id="email-address"
                  name="email"
                  type="email"
                  autoComplete="email"
                  required
                  className="appearance-none rounded-none relative block
                    w-full px-3 py-2 border border-gray-300
                    placeholder-gray-500 text-gray-900
                    rounded-t-md focus:outline-none focus:ring-indigo-500
                    focus:border-indigo-500 focus:z-10 sm:text-sm                    
                  "
                  placeholder="Email address"
                />
                <div>
                  <label
                    htmlFor="password"
                    className="sr-only"
                  >
                    Password
                  </label>
                  <input
                    id="password"
                    name="password"
                    type="password"
                    autoComplete="current-password"
                    required
                    className="appearance-none rounded-none relative
                      block w-full px-3 py-2 border border-gray-300
                      placeholder-gray-500 text-gray-900
                      rounded-b-md focus:outline-none 
                      focus:ring-indigo-500 focus:border-indigo-500
                      focus:z-10 sm:text-sm top-[-1px]
                    "
                    placeholder="Password"
                  />
                </div>
              </div>

              <div
                className="flex items-center justify-between"
              >
                <div
                  className="flex ites-center mt-2"
                >
                  <input
                    id="remember-me"
                    name="remember-me"
                    type="checkbox"
                    className="h-4 w-4 text-indigo-600
                      focus:ring-indigo-500 border-gray-300 rounded
                    "
                  />
                  <label
                    htmlFor="remember-me"
                    className="ml-2 block text-sm text-gray-900"
                  >
                    Remember me
                  </label>
                </div>
                <div className="text-sm">
                  <a
                    href="#"
                    className="font-medium text-indigo-600
                      hover:text-indigo-500
                    "
                  >
                    Forgot your password?
                  </a>
                </div>
              </div>

              <div>
                <button
                  type="submit"
                  className="group relative w-full flex justify-center
                    py-2 px-4 border border-transparent text-sm
                    font-medium rounded-md text-white
                    bg-indigo-600 hover:bg-indigo-700
                    focus:outline-none focus:ring-2
                    focus:ring-offset-2 focus:ring-indigo-500
                    mt-4
                  "
                >
                  Sign in
                </button>
              </div>

            </div>
          </form>

        </div>
      </div>
    </div>
  );
};

export default Login;