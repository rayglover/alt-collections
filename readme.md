## What?

Some performance experiments based on the existing Rust standard collections.

## Benchmarks

Run `cargo bench`. Benchmarks can alternatively be run against the `std` implementation by adding the `--features std` option.

#### BitSet
For reference, sparse = ~10%, and dense = ~50% of set membership.

		std::collections::BitSet / alt_collections::BitSet

	copy_dense         ... 3.08x
	copy_sparse        ... 4.22x
	count_dense        ... 11.01x
	count_sparse       ... 8.11x
	from_bytes         ... 1.47x
	intersect_dense    ... 6.54x
	intersect_sparse   ... 4.37x
	union_dense        ... 5.53x
	union_sparse       ... 5.60x

		alt_collections::BitSet

	copy_dense         ... bench:   1309376 ns/iter (+/- 84800)
	copy_sparse        ... bench:    321634 ns/iter (+/- 10254)
	count_dense        ... bench:    294378 ns/iter (+/- 10501)
	count_sparse       ... bench:    150750 ns/iter (+/- 6494)
	from_bytes         ... bench:    146491 ns/iter (+/- 5281)
	intersect_dense    ... bench:    353198 ns/iter (+/- 16254)
	intersect_sparse   ... bench:    185930 ns/iter (+/- 5348)
	union_dense        ... bench:    537065 ns/iter (+/- 20711)
	union_sparse       ... bench:    342698 ns/iter (+/- 13529)

		std::collections::BitSet

	copy_dense         ... bench:   4033282 ns/iter (+/- 139366)
	copy_sparse        ... bench:   1357146 ns/iter (+/- 44573)
	count_dense        ... bench:   3242001 ns/iter (+/- 704605)
	count_sparse       ... bench:   1222973 ns/iter (+/- 45306)
	from_bytes         ... bench:    215923 ns/iter (+/- 4753)
	intersect_dense    ... bench:   2309236 ns/iter (+/- 93222)
	intersect_sparse   ... bench:    806542 ns/iter (+/- 306508)
	union_dense        ... bench:   2967318 ns/iter (+/- 96139)
	union_sparse       ... bench:   1919793 ns/iter (+/- 393740)
