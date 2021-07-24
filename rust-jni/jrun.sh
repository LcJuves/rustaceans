#!/usr/bin/env bash
# Created at 2021/7/24 13:45
# @author Liangcheng Juves

base_dir="$(cd "$(dirname "$0")"; pwd)"
run_type=release

cd "${base_dir}"; cargo build --${run_type}
cd "${base_dir}/jcalls"; javac HelloWorld.java
cd "${base_dir}/jcalls"; java -Djava.library.path="${base_dir}/target/${run_type}" HelloWorld