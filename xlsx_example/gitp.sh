#!/usr/bin/env sh
# Created at 2021/4/6 21:26
# @author Liangcheng Juves

tm=$(date "+%Y/%m/%d %H:%M")
base_dir="$(cd "$(dirname "$0")"; pwd)"

git -C ${base_dir} add ${base_dir}
git -C ${base_dir} commit -m "Updated at ${tm}" --author "Liangcheng Juves <liangchengj@outlook.com>"
git -C ${base_dir} push -u origin main
