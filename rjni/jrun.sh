#!/usr/bin/env bash
# Created at 2021/7/24 13:45
# @author Liangcheng Juves

base_dir="$(cd "$(dirname "$0")"; pwd)"
run_type=release

dylib_name="$(cat "${base_dir}/Cargo.toml" | grep name | awk 'END{print}' | awk -F '"' '{print $2}')"

cd "${base_dir}"; cargo build --${run_type}

dylib="$(ls -hl "${base_dir}/target/${run_type}" | grep "${dylib_name}" | awk 'END{print}' | awk '{print $9}')"
ls -hl "${base_dir}/target/${run_type}/${dylib}"

once() {
    java_home="$1"

    if [ -d "${java_home}" ]; then
        javac_path="${java_home}/bin/javac"
        java_path="${java_home}/bin/java"

        echo -e "\n"
        "${java_path}" -version
        echo -e "============================================================"
        echo -e "============================================================\n"

        cd "${base_dir}/jcalls"; "${javac_path}" Main.java
        cd "${base_dir}/jcalls"; "${java_path}" -Djava.library.path="${base_dir}/target/${run_type}" Main

        echo -e "\n///////////////////////////////////////////////////////////////"
        echo -e "///////////////////////////////////////////////////////////////"
        echo -e "///////////////////////////////////////////////////////////////"
    fi
}


java_home_array=(
    "$JAVA_HOME"
    "/Users/liangchengj/Downloads/jdk-11.0.11+9/Contents/Home"
    "/Users/liangchengj/Downloads/jdk8u292-b10/Contents/Home"
)

for java_home in ${java_home_array[@]}; do
    once "${java_home}"
done