ifneq (,$(wildcard \./\.env))
    ENV_EXISTS=1
else
    ENV_EXISTS=0
endif
REPO=jakmeier/paddlers

build-and-run: build
	make run

build: debug-game-master-container debug-db-interface-container debug-frontend-container debug-keycloak-container

release: game-master-container db-interface-container frontend-container keycloak-container

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
	docker build --target RustBaseImg -t $(REPO):builder-base -f $< .

game-master-container: paddlers-game-master/Dockerfile rust-container
	docker build --target GameMaster -t $(REPO):game-master-snapshot -f $< .

db-interface-container: paddlers-db-interface/Dockerfile rust-container
	docker build --target DbInterface -t $(REPO):db-interface-snapshot -f $< .

debug-game-master-container: paddlers-game-master/debug.Dockerfile rust-container
	docker build --target GameMaster -t $(REPO):game-master-snapshot -f $< .

debug-db-interface-container: paddlers-db-interface/debug.Dockerfile rust-container
	docker build --target DbInterface -t $(REPO):db-interface-snapshot -f $< .

debug-frontend-container: paddlers-frontend/debug.Dockerfile rust-container
	docker build --target WebServer -t $(REPO):frontend-snapshot -f $< .

frontend-container: paddlers-frontend/Dockerfile rust-container
	docker build --target WebServer -t $(REPO):frontend-snapshot -f $< .

debug-keycloak-container: paddlers-keycloak/Dockerfile
	docker build --target KeyCloak -t $(REPO):keycloak-snapshot -f $< .

keycloak-container: paddlers-keycloak/Dockerfile
	docker build --target KeyCloak -t $(REPO):keycloak-snapshot -f $< .
