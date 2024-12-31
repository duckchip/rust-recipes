
# Rust Recipes

A simple Rust application that interacts with a PostgreSQL database to manage recipes. It provides functionality to fetch recipe details and a paginated overview of recipes.

## Features

- Fetch individual recipe details by ID.
- Get an overview of recipes with pagination.
- Connect to a PostgreSQL database to store recipes.
  
## Prerequisites

Before running the application, ensure you have the following installed:

- **Rust**: You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/).
- **Docker**: Required for running PostgreSQL in a container. You can install Docker from [docker.com](https://www.docker.com/).

## Getting Started

### Step 1: Clone the Repository

Clone the repository to your local machine:

```bash
git clone https://github.com/yourusername/rust-recipes.git
cd rust-recipes
```

### Step 2: Set Up the Database

You can use Docker to run a PostgreSQL database for this application. Follow the steps below to set up PostgreSQL.

1. **Create the `.env` file**: 
   
   The `.env` file should contain the database connection details. Here's an example of what to include in your `.env` file:

   ```env
   DB_USER=postgres
   DB_PASSWORD=yourpassword
   DB_NAME=recipe_db
   DB_HOST=localhost
   DB_PORT=5432
   ```

2. **Start PostgreSQL with Docker Compose**:

   You’ll use `docker-compose` to set up PostgreSQL. First, ensure that the `docker-compose.yml` file is in your project directory.

   Then, run the following command to start the PostgreSQL container:

   ```bash
   docker-compose up -d
   ```

   This will pull the PostgreSQL image, create the necessary container, and run it in the background.

3. **Initialize the Database**:

   The `docker-compose.yml` will automatically run the `init.sql` script to create the necessary tables (including the `recipes` table). If you encounter issues, you can check the logs with:

   ```bash
   docker-compose logs postgres
   ```

### Step 3: Set Up the Application

1. **Install Dependencies**:

   Make sure the necessary dependencies are installed. In your project directory, run:

   ```bash
   cargo build
   ```

2. **Update Database URL**:

   Ensure that the application’s database URL is constructed correctly by referencing the `.env` file. The Rust application will read the environment variables to connect to the database.

### Step 4: Run the Application

To start the application, run the following command:

```bash
cargo run
```

This will start the application, and it will connect to the PostgreSQL database running in the Docker container.

### Step 5: Test the Application

The application provides two primary functions:

1. **Fetch Recipe by ID**: This allows you to fetch the details of a recipe by its ID.
2. **Get Paginated Recipe Overview**: This retrieves a list of recipes with pagination information (total items, current page, etc.).

You can adjust the pagination parameters (e.g., page size) in the code or add a simple interface to interact with it.

### Step 6: Stop the Application and Database

To stop the PostgreSQL container, run:

```bash
docker-compose down
```

This will stop and remove the containers created by Docker Compose.

## Troubleshooting

- If you encounter issues with the database not being found (`relation "recipes" does not exist`), make sure the `init.sql` file is correctly set up to create the `recipes` table.
- Check the Docker logs for PostgreSQL:

  ```bash
  docker-compose logs postgres
  ```

- Ensure that the `.env` file is configured correctly, especially for database connection settings.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
