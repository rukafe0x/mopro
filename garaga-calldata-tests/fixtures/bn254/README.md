# BN254 Garaga calldata golden fixtures

Artifacts for checking `generate_circom_groth16_garaga_calldata` against the
independent Python implementation in
[Garaga v1.1.0](https://github.com/keep-starknet-strange/garaga/tree/v1.1.0).

| File | Role |
|------|------|
| `proof.json` | Test fixture only — builds a `CircomProofResult` for golden tests (not a runtime API input) |
| `public.json` | Test fixture only — public inputs for the fixture `CircomProofResult` |
| `verification_key.json` | Required at runtime (one-time snarkjs zkey export) |
| `expected_garaga_calldata.json` | Golden vector; regenerate from Garaga v1.1.0's Python CLI as described below. The Rust adapter generator is diagnostic only. |

Regenerate the golden vector after changing parsers or bumping Garaga. Run from the repository root; pin the Python package to the same Garaga version as the Rust dependency:

```bash
python3 -m pip install garaga==1.1.0
set -o pipefail
garaga calldata \
	--system groth16 \
	--vk garaga-calldata-tests/fixtures/bn254/verification_key.json \
	--proof garaga-calldata-tests/fixtures/bn254/proof.json \
	--public-inputs garaga-calldata-tests/fixtures/bn254/public.json \
	--format starkli |
	python3 -c 'import json, sys; json.dump(sys.stdin.read().split(), sys.stdout, indent=2); print()' \
	> garaga-calldata-tests/fixtures/bn254/expected_garaga_calldata.json
cargo test -p garaga-calldata-tests
```

The Garaga CLI's `starkli` output includes the leading calldata length felt, matching the golden fixture. Do not regenerate the fixture with the Rust adapter: its output shares the conversion path under test.

To compare the Rust adapter output with the independent fixture without modifying it:

```bash
cargo run -p garaga-calldata-tests --bin gen-garaga-calldata-fixture \
	> /tmp/garaga-calldata-rust.json
diff -u garaga-calldata-tests/fixtures/bn254/expected_garaga_calldata.json \
	/tmp/garaga-calldata-rust.json
```
