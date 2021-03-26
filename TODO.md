## Features

- [x] Load new set of posts whenever current set is near end. So basically continuous paging.

  - Above is about done, I dont know if the ordering is wrong but I think It's good. We can't test this right now to ensure I'm not aimlessly testing.
  - We are keeping the actual load button for now encase our scroll detector doesnt work.

- [ ] User profiles... load GitHub bio on first login to description and allow changing.

- [ ] Add profile picture support via GitHub.

- [ ] MAYBE site staff badge on tweets or special color.

- [ ] Allow following users so you only see their tweets. Site staff tweets will always be seen no matter if following or not.

- [x] Add liking to posts...

  - This has been done but we are calling them hearts on kitter.
  - You are able to heart your own post and I intend on keeping this.

- [ ] Comments and nested comments... hopefully.

- [ ] Eventually a UI overhaul... most likely with React or Vue.

## Quality of Life

- [ ] Hide Load button if theres not all posts there or no posts.

- [x] Add a "SYSTEM" message if no posts are found.

- [x] Dont show like button on SYSTEM message.

- [ ] Mobile.. this is in QOL because I am honestly too lazy to prioritize it... sorry mobile users.

- [ ] Change heart button color if you like a post.

## Bugs

- [x] A bug with hearts where spam clicking would either subtract or add one to the total count.
  - Fix was to loosely base count off the hearted_users array
