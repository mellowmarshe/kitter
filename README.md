## **Kitter**

Kitter is a twitter like website written with Rust. [Actix](https://actix.rs/) is the web framework we use.

| Service              |   Description    |
| :------------------- | :--------------: |
| [backend](backend)   | The Rust backend |
| [frontend](frontend) | The Vue frontend |

A few concepts/words we use to describe parts of the application:

- Hearts
  - Hearts are what we call likes

### ✨ **API** ✨

The base API route is `/api`

The data should be valid JSON and valid JSON will be returned.

The api is secured with the Authorization header. You will need a token to use the API.

Format:

```
[REQUEST] ROUTE

DESCRIPTION

! PARAM  : TYPE : DESC
? RETURN : TYPE : DESC

Example request using curl
...
```

**Routes for posts**

The routes used to manage posts

These are prefixed with `/post`

```

[POST] /add

Adds a new post

! content       : string    : the content of the post. this must be less than 512 characters.
? id            : integer   : the id of the post
? author        : string    : the author of the post
? author_id     : integer   : the id of the poster
? content       : string    : the content of the post
? hearts        : integer   : the amount of hearts or "likes"
? hearted_users : integer[] : the users that hearted this post
? timestamp     : string    : the timestamp of when posted

curl --header "Content-Type: application/json" \
  --header "Authorization: Bearer TOKEN" \
  --request POST \
  --data '{"content": "owo"}' \
  http://localhost:8083/api/post/add

[POST] /posts

Gets all posts and optionally a limited or offset amount

! OPTIONAL
! offset        : integer   : how many rows should be skipped, used for paging
! limit         : integer   : how many rows should be returned
? LIST OF
? id            : integer   : the id of the post
? author        : string    : the author of the post
? author_id     : integer   : the id of the poster
? content       : string    : the content of the post
? hearts        : integer   : the amount of hearts or "likes"
? hearted_users : integer[] : the users that hearted this post
? timestamp     : string    : the timestamp of when posted

curl --header "Content-Type: application/json" \
  --header "Authorization: Bearer TOKEN"
  --request POST \
  http://localhost:8083/api/post/posts

[POST] /heart

Toggles the heart status on a post for a user

! id            : integer   : the id of the post to be hearted
? id            : integer   : the id of the post
? author        : string    : the author of the post
? author_id     : integer   : the id of the poster
? content       : string    : the content of the post
? hearts        : integer   : the amount of hearts or "likes"
? hearted_users : integer[] : the users that hearted this post
? timestamp     : string    : the timestamp of when posted

curl --header "Content-Type: application/json" \
  --header "Authorization: Bearer TOKEN"
  --request POST \
  --data '{"id": 1}' \
  http://localhost:8083/api/post/heart

[DELETE] /delete

Deletes a post

! id      : integer  : the id of the post to be deleted
? id      : integer : the id of the post

curl --header "Content-Type: application/json" \
  --header "Authorization: Bearer TOKEN"
  --request DELETE \
  --data '{"id": 1}' \
  http://localhost:8083/api/post/delete

```

**Routes for users**

The routes used to manage users

These are prefixed with `/user`

```
Requirements:

Usernames must be between 3 and 32 characters. They can contain 1-9, a-Z, . and \_. They must start with a-Z.

Password must be atleast 8 characters with an uppercase, lowercase and number. Special characters are allowed.

Emails must be a valid email.
```

```

[POST] /register

Register a new user

! username  : string  : the username of the user, must meet requirements listed above
! email     : string  : the email of the user
! password  : string  : the password of the user, must meet requirements listed above
? id        : integer : the id of the user
? username  : string  : the username of the user
? email     : string  : the email of the user
? password  : string  : the password of the user
? timestamp : string  : the timestamp of creation

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"username": "domterion", "email": "dominic@domm.me", "password": "eI*y0K9UBIUjjWh*j@eIGgHL"}' \
  http://localhost:8083/api/user/register

[POST] /login

Log a user in

! username  : string  : the username of the user
! password  : string  : the password of the user
? token     : string  : the JWT, this is used in authentication for the api
? type      : string  : the type of the token, usually bearer

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"username": "domterion", "password": "eI*y0K9UBIUjjWh*j@eIGgHL"}' \
  http://localhost:8083/api/user/login

[GET] /me

Get information about the user

? id        : integer : the id of the user
? username  : string  : the username of the user
? email     : string  : the email of the user
? password  : string  : the password of the user. this will be blank and yes its on purpose.
? timestamp : string  : the timestamp of creation

curl --header "Content-Type: application/json" \
  --header "Authorization: Bearer TOKEN"
  --request GET \
  http://localhost:8083/api/user/me
```

### ✨ **Information** ✨

Do not insert from the PSQL CLI unless the timezone is set to UTC. This will cause problems if it isn't UTC such as improper times.
