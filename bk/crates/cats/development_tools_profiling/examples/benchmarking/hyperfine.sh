# Replace these commands with two equivalent ways to run your workload.
hyperfine --warmup 3 --runs 10 'target/release/my_program --input sample.txt' 'target/release/my_program --input sample.txt --optimized'

# Save the measurements so they can be reviewed with the change.
hyperfine --export-markdown results.md --warmup 3 --runs 10 'target/release/my_program --input sample.txt'
