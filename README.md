# AXUM + PostgresSQL CRUD Application

## For testing

- ### Create user:
```bash
curl --location '127.0.0.1:7878/users' \
--header 'Content-Type: application/json' \
--data '{
    "name": "Kirill",
    "age": 36
}'
```

- ### Get all:
```bash
  curl --location '127.0.0.1:7878/users'
```

- ### Get one:
```bash
curl --location '127.0.0.1:7878/users/1'
```

- ### Update user:
```bash
curl --location --request PATCH '127.0.0.1:7878/users/3' \
--header 'Content-Type: application/json' \
--data '{
    "name": "John Dow",
    "age": 35
}'
```

- ### Delete user:
```bash
curl --location --request DELETE '127.0.0.1:7878/users/3' \
--data ''
```