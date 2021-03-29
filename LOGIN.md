## Login system

### Two routes under /user:

```

[POST] /register

Register a new user

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"username": "domterion", "email": "dominic@domm.me", "password": "eI*y0K9UBIUjjWh*j@eIGgHL"}' \
  http://localhost:8083/api/user/register

[POST] /login

Log a user in

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"username": "domterion", "password": "eI*y0K9UBIUjjWh*j@eIGgHL"}' \
  http://localhost:8083/api/user/login
```

### Requirements:

Usernames must be between 3 and 32 characters. They can contain 1-9, a-Z, . and \_. They must start with a-Z.

Password must be atleast 8 characters with an uppercase, lowercase and number. Special characters are allowed.

Emails must be a valid email.

### Handler for register:

Hash the password with bcrypt crate
Maybe use bcrypt to salt

Valid:

Password must be 8-16 characters with atleast one symbol and one uppercase character.

Usernames cant be more than 32 characters long and only a-Z + \_

Verify emails with regex

Functions:

Maybe make a utils folder to separate util functions:
