# Competitive Programming (CP) Setup

A customized, robust C++ environment designed for competitive programming (especially on Codeforces). It includes automated fetching of test cases, localized compilation with AC/WA diffing, and resource-measuring run scripts.

## Structure and Tools

### Core Files

- **`template.cpp`**: Boilerplate C++ code with fast I/O and commonly used macros.
- **`code.cpp`**: The primary working file where you write your solution for a single problem.
- **`debug.h`**: A local debugging header that defines the `debug(...)` macro, automatically pretty-printing variables, vectors, and pairs to standard error without cluttering the main output. Disabled automatically when submitted online (via `#ifndef ONLINE_JUDGE`).

### Running & Testing Scripts

- **`cprun.sh`**: The main execution script. It compiles `code.cpp` using Clang++ (`-std=c++17 -O2 -Wall -Wextra -Wshadow`), handles optional precompiled headers (in `bits/stdc++.h.pch`), checks compilation time, and then runs the binary against all test cases.
    - Automatically iterates through `testcases/in*.txt` and compares output with `testcases/out*.txt`.
    - Gives colored AC (Accepted) / WA (Wrong Answer) verdicts.
    - Prints execution time and limits (if `limits.txt` is present).
    - Shows line-by-line colored diffs on WA.
    - Falls back to `input.txt` and `output.txt` if the `testcases/` directory doesn't exist.
- **`edgerun.sh`**: Compiles and runs `code.cpp` specifically against `edgecase.txt`, using `/usr/bin/time` to report detailed time, CPU usage, and memory usage.

### Utility Scripts

- **`new_file.sh <filename>`**: Copies `template.cpp` into a new file named `<filename>`.
- **`edge.sh`**: Quickly paste/type multi-line inputs from the terminal directly into `edgecase.txt`.
- **`writein.sh`**: Quickly paste/type multi-line inputs from the terminal directly into `input.txt`.

### Test Case Fetching (Codeforces)

- **`cf-fetch`**: A custom, compiled Rust binary (source in `cp-fetch/`) designed to scrape and set up Codeforces problems.
    - **Single Problem Mode**: Pass a Codeforces problem URL. It fetches all input/output test cases into a `testcases/` folder and saves the time/memory constraints to `limits.txt`.
    - **Contest Mode**: Pass a Codeforces contest URL. It creates a `Contest_<id>` directory, sets up subdirectories for each problem, copies your `template.cpp` / `cprun.sh` / `debug.h` into them, and downloads all test cases automatically!

## Workflow

### Solving a Single Problem

1. Use `code.cpp` to write your solution.
2. If it's a Codeforces problem, run `./cf-fetch <URL>` to pull the test cases.
3. Run `./cprun.sh` to compile, test, and view verdicts.
4. If you need to test a custom edge case, run `./edge.sh` to paste it, and `./edgerun.sh` to execute your code against it.

### Setting Up a Contest

1. Run `./cf-fetch https://codeforces.com/contest/<id>`
2. Navigate into the generated `Contest_<id>/<Problem_Letter>` folder.
3. Code your solution in `code.cpp`.
4. Test with `./cprun.sh`.

## ⚙️ Compilation Details

The scripts use `clang++` with the following flags by default:

- `-std=c++17`
- `-O2`
- `-Wall -Wextra -Wshadow`

If a precompiled header exists at `bits/stdc++.h.pch`, it is automatically included to speed up compilation times.
