# Blog admin panel

The resumed Claude session approved one password-protected administrator, selectable existing authors, image URLs, and a Dioxus admin inside the existing app. The current request is to finish that work.

## Behavior

- `/admin` lists drafts and published posts, with search and status filters.
- `/admin/new` and `/admin/edit/:id` provide a Markdown editor, metadata fields, and a live preview using the public renderer and CSS. The preview runs in a sandboxed iframe so pasted HTML cannot execute scripts in the admin session.
- Saving preserves publication status. Publishing and unpublishing are explicit actions. Deleting requires confirmation. A styled, keyboard-accessible dialog protects unsaved edits when following links, using Back/Forward, or signing out.
- Title, summary, body, author, topic, tags, header image URL, and contact CTA are editable. Image uploading is outside the approved scope.
- Slugs and reading time remain database-managed. Initial publication uses the publication time; subsequent edits preserve the existing date. Draft pages and public APIs return no draft content.

## Security and configuration

Authentication fails closed without a valid Argon2id password hash and a session secret of at least 32 bytes. Signed sessions expire after 30 days and rotate when the hash or secret changes. Cookies are HttpOnly, SameSite=Strict, Secure over HTTPS, and host scoped. HTTP is allowed only for loopback development.

Every protected server function checks the session before accessing the database. Mutations, including login, require the configured origin. Login attempts have per-address and global limits, with bounded password verification concurrency. Admin pages and APIs are private, uncached, and excluded from indexing.

Tests use an isolated local SurrealDB, never the production database. No production data is changed by verification.

The schema adds one optional `first_published_at` field. This is needed to distinguish a never-published draft from an unpublished article, preserving its original date on republishing. Existing published posts retain their date, and the field is populated when their publication status is first changed.

## Implementation and verification

1. Authentication: session signing, expiration, cookie parsing, login throttling, origin checks; unit tests for forgery, expiry, rotation, and rate limits.
2. Post operations: parameterized CRUD against the existing schema, field validation, published-only public reads; local DB tests for save, publish, unpublish, delete, and missing records.
3. UI: guarded admin layout, post list, responsive editor, sandboxed preview, explicit save/publication/delete actions and unsaved edit protection.
4. Delivery: setup helper and README instructions, shared and server test suites, wasm/server build, browser workflow at desktop and mobile sizes, independent review.

Review focus: unauthenticated draft access, CSRF on every mutation, stale preview responses, accidental publication on save, and publication date preservation.

The independent review found two issues, both corrected: Back/Forward could discard edits, and a failed publication could discard the successfully saved draft's ID. History traversal now waits for an explicit dialog decision, preserving Dioxus scroll state. Publishing retains the saved draft before making the publication request, so retrying uses the same record. A local injected publication failure reproduced two duplicate drafts before the fix and one retained draft after it.

HTTP verification covers every protected endpoint without a session, cross-origin mutations, CRUD, validation, hidden draft pages, preserved publication dates, and logout. Admin server functions use typed errors to preserve HTTP status codes and return generic messages for unexpected server failures.

Final verification passed: 44 server-feature Rust tests, 20 shared tests, 5 asynchronous navigation/dialog tests, wasm compilation, and a fullstack build. Chrome checks cover Back cancel/confirm, Forward cancel, sign-out cancel, keyboard save, Markdown/code/math preview, and a 390px layout without horizontal overflow. Preview documents remount the sandboxed iframe so updates do not add entries to the [shared browser session history](https://html.spec.whatwg.org/dev/browsing-the-web.html).
