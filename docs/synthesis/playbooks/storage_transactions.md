# Storage and transaction failures

Separate logical visibility from physical durability. Test begin/read/write,
commit, rollback, and conflicting transactions deterministically. For persistent
designs, close and reopen a fresh instance and assert committed data survives
while incomplete work does not. Verify ID allocation, both adjacency directions,
index updates, WAL ordering, and recovery idempotence. Match the concurrency and
isolation choices in the frozen design vector; do not silently downgrade them.
