run: docker-compose.test.yml game-master-container db-interface-container frontend-container
	docker-compose -f $< up

rust-container: Dockerfile
	docker build --target RustBaseImg -t rust:nightly .

game-master-container: Dockerfile rust-container
	docker build --target GameMaster -t paddlers/game-master:latest -f paddlers-game-master/Dockerfile .

db-interface-container: Dockerfile rust-container
	docker build --target DbInterface -t paddlers/db-interface:latest -f paddlers-db-interface/Dockerfile .

frontend-container: Dockerfile rust-container
	docker build --target WebServer -t paddlers/frontend:latest -f paddlers-frontend/Dockerfile .

