# Sambook MVP

## Goal

Build the smallest useful version of Sambook that proves the writing loop:

1. A user receives three Korean words.
2. The user writes a short story.
3. The user saves it as public or private.
4. Public stories can be browsed.
5. Stories using the same word set can be compared.

## MVP Features

### Prompt Generation

- Generate three random Korean words.
- Show the selected words clearly during writing.
- Allow rerolling the prompt before writing.
- Store the exact word set used by each story.

### Story Writing

- Provide a focused editor.
- Support optional title.
- Support body text.
- Show character count.
- Show whether each prompt word appears in the body.
- Support visibility: public or private.

### Story Storage

- Save story title, body, visibility, author, word set, and timestamps.
- Preserve the original three words even if the word bank changes later.
- Allow editing only by the author.

### Public Reading

- Show public story list.
- Show story detail.
- Show all public stories written from the same word set.
- Hide private stories from everyone except the author.

### Account

- Support basic sign-in.
- A signed-in user can write and save.
- Anonymous users can read public stories and try prompts locally.

## Non-MVP

These are intentionally delayed:

- comments
- follows
- notifications
- full text search
- recommendations
- contests
- AI-generated writing
- mobile app
- payments
- moderation dashboard

## First Content Target

The first seed should include at least 1,000 Korean words.

The seed should avoid only abstract nouns. It needs many concrete words that can create scenes:

- objects
- places
- occupations
- weather
- body actions
- emotions
- strange nouns
- genre-friendly words

## MVP Success Criteria

The MVP is successful if:

- a user can write a story within one minute of opening the app
- the same word set can lead to multiple public stories
- private writing still feels useful without other users
- the product identity is clear without a long explanation

