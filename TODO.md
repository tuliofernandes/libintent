- [X] lib's core types
- [X] minimally functional dispatcher

- [X] the pilot, model intent (fs.list)
    - [X] functional core
    - [X] a format (json), implemented separatedly, outside the intents scope

- [X] the web.search intent

- some initial formats (json, plain, source, etc.)

## Packaging libintent docs (octopus repo)

To ship libintent man pages as part of the octopus package:

- [ ] In the octopus build/packaging step, locate libintent's `man/man1/` directory
      (available on disk after `cargo fetch` since it is part of the crate source tree).
- [ ] Run `scdoc` on each `.scd` file to produce the compiled `.1` roff files:
      `scdoc < web.search.1.scd > web.search.1`
- [ ] Install the compiled `.1` files alongside octopus's own man pages under
      `/usr/share/man/man1/` (or `$(DESTDIR)/usr/share/man/man1/` for staged installs).
- [ ] Add `scdoc` as a build-time dependency in the octopus package spec
      (`.deb` `Build-Depends`, `.spec` `BuildRequires`, or equivalent).
- [ ] Optionally gzip the `.1` files (`gzip -9`) — most distro packaging tools do
      this automatically; check whether `cargo-deb` / `cargo-generate-rpm` handle it.

Note: each new intent added to libintent must include its own `.scd` file in
`man/man1/`; the octopus packaging step picks them all up automatically with a glob.

## Tech Debt
- [ ] Implement a generic HTTP client inside the web infra, in order to decouple request details from the intents.
- [ ] Lock down infra visibility — make `pub(crate) mod infras` in `lib.rs` so `List`, `Search`,
      and all infra structs are inaccessible outside the crate.
      Blocked by: `tests/infras/fs/list/list.rs` and `tests/infras/web/search/search.rs`
      calling intent structs directly via `List.execute()` / `Search.execute()`.
      Fix: migrate those tests to go through `Dispatcher::with_defaults()` + `dispatch()`.

- [ ] Hide internal helpers `parse_brave_results` and `extract_page_text` in `search.rs`
      (remove `pub`). Blocked by direct calls in `tests/infras/web/search/search.rs`.
      Fix: once tests are migrated to dispatch-based assertions, drop `pub` from both functions.