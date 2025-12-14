// pm2 start frontend.config.js

module.exports = {
  apps: [
    {
      name: 'lxage-web',
      cwd: '/workspace/github.com/aqrun/oicnp/apps/web',
      script: 'node_modules/.bin/next',
      args: 'start -p 9000',
      instances: 1,
      exec_mode: 'fork',
      watch: false,
      max_memory_restart: '500M',
      env: {
        NODE_ENV: 'production',
        PORT: 9000,
        HOSTNAME: '0.0.0.0',
        NODE_OPTIONS: '--max-old-space-size=500'
      },
      error_file: '/var/log/pm2/lxage-web-error.log',
      out_file: '/var/log/pm2/lxage-web-out.log',
      log_date_format: 'YYYY-MM-DD HH:mm:ss',
      merge_logs: true,
      log_type: 'json'
    },
    {
      name: 'lxage-admin',
      cwd: '/workspace/github.com/aqrun/oicnp/apps/backend',
      script: 'node_modules/.bin/next',
      args: 'start -p 9001',
      instances: 1,
      exec_mode: 'fork',
      watch: false,
      max_memory_restart: '500M',
      env: {
        NODE_ENV: 'production',
        PORT: 9001,
        HOSTNAME: '0.0.0.0',
        NODE_OPTIONS: '--max-old-space-size=1024',
      },
      error_file: '/var/log/pm2/lxage-admin-error.log',
      out_file: '/var/log/pm2/lxage-admin-out.log',
      log_date_format: 'YYYY-MM-DD HH:mm:ss',
      merge_logs: true,
      log_type: 'json'
    }
  ]
};