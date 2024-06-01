cli commands

```
aws configure
aws ecr create-repository --repository-name websocket-app
docker build -t socket-app .
docker tag socket-app:latest <account_id>.dkr.ecr.eu-west-1.amazonaws.com/websocket-app:latest
aws ecr get-login-password --region eu-west-1 | docker login --username AWS --password-stdin <account_id>.dkr.ecr.eu-west-1.amazonaws.com
docker push <account_id>.dkr.ecr.eu-west-1.amazonaws.com/socket-app:latest
docker push <account_id>.dkr.ecr.eu-west-1.amazonaws.com/websocket-app:latest
```
