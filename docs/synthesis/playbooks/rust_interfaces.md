# Rust interface failures

Read the trait definition and its associated types directly. Match exact method
signatures, mutability, thread-safety bounds, and error types. Import the trait
when invoking its methods. Prefer candidate-owned adapters over changes to core
traits. Keep the five required public exports and `MetagdbImpl` constructors
exactly as documented. Compile the smallest affected module before broad tests.
