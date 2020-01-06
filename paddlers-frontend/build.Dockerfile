# Dockerfile for
# jakmeier/paddlers:frontend-builder
#
# A separate docker image for the fully built frontend which only needs to be copied to the webserver image.
# This image only exists to reduce local compilation time by permanently caching a full build of a recent version
# with all dependencies already compiled.
# Extending this image and compiling the project again takes full advantage of incremental builds.
#
# Note that this image does not have to be updated for every update of jakmeier/paddlers:frontend-snapshot
# which will build the project again from the newest source anyway.
#
FROM jakmeier/paddlers:builder-base
# Install cargo web
RUN USER=root cargo install cargo-web
# Copy all source code
RUN mkdir paddlers-shared-lib
COPY ./paddlers-shared-lib/src ./paddlers-shared-lib/src
COPY ./paddlers-shared-lib/Cargo.toml ./paddlers-shared-lib/
RUN mkdir paddlers-frontend
COPY ./paddlers-frontend/api ./paddlers-frontend/api
COPY ./paddlers-frontend/src ./paddlers-frontend/src
COPY ./paddlers-frontend/Cargo.toml ./paddlers-frontend/
COPY ./Cargo.lock ./paddlers-frontend/
# Build project
RUN cd paddlers-frontend; cargo web deploy --target=wasm32-unknown-unknown --release --features=dev_view