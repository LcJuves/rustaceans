#!/usr/bin/env sh
# Created at 2021/7/27 13:10
# @author Liangcheng Juves

base_dir="$(cd "$(dirname "%0")"; pwd)"

"${base_dir}/hello-rocket" > "${base_dir}/hello-rocket.log" 2>&1 &
