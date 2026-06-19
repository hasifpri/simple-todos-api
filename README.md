// Get All Todo

curl --location 'http://localhost:3000/todos'


// Create Todo

curl --location 'http://localhost:3000/todos' \
--header 'Content-Type: application/json' \
--data '{
    "title": "Belajar Rust",
    "description": "Buat REST API",
    "is_completed": false
}'


// Get Detail Todo

curl --location 'http://localhost:3000/todos/0c17c55e-8438-4571-9f52-5bd2f3210582'

// Update Todo

curl --location --request PUT 'http://localhost:3000/todos/e7b4a154-ee28-437d-931e-d8c64f76687f' \
--header 'Content-Type: application/json' \
--data '{
    "title": "Belajar Rust Update",
    "description": "Buat REST API Update",
    "is_completed": true
}'


// Delete Todo

curl --location --request DELETE 'http://localhost:3000/todos/e7b4a154-ee28-437d-931e-d8c64f76687f'

// Patch Todo
curl --location --request PATCH 'http://localhost:3000/todos/0c17c55e-8438-4571-9f52-5bd2f3210582/flag-done'
