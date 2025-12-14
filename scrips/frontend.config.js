// pm2 start frontend.config.js

module.exports = {
  apps: [
    {
      name: 'lxage-web',
      cwd: './apps/web',
      script: 'node_modules/.bin/next',
      args: 'start -p 9000',
      instances: 'max',
      exec_mode: 'cluster',
      env: {
        NODE_ENV: 'production',
        PORT: 9000
      }
    },
    {
      name: 'lxage-admin',
      cwd: './apps/backend',
      script: 'node_modules/.bin/next',
      args: 'start -p 9001',
      instances: 1,
      env: {
        NODE_ENV: 'production',
        PORT: 9001
      }
    }
  ]
};