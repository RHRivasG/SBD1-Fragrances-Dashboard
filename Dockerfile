FROM postgres:latest
ENV POSTGRES_PASSWORD=postgres
WORKDIR /db/

COPY ./scripts/KMR_SCHEMA_SCRIPT.sql /docker-entrypoint-initdb.d/ASCHEMA.sql
COPY ./scripts/KMR_INSERT_SCRIPT.sql /docker-entrypoint-initdb.d/BINSERT.sql
COPY ./scripts/KMR_INSERT02_SCRIPT.sql /docker-entrypoint-initdb.d/CINSERT.sql

RUN chmod a+r /docker-entrypoint-initdb.d/*
