# hashed

Convert any hashable type into a much smaller `Hashed<T>` which still supports checking equality.

All that is stored inside the `Hashed<T>` is the `u64` hash of the type, so this can save a lot of space over storing objects themselves, while being more convenient than hashing manually and with added type safety. The downside is, of course, *the only thing you can do with the resulting `Hashed<T>` is check if it is equal to another `Hashed<T>`*. No magic here.
