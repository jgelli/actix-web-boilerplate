# Rust Project with Actix-Web and PostgreSQL

This repository contains a Rust project using the Actix-Web framework, configured to run within Docker containers. It includes setups for persistent PostgreSQL data.

## Prerequisites

Make sure you have installed on your system:

- Docker
- Docker Compose

## Setting Up the Environment

  1. **Create the `.env` file:**

      Copy the `.env.example` file to `.env` in the root of the project. This file contains the environment variables needed for Aplication and PostgreSQL connection.


## Makefile Commands

  The Makefile includes commands to facilitate development and management of the Docker environment. Here are the main available commands:

  - **`make up`**: To build and start the Docker containers for Actix-Web and PostgreSQL. Example:

      - This command will initialize the Actix-Web service, accessible at http://localhost:8081.
      -	PostgreSQL will be available on the default port 5432 of your localhost.

  - **`make run-migration file=<filename.sql>`**: Executes a specific SQL file located in the `src/migrations` directory within the PostgreSQL container. Example:

    ```bash
    make run-migration file=0_initial.sql
    ```
