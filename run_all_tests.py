#!/bin/python
import os

prev_dir = os.getcwd()

def pushd(dir):
    prev_dir = os.getcwd()
    os.chdir(dir)

def popd():
    os.chdir(prev_dir)

COL='\033[1;34m' # Blue Color
NC='\033[0m' # No Color

print("Module: " + COL + prev_dir.rsplit('/', 1)[-1] + NC)
os.system('cargo test -q')

for next_dir in os.listdir("."):
    if os.path.exists(os.path.join(next_dir, "Cargo.toml")):
        pushd(next_dir)
        print("Module: " + COL + next_dir + NC)
        os.system('cargo test -q --lib')
        popd()