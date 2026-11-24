# Contract API

## `vote(uri: Bytes, amount: i32, descr: Bytes) -> State`
- `uri`: link identifier (bytes)
- `amount`: positive to upvote/submit, negative to downvote
- `descr`: description bytes (used on new submission)
- Returns updated `State`

Behavior:
- Ignores `amount == 0` or empty `uri`
- New URI + positive amount => creates article
- Existing URI => adds amount to score
- Score <= 0 => article removed

## `refr_state() -> State`
Returns full contract state.

## `get_article(uri: Bytes) -> Option<ArticleData>`
Returns one article if present.

## `article_count() -> u32`
Returns number of stored articles.
