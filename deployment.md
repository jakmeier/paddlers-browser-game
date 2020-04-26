# Paddlers Demo deployment

1) Build all docker images on dockerhub
2) Redeploy all images that should be updated
3) Make sure all containers are running and found a connection to each other, otherwise restart containers
4) SSH into frontend, generate keys, issue certificate with acme.sh, change nginx to SSL config and restart
## Optional
5) For game data reset, without deleting account information, redeploy game DB without volume preservation and restart game-master + db-interface afterwards