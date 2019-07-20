#!/bin/bash
graphql-client introspect-schema http://localhost:65432/graphql > paddlers-frontend/api/schema.json