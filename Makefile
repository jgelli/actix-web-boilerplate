include .env

export DB_USER
export DB_PASSWORD
export DB_NAME
export DB_HOST
export DB_URL

POSTGRES_CONTAINER_NAME = boilerplate-postgres

EXEC_PSQL = docker exec -i $(POSTGRES_CONTAINER_NAME) psql -U $(DB_USER) -d $(DB_NAME)

MIGRATION_DIR = /migrations

.PHONY: up
up:
	docker-compose up -d

.PHONY: down
down:
	docker-compose down

.PHONY: run-migration
run-migration:
	@if [ -z "$(file)" ]; then \
		echo "Error: Specify the .sql file name. Example: make run-migration file=0_initial.sql"; \
		exit 1; \
	fi
	$(EXEC_PSQL) -f $(MIGRATION_DIR)/$(file)


.PHONY: clean
clean: down
