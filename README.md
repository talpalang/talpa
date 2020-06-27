Temp repo for a programing language made to be compiled into other languages.

Currently almost nothing works yet :),
Currently all code is written around tests so to add more code also requires adding more tests.
Tests can be exuecuted using:
```bash
# run all tests
cargo test

# run a spesific test
cargo test test_nothing
```

Some design goals i think are important:
- Preferably no dependencies
- No non cargo tools required to build this project *([By installing rust](https://www.rust-lang.org/tools/install) you should have everything to get started working on this though i might require rust nightly if needed)*
