---
name: sambook
description: Korean-first Sambook co-creation skill for shaping or running a three-word short-story writing playground. Use when the user wants to get three random Korean words, write a short story, save dated writing results, evolve Sambook as an agent-assisted creative product, define prompts or word banks, refine writing loops, update MVP or data-model direction, create product specs, or decide whether to prototype or implement Sambook.
---

# Sambook

Sambook is a Korean-first writing playground where people write short stories from three Korean words and compare how others used the same words.

Treat Sambook as an agent-assisted creative product, not a generic web app scaffold. Unless the user explicitly asks for implementation code, produce product decisions, writing-system artifacts, word-bank rules, UX flows, specs, or experiments.

## Workflow

1. Identify the artifact the user wants: product direction, prompt design, word-bank curation, writing rules, MVP scope, data model, UX flow, experiment plan, or implementation plan.
2. Load only the relevant reference:
   - `references/product-direction.md` for product identity, target feeling, and non-goals.
   - `references/mvp.md` for smallest useful scope and deferred features.
   - `references/data-model.md` for persistence concepts and same-word-set behavior.
3. Respond in Korean by default unless the user asks otherwise.
4. Keep outputs concrete: examples, lists, rules, short specs, or next-step plans.
5. Ask at most one clarifying question when the target artifact is unclear; otherwise make a conservative assumption and proceed.

## Writing Session

Use this flow when the user wants to write with Sambook:

1. Generate exactly three Korean words. Prefer concrete, scene-producing words over only abstract nouns.
2. Show the three words clearly and invite the user to write a short piece.
3. If the user provides story text, save it under `result/YYYY-MM-DD/`.
4. Use a filename like `HHmmss-word1-word2-word3.md`. Keep filenames short and filesystem-safe.
5. Store the result as Markdown with this shape:

```markdown
---
date: YYYY-MM-DD
words:
  - word1
  - word2
  - word3
---

# Optional Title

story body
```

If the user asks for another prompt, reroll all three words. If the user provides a title, include it; otherwise use `# Untitled`.

`result/` is local writing output and should stay ignored by Git.

## Product Guardrails

- Preserve the core loop: get three Korean words, write a short story, save privately or publicly, compare same-word-set stories.
- Keep writing light. Prefer short-session mechanics over productivity-tool framing.
- Prioritize same-prompt comparison over generic social feeds.
- Keep Sambook distinct from AI writing generators. The agent may help shape the system, but the product promise is human writing from prompts.
- Treat public/private memory as essential: private writing should remain useful even without a community.
- Use Korean examples and Korean literary nuance when producing prompts, labels, or sample content.

## When Implementation Is Requested

If the user explicitly asks to build software, first state the implementation boundary and use the references to create a narrow plan. Preserve the product intent before choosing frameworks, routes, schemas, or UI details.

Do not recreate a full app scaffold just because the user asks to think through Sambook. Code is an output only when directly requested.
