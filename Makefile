ifneq (,$(wildcard \./\.env))
    ENV_EXISTS=1
else
    ENV_EXISTS=0
endif
REPO=jakmeier/paddlers

build-and-run: build
	make run

build: game-master-container db-interface-container frontend-container

run: docker-compose.local.yml
	docker-compose -f $< up --no-start
ifeq ($(ENV_EXISTS),1)
	docker cp ./.env paddlers_game-master_1:/app/.env
	docker cp ./.env paddlers_db-interface_1:/app/.env
else
	docker cp ./local.env paddlers_game-master_1:/app/.env
	docker cp ./local.env paddlers_db-interface_1:/app/.env
endif
	docker-compose -f $< up --no-recreate --no-build

rust-container: Dockerfile
	docker build --target RustBaseImg -t $(REPO):builder-base .

game-master-container: Dockerfile rust-container
	docker build --target GameMaster -t $(REPO):game-master-snapshot -f paddlers-game-master/Dockerfile .

db-interface-container: Dockerfile rust-container
	docker build --target DbInterface -t $(REPO):db-interface-snapshot -f paddlers-db-interface/Dockerfile .

frontend-container: Dockerfile rust-container
	docker build --target WebServer -t $(REPO):frontend-snapshot -f paddlers-frontend/Dockerfile .
