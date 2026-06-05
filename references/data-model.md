# Sambook Data Model

## Principles

Stories must preserve the exact three words that generated them.

The word bank can evolve over time, but old stories should not change meaning when a word entry is edited, removed, or retagged.

## Core Tables

## users

Stores account identity.

| Column | Type | Notes |
| --- | --- | --- |
| id | uuid | primary key |
| email | text | unique |
| display_name | text | visible writer name |
| image_url | text | optional |
| created_at | timestamp | account creation time |
| updated_at | timestamp | last profile update time |

## words

Stores the curated Korean word bank.

| Column | Type | Notes |
| --- | --- | --- |
| id | uuid | primary key |
| text | text | Korean word |
| part_of_speech | text | noun, verb, adjective, adverb, etc. |
| difficulty | text | easy, normal, hard |
| mood | text | calm, strange, sad, comic, tense, etc. |
| genre_tags | text[] | fantasy, horror, romance, sf, mystery, daily, etc. |
| concreteness | text | concrete, mixed, abstract |
| safety_level | text | normal, sensitive, blocked |
| is_active | boolean | can be used in generation |
| created_at | timestamp | creation time |
| updated_at | timestamp | last update time |

## word_sets

Stores a generated three-word prompt.

| Column | Type | Notes |
| --- | --- | --- |
| id | uuid | primary key |
| word_1_id | uuid | optional link to words |
| word_2_id | uuid | optional link to words |
| word_3_id | uuid | optional link to words |
| word_1_text | text | preserved text |
| word_2_text | text | preserved text |
| word_3_text | text | preserved text |
| source | text | random, daily, curated |
| daily_date | date | set only for daily prompt |
| normalized_key | text | stable key for same-set lookup |
| created_at | timestamp | generation time |

## stories

Stores user writing.

| Column | Type | Notes |
| --- | --- | --- |
| id | uuid | primary key |
| author_id | uuid | references users |
| word_set_id | uuid | references word_sets |
| title | text | optional |
| body | text | story text |
| visibility | text | public or private |
| character_limit | integer | selected writing mode |
| contains_word_1 | boolean | body includes first prompt word |
| contains_word_2 | boolean | body includes second prompt word |
| contains_word_3 | boolean | body includes third prompt word |
| published_at | timestamp | null for private drafts |
| created_at | timestamp | creation time |
| updated_at | timestamp | last edit time |

## story_likes

Stores lightweight reactions.

| Column | Type | Notes |
| --- | --- | --- |
| story_id | uuid | references stories |
| user_id | uuid | references users |
| created_at | timestamp | like time |

Unique key: `(story_id, user_id)`.

## story_bookmarks

Stores saved stories.

| Column | Type | Notes |
| --- | --- | --- |
| story_id | uuid | references stories |
| user_id | uuid | references users |
| created_at | timestamp | bookmark time |

Unique key: `(story_id, user_id)`.

## Important Indexes

- `stories(visibility, created_at)`
- `stories(author_id, created_at)`
- `stories(word_set_id, visibility, created_at)`
- `word_sets(normalized_key)`
- `word_sets(daily_date)`
- `words(text)`
- `words(is_active)`

## Same-Set Lookup

The same three words should match even if the order changes.

Example:

- `고래`, `엘리베이터`, `유언장`
- `유언장`, `고래`, `엘리베이터`

Both can share a normalized key:

```text
고래|엘리베이터|유언장
```

The normalized key should sort the three words by a stable Korean collation strategy before joining them.

## Agent Usage Note

Use this model as product and persistence guidance. Do not assume Sambook needs app code unless the user explicitly requests implementation.

When implementation is requested, preserve the product rules first: stories keep the exact three words, same-word-set lookup is stable, and private stories remain hidden from public browsing.
