#!/bin/bash
graphql-client introspect-schema http://localhost:65432/graphql > api/schema.json