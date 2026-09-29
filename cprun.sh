#!/bin/bash
clear
echo -e "\e[1;36m[ Compiling... ]\e[0m"

CP_DIR="/home/amxnn05/Dev/CP"
PCH_FLAGS=""
if [ -f "$CP_DIR/bits/stdc++.h.pch" ]; then
    PCH_FLAGS="-I$CP_DIR -include-pch $CP_DIR/bits/stdc++.h.pch"
fi

# Compile with time
start_compile=$(date +%s%N)
clang++ -std=c++17 -O2 -Wall -Wextra -Wshadow $PCH_FLAGS code.cpp -o main
compile_status=$?
end_compile=$(date +%s%N)
compile_time=$((($end_compile - $start_compile)/1000000))

if [ $compile_status -ne 0 ]; then
    echo -e "\e[1;31m[ Compilation failed ]\e[0m"
    exit 1
fi

echo -e "\e[1;32m[ Compilation successful! (${compile_time}ms) ]\e[0m\n"

# Check limits
TIME_LIMIT_INFO=""
if [ -f "limits.txt" ]; then
    source limits.txt
    TIME_LIMIT_INFO=" [Limit: $TIME_LIMIT | $MEM_LIMIT]"
fi

echo -e "\e[1;34m[ Running test cases...${TIME_LIMIT_INFO} ]\e[0m\n"

run_test() {
    local in_file=$1
    local out_file=$2
    local test_name=$3

    echo -e "\e[1;33m>>> $test_name\e[0m"

    # Run and capture output and time
    start_time=$(date +%s%N)
    ./main < "$in_file" > my_output.txt
    run_status=$?
    end_time=$(date +%s%N)
    
    t_time_ms=$((($end_time - $start_time)/1000000))
    t_time="${t_time_ms}ms"

    if [ $run_status -ne 0 ]; then
        echo -e "   \e[1;31m[ Runtime Error! ]\e[0m"
        return
    fi

    # Compare
    if [ -f "$out_file" ]; then
        # Trim trailing whitespace and newlines for comparison
        sed -i -e :a -e '/^\n*$/{$d;N;};/\n$/ba' my_output.txt
        sed -i -e :a -e '/^\n*$/{$d;N;};/\n$/ba' "$out_file"
        
        diff -w -B my_output.txt "$out_file" > diff_output.txt
        if [ $? -eq 0 ]; then
            echo -e "   \e[1;32m[ AC ]\e[0m  Time: ${t_time}"
        else
            echo -e "   \e[1;31m[ WA ]\e[0m  Time: ${t_time}"
            echo -e "   \e[1;35m--- Expected ---\e[0m"
            cat "$out_file" | sed 's/^/   /'
            echo ""
            echo -e "   \e[1;35m--- Yours ---\e[0m"
            awk '
              function strip(s) {
                  sub(/[ \t\r]+$/, "", s);
                  return s;
              }
              NR==FNR { expected[NR]=strip($0); next }
              {
                  clean_val = strip($0);
                  if (clean_val == expected[FNR]) {
                      printf "   \033[1;32m%s\033[0m\n", $0
                  } else {
                      printf "   \033[1;31m%s\033[0m\n", $0
                  }
              }
            ' "$out_file" my_output.txt
            echo ""
        fi
    else
        echo -e "   \e[1;33m[ Done (No expected output) ]\e[0m  Time: ${t_time}"
        cat my_output.txt | sed 's/^/   /'
    fi
    echo ""
}

# Find testcases
if [ -d "testcases" ] && [ "$(ls -A testcases/in*.txt 2>/dev/null)" ]; then
    for in_file in $(ls -v testcases/in*.txt); do
        test_idx=$(basename "$in_file" .txt)
        test_idx=${test_idx#in}
        out_file="testcases/out${test_idx}.txt"
        run_test "$in_file" "$out_file" "Test Case $test_idx"
    done
else
    # Fallback
    run_test "input.txt" "output.txt" "Test Case (input.txt)"
fi
