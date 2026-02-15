* https://gitee.com/chenlunfu/qiluo_admin

```shell
scp /home/aqrun/workspace/github.com/aqrun/oicnp/target/x86_64-unknown-linux-musl/release/oic $SERVER_USERNAME@$SERVER_IP:$SERVER_PROJECT_PATH/target/release
scp /home/aqrun/workspace/github.com/aqrun/oicnp/target/x86_64-unknown-linux-musl/release/ic $SERVER_USERNAME@$SERVER_IP:$SERVER_PROJECT_PATH/target/release
scp -r -v -o "StrictHostKeyChecking=no" /home/aqrun/workspace/github.com/aqrun/oicnp/apps/web/.next $SERVER_USERNAME@$SERVER_IP:$SERVER_PROJECT_PATH/apps/web/
scp -r -v -o "StrictHostKeyChecking=no" /home/aqrun/workspace/github.com/aqrun/oicnp/apps/backend/.next $SERVER_USERNAME@$SERVER_IP:$SERVER_PROJECT_PATH$/apps/backend/
scp -r -v -o "StrictHostKeyChecking=no" /home/aqrun/workspace/github.com/aqrun/oicnp/apps/web-app/dist $SERVER_USERNAME@$SERVER_IP:$SERVER_PROJECT_PATH/apps/web-app/
```