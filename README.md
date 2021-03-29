## **Kitter**

Kitter is a twitter like website written with Rust. [Actix](https://actix.rs/) is the web framework we use.

A few concepts/words we use to describe parts of the application:

- Hearts
  - Hearts are what we call likes

### ✨ **API** ✨

The base API route is `/api`

The data should be valid JSON and valid JSON will be returned.

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
  --request POST \
  --data '{"id": 1}' \
  http://localhost:8083/api/post/heart

[DELETE] /delete

Deletes a post

! id      : integer  : the id of the post to be deleted
? id      : integer : the id of the post

curl --header "Content-Type: application/json" \
  --request DELETE \
  --data '{"id": 1}' \
  http://localhost:8083/api/post/delete

```

### ✨ **Information** ✨

Do not insert from the PSQL CLI unless the timezone is set to UTC. This will cause problems if it isn't UTC such as improper times.
