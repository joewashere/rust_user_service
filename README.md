
# Rust Rocket JWT Authentication

A Rust-based web application utilizing the Rocket framework to provide a simple user registration, login, and deletion API with JWT (JSON Web Token) for authentication.

## Features

- **User Registration**: Allows new users to register with a username and password.
- **User Login**: Authenticates users and provides a JWT for subsequent requests.
- **User Deletion**: Allows users to delete their account using JWT for validation.
- **JWT Authentication**: Uses JWTs for authenticating user requests.

## Dependencies

- Rocket: For web framework functionalities.
- rusqlite: For SQLite database interactions.
- bcrypt: For hashing and verifying passwords.
- jsonwebtoken: For JWT encoding and decoding.
- serde_json: For JSON serialization and deserialization.

## Setup & Installation

1. Ensure you have Rust and Cargo installed. If not, [install Rust](https://www.rust-lang.org/learn/get-started).
2. Clone the repository:
   ```bash
   git clone [your-repo-link]
   cd [your-repo-directory]
   ```
3. Install the necessary dependencies:
   ```bash
   cargo install
   ```
4. Run the application:
   ```bash
   cargo run
   ```

## API Endpoints

- **POST /v1/register**: Register a new user.
  - Request Body: 
    ```json
    {
      "username": "your_username",
      "password": "your_password"
    }
    ```
- **POST /v1/login**: Authenticate a user.
  - Request Body: 
    ```json
    {
      "username": "your_username",
      "password": "your_password"
    }
    ```
  - Response: JWT token on successful authentication.
- **DELETE /v1/delete**: Delete a user account.
  - Headers: 
    ```plaintext
    Authorization: Bearer [your_jwt_token]
    ```
  - Request Body:
    ```json
    {
      "username": "your_username"
    }
    ```
