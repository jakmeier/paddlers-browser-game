
REPO=jakmeier/paddlers
LOCAL=jakmeier/paddlers-local
DOT := $(shell command -v dot 2> /dev/null)

build-and-run: build
	make run

build: debug-game-master-container debug-db-interface-container debug-frontend-container debug-keycloak-container

mobile: debug-game-master-container debug-db-interface-container mobile-frontend-container mobile-keycloak-container
	make run

release: game-master-container db-interface-container frontend-container keycloak-container

run: docker-compose.local.yml
	docker-compose -f $< up --no-start
	docker cp ./local.env paddlers_game-master_1:/app/.env
	docker cp ./local.env paddlers_db-interface_1:/app/.env
	docker-compose -f $< up --no-recreate --no-build

upload-frontend-builder: frontend-builder
	docker login; docker push $(REPO):frontend-builder

rust-container: Dockerfile
	docker build --target RustBaseImg -t $(REPO):builder-base -f $< .

frontend-builder: paddlers-frontend/build.Dockerfile
	docker build -t $(REPO):frontend-builder -f $< .

game-master-container: paddlers-game-master/Dockerfile rust-container
	docker build --target GameMaster -t $(REPO):game-master-snapshot -f $< .

db-interface-container: paddlers-db-interface/Dockerfile rust-container
	docker build --target DbInterface -t $(REPO):db-interface-snapshot -f $< .

debug-game-master-container: paddlers-game-master/debug.Dockerfile rust-container
	docker build --target GameMaster -t $(LOCAL):game-master-snapshot -f $< .

debug-db-interface-container: paddlers-db-interface/debug.Dockerfile rust-container
	docker build --target DbInterface -t $(LOCAL):db-interface-snapshot -f $< .

debug-frontend-container: paddlers-frontend/debug.Dockerfile
	docker build --target WebServer -t $(LOCAL):frontend-snapshot -f $< .

frontend-container: paddlers-frontend/Dockerfile rust-container
	docker build --target WebServer -t $(REPO):frontend-snapshot -f $< .

mobile-frontend-container: paddlers-frontend/mobile.Dockerfile
	docker build --target WebServer -t $(LOCAL):frontend-snapshot -f $< .

debug-keycloak-container: paddlers-keycloak/debug.Dockerfile
	docker build --target KeyCloak -t $(LOCAL):keycloak-snapshot -f $< .

keycloak-container: paddlers-keycloak/Dockerfile
	docker build --target KeyCloak -t $(REPO):keycloak-snapshot -f $< .

mobile-keycloak-container: paddlers-keycloak/mobile.Dockerfile
	docker build --target KeyCloak -t $(LOCAL):keycloak-snapshot -f $< .

# When container are alrady running, use these for partial update
recreate-frontend: debug-frontend-container
	docker-compose -f docker-compose.local.yml up -d --no-deps --build frontend

recreate-frontend-mobile: mobile-frontend-container
	docker-compose -f docker-compose.local.yml up -d --no-deps --build frontend

recreate-db-interface: debug-db-interface-container
	docker-compose -f docker-compose.local.yml up -d --no-deps --build db-interface

recreate-game-master: debug-game-master-container
	docker-compose -f docker-compose.local.yml up -d --no-deps --build game-master

benchmark-file-sizes:
	./paddlers-frontend/benchmarks/app_size_stats.sh

# Translations
paddlers-frontend/static/locale/%.mo: texts/%.po
	msgfmt $< -o $@

.PHONY: translations
translations: paddlers-frontend/static/locale/en.mo paddlers-frontend/static/locale/de.mo

.PHONY: images
images:
	$(MAKE) -C paddlers-frontend images

# Code generation
.PHONY: generate-files-from-specifications
generate-files-from-specifications:
	cd specification-loader; cargo +nightly run -- generate enum ../paddlers-shared-lib/src/generated
	cd specification-loader; cargo +nightly run -- generate chart ../specification
# If this fails, install graphviz  (e.g. sudo apt-get install graphviz)
	cd specification; dot -Tsvg story.dot > story.svg
