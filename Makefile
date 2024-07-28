SHELL := /bin/bash

include .env

export DB_USER
export DB_PASSWORD
export DB_NAME
export DB_HOST
export DB_URL

POSTGRES_CONTAINER_NAME = boilerplate-postgres

DOCKER_EXEC = docker exec -i

EXEC_PSQL = $(DOCKER_EXEC) $(POSTGRES_CONTAINER_NAME) psql -U $(DB_USER) -d $(DB_NAME)

DOCKER_MIGRATION_DIR = /migrations
PROJECT_MIGRATION_DIR = src/migrations

.PHONY: up
up:
	docker-compose --env-file .env up -d

.PHONY: down
down:
	docker-compose down

.PHONY: run-migration
run-migration:
	@if [ -z "$(file)" ]; then \
		echo "Error: Specify the .sql file name or 'all'. Example: make run-migration file=0_initial.sql or make run-migration file=all"; \
		exit 1; \
	elif [ "$(file)" = "all" ]; then \
		for filepath in $$(ls $(PROJECT_MIGRATION_DIR)/*.sql | sort -V); do \
			filename=$$(basename $$filepath); \
			echo "Processing $$file"; \
			$(EXEC_PSQL) -f $(DOCKER_MIGRATION_DIR)/$$filename; \
		done; \
	else \
		$(EXEC_PSQL) -f src$(DOCKER_MIGRATION_DIR)/$(file); \
	fi

# process_sql:
# 	@bash -c 'for file in $(shell ls src/migrations/*.sql | sort -V); do \
# 		echo "Processing $$file"; \
# 	done'


.PHONY: clean
clean: down
