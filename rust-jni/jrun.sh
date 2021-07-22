#!/usr/bin/env bash

base_dir="$(cd "$(dirname "$0")"; pwd)"

cd "${base_dir}"; cargo build --release
cd "${base_dir}/jcalls"; javac HelloWorld.java
cd "${base_dir}/jcalls"; java HelloWorld
