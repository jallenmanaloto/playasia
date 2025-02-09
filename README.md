# Rust Poem CRUD Application

A simple CRUD application built with Rust and [Poem](https://docs.rs/poem/latest/poem/index.html) as part of a job application with Playasia. This project demonstrates basic Create, Read, Update, and Delete operations on a file-based JSON "database" with JWT-based authentication on non-GET endpoints.

## Features
- **Server health check:**  
  Implement an endpoint to ping the server and ensure that it is active.

- **CRUD Operations:**  
  Implement endpoints to create, read, update, and delete items.

- **File-Based Storage:**  
  Uses a `data.json` file to store data as a JSON array.

- **JWT Authentication:**  
  Secures non-GET endpoints with JWT-based middleware. All requests aside from GET must include a valid Bearer token.

- **Comprehensive Testing:**  
  Integration tests using Poem's testing utilities along with the `serial_test` crate to run tests sequentially, ensuring consistency.

## Requirements

- [Rust (stable)](https://www.rust-lang.org/)
- Cargo
- The following Rust crates:
  - [Poem](https://crates.io/crates/poem)
  - [serde](https://crates.io/crates/serde) and [serde_json](https://crates.io/crates/serde_json)
  - [serial_test](https://crates.io/crates/serial_test) (dev-dependency for tests)

## Installation

1. **Clone the repository:**

   ```bash
   # clone the repo
   git clone https://github.com/jallenmanaloto/playasia.git
   
   # open directory
   cd playasia

2. **Run the application:**

    ```bash
   # the server will start on the configured address (127.0.0.1:8000)
   cargo run

## API Endpoints

The application exposes the following endpoints:

- **GET /health**  
  Returns a simple health-check response indicating that the service is running.

- **GET /items**  
  Retrieves a list of all items stored in the application’s database (a JSON file).
  
- **GET /items/:id**  
  Retrieves the details of a specific item by its ID.

- **POST /items**  
  Creates a new item.  
  
  **Headers:**  
  - `Content-Type: application/json`  
  - `Authorization: Bearer <token>` (required for non-GET requests)  
  
  **Request Body Example:**  
  ```json
  {
    "name": "Item Name"
  }
  ```
  
  **Response Example:** 
  ```json
  {
    "id": 1,
    "name": "Item Name"
  }
  ```
  
- **PUT /items/:id**  
Updates an existing item. 

    **Headers:**  
    - `Content-Type: application/json`  
    - `Authorization: Bearer <token>` (required for non-GET requests)  
  
  **Request Body Example:**  
  ```json
  {
    "name": "Update Item Name"
  }
  ```
  **Response Example:** 
  ```json
  {
    "id": 1,
    "name": "Update Item Name"
  }
  ```

- **DELETE /items/:id**  
  Deletes an item by its ID.

  **Headers:**  
  - `Content-Type: application/json`  
  - `Authorization: Bearer <token>` (required for non-GET requests)

  **Response Example:** 
  ```json
  {
    "message": "Item deleted successfully"
  }
  ```
  
## Testing

Integration tests for the application are located in the tests directory and cover both the item endpoints and the middleware behavior. Key points include:

- **Item Endpoints**  
  Tests verify that items can be created, retrieved, updated, and deleted as expected.
    - For example, tests create items via POST, then validate their presence via GET, and finally ensure deletion removes them.

- **Middleware**  
  Tests ensure that:
    - GET requests are allowed without an authentication token.
    - Non-GET requests (POST, PUT, DELETE) are rejected unless a valid Bearer token is provided.

- **Running Tests:**  
  To run all tests:
```bash
    cargo test
```

## Author

Jonathan Allen Manaloto

[LinkedIn](https://www.linkedin.com/in/allenmanaloto/)

Email address: jallen.manaloto@gmail.com
