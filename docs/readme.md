* https://gitee.com/chenlunfu/qiluo_admin

```shell
scp /home/aqrun/workspace/github.com/aqrun/oicnp/target/release/oic $SERVER_USERNAME@$SERVER_IP:/workspace/github.com/aqrun/oicnp/target/release
scp /home/aqrun/workspace/github.com/aqrun/oicnp/target/release/ic $SERVER_USERNAME@$SERVER_IP:/workspace/github.com/aqrun/oicnp/target/release
scp -r -v -o "StrictHostKeyChecking=no" /home/aqrun/workspace/github.com/aqrun/oicnp/apps/web/.next $SERVER_USERNAME@$SERVER_IP:/workspace/github.com/aqrun/oicnp/apps/web/
scp -r -v -o "StrictHostKeyChecking=no" /home/aqrun/workspace/github.com/aqrun/oicnp/apps/backend/.next $SERVER_USERNAME@$SERVER_IP:/workspace/github.com/aqrun/oicnp/apps/backend/
scp -r -v -o "StrictHostKeyChecking=no" /home/aqrun/workspace/github.com/aqrun/oicnp/apps/web-app/dist $SERVER_USERNAME@$SERVER_IP:/workspace/github.com/aqrun/oicnp/apps/web-app/
```