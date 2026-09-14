# xmip-core-retain

Retention: what a gate refused and what policy says to keep, held while it is
live, and what happens to it over time. `RetentionPolicy` answers with one of
two actions — keep, or archive — because Xmip retains and archives and never
deletes (ADR-0040); what becomes of an archive is the archive owner's decision.

Retain is in the message path: a Journey waits for it, which is why it is a
Capability and not an Operation (`repository-model.md` section 1). It is not
audit — audit holds no payloads, retention is the one of the five that does
(`observability-model.md` section 1) — and it is not the archive, which moves
already-retained data on a schedule.

ADR-0013 clause 2 makes it the owner of retained Streams and ADR-0040 ends
retention at archiving; each store is a technology mounted under this
repository, and `architecture.toml` names them.
