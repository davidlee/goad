# Slint's markdown subset rejects headings, images, and most block-level structure

Learned at slice 002, PHASE-04, measured on contact
(`i-slint-common-1.17.1/styled_text.rs`, `unsupported_tag_name`).

## The fact

`StyledText::from_markdown` (Slint 1.17.1) rejects: headings, images, block
quotes, code blocks, tables, HTML blocks, footnotes, definition lists, and
super/subscript — each with its own named error
(`StyledTextFromMarkdownError`, e.g. "Markdown headings are not
supported"). It accepts: plain paragraphs, emphasis/strong, lists, and
links.

A test corpus built from an "obviously fine" Markdown sample (e.g. one that
opens with `"# Heading"`) fails immediately — this was found when a
mapper test's "accepted" corpus included a heading and got a rejection
instead of a parse.

Multiple parse errors in one document are joined with a real `\n` in
`StyledTextFromMarkdownError`'s message (confirmed with a heading plus a
block quote in the same input) — not a synthetic separator assumed from
reading the library's source comment.

## Why it matters here

This is exactly the boundary AC-9 exists to cover: a body the parser
rejects must still be shown (degraded to plain text) and the degradation
reported, never silently dropped and never used as grounds to refuse the
whole view.

## How to apply

- When building a Markdown test corpus against Slint, split it explicitly
  into an accepted set (paragraphs, emphasis/strong, lists, links) and a
  rejected set (headings, images, quotes, code blocks, tables, HTML,
  footnotes, definition lists, sub/superscript) — do not assume a
  "simple-looking" sample is accepted without checking it against this
  list.
- A rejected body is a case for the degrade-and-report path (`Undrawn::
  MarkdownUnsupported`), not a bug in the mapper.
