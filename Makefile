run: docker-compose.test.yml game-master-container db-interface-container frontend-container
	docker-compose -f $< up

rust-container: Dockerfile
	docker build --target RustBaseImg -t jakmeier/paddlers:builder-base .

game-master-container: Dockerfile rust-container
	docker build --target GameMaster -t jakmeier/paddlers:game-master-snapshot -f paddlers-game-master/Dockerfile .

db-interface-container: Dockerfile rust-container
	docker build --target DbInterface -t jakmeier/paddlers:db-interface-snapshot -f paddlers-db-interface/Dockerfile .

frontend-container: Dockerfile rust-container
	docker build --target WebServer -t jakmeier/paddlers:frontend-snapshot -f paddlers-frontend/Dockerfile .
