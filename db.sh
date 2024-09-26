#!/bin/sh

env SURREAL_EXPERIMENTAL_GRAPHQL=true surreal start --log trace --user root --pass root --bind 127.0.0.1:8000 surrealkv:rustblog.db