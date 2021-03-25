## **Kitter**

Kitter is a twitter like website written with Rust. [Actix](https://actix.rs/) is the web framework we use. We use [GitHub OAuth2](https://docs.github.com/en/developers/apps/authorizing-oauth-apps) for logging in because I'm honestly lazy.

### ✨ **API** ✨

The base API route is `/api`

The data should be valid JSON and valid JSON will be returned

Format:

```
[REQUEST] ROUTE
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
! content   : string  : the content of the post. this must be less than 512 characters.
? id        : integer : the id of the post
? author    : string  : the author of the post
? author_id : integer : the id of the poster
? content   : string  : the content of the post
? timestamp : string  : the timestamp of when posted

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"content": "owo"}' \
  http://localhost:8083/api/post/add

[POST] /posts
! OPTIONAL
! offset    : integer : how many rows should be skipped, used for paging
! limit     : integer : how many rows should be returned
? LIST OF
? id        : integer : the id of the post
? author    : string  : the author of the post
? author_id : integer : the id of the poster
? content   : string  : the content of the post
? timestamp : string  : the timestamp of when posted

curl --header "Content-Type: application/json" \
  --request GET \
  http://localhost:8083/api/post/posts

[DELETE] /delete
! id      : integer  : the id of the post to be deleted
? id      : integer : the id of the post

curl --header "Content-Type: application/json" \
  --request DELETE \
  --data '{"id": 4}' \
  http://localhost:8083/api/post/delete

```

### ✨ **Information** ✨

Do not insert from the PSQL CLI unless the timezone is set to UTC. This will cause problems if it isn't UTC such as improper times.
