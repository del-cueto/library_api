library_api

A RESTful API in Rust for managing a library of books using SQLite and JWT authentication.

Prerequisites

Rust (1.70+ recommended) and Cargo installed.

SQLite installed (for local development).

sqlx-cli installed for running migrations:

cargo install sqlx-cli --no-default-features --features sqlite

Git installed to clone the repository.

Clone the repository

git clone git@github.com:del-cueto/library_api.git
cd library_api

Setup environment variables

Create a .env file in the project root with:

DATABASE_URL=sqlite://./library.db
JWT_SECRET=your-secret-key

DATABASE_URL can be a relative path (sqlite://./library.db) for local file library.db in project root.

JWT_SECRET should be a strong, random string.

Load the environment variables before running the app (shell will pick up .env automatically if using dotenvy).

Run database migrations

Ensure sqlx-cli is installed and the .env is loaded.

# If using dotenvy: load .env then run:
# In bash: source .env
sqlx migrate run

This creates the books table in library.db.

Build and run the application

Compile in release mode:

cargo build --release

Run the server:

# Ensure .env is loaded or set env vars manually
cargo run --bin library_api

By default, the server listens on http://127.0.0.1:3000.

API Endpoints

POST /login

Body JSON: { "username": "admin", "password": "password" }

Returns: JWT string

GET /books

Public: list all books

GET /books/{id}

Public: get a book by ID

GET /books/search?title=...&author=...

Public: search by title and/or author (partial match)

POST /books

Protected: requires Authorization: Bearer <token> header

Body JSON: { "title": "...", "author": "...", "published_year": 2025 }

PUT /books/{id}

Protected: requires JWT

Body JSON: any of fields to update: { "title": "New Title" }

DELETE /books/{id}

Protected: requires JWT

Testing

Run all tests (unit and integration):

cargo test

Integration tests use an in-memory SQLite database and cover login, CRUD, search, and error cases.

Using Postman or curl

Example with curl:

# Obtain token
TOKEN=$(curl -s -X POST http://127.0.0.1:3000/login \
-H 'Content-Type: application/json' \
-d '{"username":"admin","password":"password"}')

# Create a book
curl -X POST http://127.0.0.1:3000/books \
-H "Authorization: Bearer $TOKEN" \
-H 'Content-Type: application/json' \
-d '{"title":"Test","author":"Me","published_year":2025}'

# List books
curl http://127.0.0.1:3000/books

Logging

The application uses tracing for structured logging. Logs appear on stdout.

Optional: Pagination

If implemented, add query parameters ?page=1&per_page=10 to GET /books.

Optional: Docker

Dockerfile is provided for containerizing the app. To build:

docker build -t library_api:latest .

To run with volume for persistence:

mkdir -p data
docker run -d -p 3000:3000 \
-e DATABASE_URL="sqlite:////data/library.db" \
-e JWT_SECRET="your-secret" \
-v "$(pwd)/data":/data \
--name library_api library_api:latest

README Maintenance

Update this file with any new instructions or features. Include screenshots or links to documentation as needed.

