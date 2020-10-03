# TODO

## [Authentication](https://developer.twitter.com/en/docs/authentication/overview)
- [x] POST oauth/access_token
- [ ] GET  oauth/authenticate
- [x] GET  oauth/authorize
- [x] POST oauth/request_token
- [ ] POST oauth2/token
- [x] POST oauth/invalidate_token
- [ ] POST oauth2/invalidate_token
  
## Twitter API v1.1

### Tweets
#### [Post, retrieve, and engage with Tweets](https://developer.twitter.com/en/docs/twitter-api/v1/tweets/post-and-engage/overview)
- [ ] POST statuses/update
- [ ] POST statuses/destroy/:id
- [ ] GET  statuses/show/:id
- [ ] GET  statuses/oembed
- [ ] GET  statuses/lookup
- [ ] POST statuses/retweet/:id
- [ ] POST statuses/unretweet/:id
- [ ] GET  statuses/retweets/:id
- [ ] GET  statuses/retweet_of_me
- [ ] GET  statuses/retweeters/ids
- [ ] POST favorites/create
- [ ] POST favorites/destroy
- [ ] GET  favorites/list
- [ ] ~~POST statuses/update_with_media (deprecated)~~

#### [Get Tweet timelines](https://developer.twitter.com/en/docs/twitter-api/v1/tweets/timelines/overview)
- [ ] GET statuses/home_timeline
- [ ] GET statuses/mentions_timeline
- [ ] GET statuses/user_timeline

#### [Curate a collection of Tweets](https://developer.twitter.com/en/docs/twitter-api/v1/tweets/curate-a-collection/overview)
- [ ] GET  collections/entries
- [ ] GET  collections/list
- [ ] GET  collections/show
- [ ] POST collections/create
- [ ] POST collections/destroy
- [ ] POST collections/entries/add
- [ ] POST collections/entries/curate
- [ ] POST collections/entries/move
- [ ] POST collections/entries/remove
- [ ] POST collections/update

#### [Search Tweets](https://developer.twitter.com/en/docs/twitter-api/v1/tweets/search/overview/standard)
- [ ] Standard search API
- [ ] Enterprise search APIs
- [ ] Premium search APIs

#### [Filter realtime Tweets](https://developer.twitter.com/en/docs/twitter-api/v1/tweets/filter-realtime/overview/statuses-filter)
- [ ] POST statuses/filter
- [ ] PowerTrack API
- [ ] Replay API
- [ ] PowerTrack Rules API 

#### [Sample realtime Tweets](https://developer.twitter.com/en/docs/twitter-api/v1/tweets/sample-realtime/overview/get_statuses_sample)
- [ ] Decahose stream
- [ ] GET statuses/sample

### Users
#### [Manage account settings and profile](https://developer.twitter.com/en/docs/twitter-api/v1/accounts-and-users/manage-account-settings/overview)
- [ ] GET  account/settings
- [ ] GET  account/verify_credentials
- [ ] GET  users/profile_banner
- [ ] POST account/remove_profile_banner
- [ ] POST account/settings
- [ ] POST account/update_profile
- [ ] POST account/update_profile_banner
- [ ] POST account/update_profile_image
- [ ] GET  saved_searches/list
- [ ] GET  saved_searches/show/:id
- [ ] POST saved_searches/create
- [ ] POST saved_searches/destroy/:id

#### [Mute, block, and report users](https://developer.twitter.com/en/docs/twitter-api/v1/accounts-and-users/mute-block-report-users/overview)
- [ ] GET  blocks/ids
- [ ] GET  blocks/list
- [ ] GET  mutes/users/ids
- [ ] GET  mutes/users/list
- [ ] POST blocks/create
- [ ] POST blocks/destroy
- [ ] POST mutes/users/create
- [ ] POST mutes/users/destroy
- [ ] POST users/report_spam

#### [Follow, search, and get users](https://developer.twitter.com/en/docs/twitter-api/v1/accounts-and-users/follow-search-get-users/overview)
- [ ] GET  followers/ids
- [ ] GET  followers/list
- [ ] GET  friends/ids
- [ ] GET  friends/list
- [ ] GET  friendships/incoming
- [ ] GET  friendships/lookup
- [ ] GET  friendships/no_retweets/ids
- [ ] GET  friendships/outgoing
- [ ] GET  friendships/show
- [ ] GET  users/lookup
- [ ] GET  users/search
- [ ] GET  users/show
- [ ] POST friendships/create
- [ ] POST friendships/destroy
- [ ] POST friendships/update

#### [Create and manage lists](https://developer.twitter.com/en/docs/twitter-api/v1/accounts-and-users/create-manage-lists/overview)
- [ ] GET  lists/list
- [ ] GET  lists/members
- [ ] GET  lists/members/show
- [ ] GET  lists/memberships
- [ ] GET  lists/ownerships
- [ ] GET  lists/show
- [ ] GET  lists/statuses
- [ ] GET  lists/subscribers
- [ ] GET  lists/subscribers/show
- [ ] GET  lists/subscriptions
- [ ] POST lists/create
- [ ] POST lists/destroy
- [ ] POST lists/members/create
- [ ] POST lists/members/create_all
- [ ] POST lists/members/destroy
- [ ] POST lists/members/destroy_all
- [ ] POST lists/subscribers/create
- [ ] POST lists/subscribers/destroy
- [ ] POST lists/update

#### [User profile images and banners](https://developer.twitter.com/en/docs/twitter-api/v1/accounts-and-users/user-profile-images-and-banners)

### Direct Messages
#### [Sending and receiving events](https://developer.twitter.com/en/docs/twitter-api/v1/direct-messages/sending-and-receiving/overview)
- [ ] POST   direct_messages/events/new (message_create)
- [ ] GET    direct_messages/events/show
- [ ] GET    direct_messages/events/list
- [ ] DELETE direct_messages/events/destroy

#### [Welcome Messages](https://developer.twitter.com/en/docs/twitter-api/v1/direct-messages/welcome-messages/overview)
- [ ] DELETE direct_messages/welcome_messages/destroy
- [ ] DELETE direct_messages/welcome_messages/rules/destroy
- [ ] GET    direct_messages/welcome_messages/show
- [ ] GET    direct_messages/welcome_messages/rules/show
- [ ] GET    direct_messages/welcome_messages/rules/list
- [ ] GET    direct_messages/welcome_messages/list
- [ ] POST   direct_messages/welcome_messages/new
- [ ] POST   direct_messages/welcome_messages/rules/new
- [ ] PUT    direct_messages/welcome_messages/update

#### [Message attachments](https://developer.twitter.com/en/docs/twitter-api/v1/direct-messages/message-attachments/overview)

#### [Quick Replies](https://developer.twitter.com/en/docs/twitter-api/v1/direct-messages/quick-replies/overview)
- [ ] Location Quick Reply
- [ ] Options Quick Reply
- [ ] Text Input Quick Reply

#### [Buttons](https://developer.twitter.com/en/docs/twitter-api/v1/direct-messages/buttons/overview)
- [ ] Buttons

#### [Typing indicator and read receipts](https://developer.twitter.com/en/docs/twitter-api/v1/direct-messages/typing-indicator-and-read-receipts/overview)
- [ ] POST direct_messages/mark_read
- [ ] POST direct_messages/indicate_typing


#### [Conversation management](https://developer.twitter.com/en/docs/twitter-api/v1/direct-messages/conversation-management/overview)

#### [Custom profiles](https://developer.twitter.com/en/docs/twitter-api/v1/direct-messages/custom-profiles/overview)
- [ ] Send a Direct Message with custom profile
- [ ] DELETE custom_profiles/destroy.json
- [ ] GET    custom_profiles/:id
- [ ] POST   custom_profiles/new.json
- [ ] GET    custom_profiles/list

#### [Customer feedback cards](https://developer.twitter.com/en/docs/twitter-api/v1/direct-messages/customer-feedback/overview)
- [ ] GET feedback/show/:id.json
- [ ] GET feedback/events.json
- [ ] POST feedback/create.json

### Media
#### [Upload media](https://developer.twitter.com/en/docs/twitter-api/v1/media/upload-media/overview)
- [ ] POST media/upload (INIT)
- [ ] POST media/upload (APPEND)
- [ ] GET  media/upload (STATUS)
- [ ] POST media/upload (FINALIZE)
- [ ] POST media/upload
- [ ] POST media/metadata/create
- [ ] POST media/subtitles/delete
- [ ] POST media/subtitles/create

### Trend
#### [Get trends near a location](https://developer.twitter.com/en/docs/twitter-api/v1/trends/trends-for-location/overview)
- [ ] GET trends/place

#### [Get locations with trending topics](https://developer.twitter.com/en/docs/twitter-api/v1/trends/locations-with-trending-topics/overview)

### Geo
#### [Get information about a place](https://developer.twitter.com/en/docs/twitter-api/v1/geo/place-information/overview)
- [ ] GET geo/id/:place_id

#### [Get places near a location](https://developer.twitter.com/en/docs/twitter-api/v1/geo/places-near-location/overview)
- [ ] GET geo/reverse_geocode
- [ ] GET geo/search

## Twitter API v2

### [Tweets lookup](https://developer.twitter.com/en/docs/twitter-api/tweets/lookup/introduction)
- [ ] GET /2/tweets (lookup by list of IDs)
- [ ] GET /2/tweets/:id (lookup by single ID)

### [Users lookup](https://developer.twitter.com/en/docs/twitter-api/users/lookup/introduction)
- [ ] GET /2/users/:id (lookup by single ID)
- [ ] GET /2/users (lookup by list of IDs)
- [ ] GET /2/users/by/username/:username (lookup by single username)
- [ ] GET /2/users/by (lookup by list of usernames)

### [Search Tweets: recent search](https://developer.twitter.com/en/docs/twitter-api/tweets/search/introduction)
- [ ] GET /2/tweets/search/recent

### [Filtered stream](https://developer.twitter.com/en/docs/twitter-api/tweets/filtered-stream/introduction)
- [ ] GET  /2/tweets/search/stream/rules
- [ ] GET  /2/tweets/search/stream
- [ ] POST /2/tweets/search/stream/rules

### [Sampled stream](https://developer.twitter.com/en/docs/twitter-api/tweets/sampled-stream/introduction)
- [ ] GET /2/tweets/sample/stream

### [Hide replies](https://developer.twitter.com/en/docs/twitter-api/tweets/hide-replies/introduction)
- [ ] PUT /2/tweets/:id/hidden
