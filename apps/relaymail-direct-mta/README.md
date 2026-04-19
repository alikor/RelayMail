# relaymail-direct-mta (future)

Planned RelayMail service that sends mail directly over SMTP, acting as a
self-managed MTA.

**Not implemented in this phase.** See [FUTURE.md](FUTURE.md) and
[../../docs/future-direct-mta.md](../../docs/future-direct-mta.md) for the
intended scope.

When implemented, this service will consume the same capability traits
defined in `relaymail-core` (`ObjectStore`, `MessageSource`, `IdempotencyStore`)
and `relaymail-delivery` (`EmailSender`) so the rest of the monorepo does not
need to be aware of the delivery mechanism.
