#!/bin/bash
clear
echo "🔧 Compiling..."
clang++ -std=c++17 -O2 -Wall code.cpp -o main

if [ $? -eq 0 ]; then
    echo -e "\n🚀 Running with edgecase.txt...\n"
    /usr/bin/time -f "\n \n 📊 Stats → Time: %E | CPU: %P | Mem: %M KB" ./main < edgecase.txt
else
    echo "❌ Compilation failed."
fi
