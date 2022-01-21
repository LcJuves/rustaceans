#!/usr/bin/env bash
# Created at 2021/7/28 21:48
# @author Liangcheng Juves

kill -9 "$(netstat -tlnp | grep 'hello-rocket' | awk -F "/" '{print $1}' | awk '{print $7}')" 2>/dev/null
