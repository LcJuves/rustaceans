#!/usr/bin/env bash
# Created at 2021/7/25 14:45
# @author Liangcheng Juves

base_dir="$(cd "$(dirname "$0")"; pwd)"

is_macos() {
    if [[ ! "$(uname -o)" =~ "Linux" ]]; then
        return 1;
    else
        return 0;
    fi
}

once() {
    java_home="$1"

    if [ -d "${java_home}" ]; then
        # Clean
        rm -rf "${base_dir}/${dylib}"
        rm -rf "${base_dir}/CallJNI.h"

        javac_path="${java_home}/bin/javac"
        java_path="${java_home}/bin/java"

        echo -e "\n"
        "${java_path}" -version
        echo -e "\n"

        cd "${base_dir}/../jcalls"; "${javac_path}" -encoding utf8 -h "${base_dir}" CallJNI.java

        if [ $(is_macos) ]; then
            gcc -dynamiclib -I "${base_dir}" "${base_dir}/main.c" -o "${base_dir}/${dylib}"
        else
            gcc -I "${base_dir}" -fPIC -shared -o "${base_dir}/${dylib}" "${base_dir}/main.c"
        fi

        ls -hl "${base_dir}/${dylib}"

        echo -e "\n============================================================"
        echo -e "============================================================\n"

        cd "${base_dir}/../jcalls"; "${javac_path}" Main.java
        cd "${base_dir}/../jcalls"; "${java_path}" -Djava.library.path="${base_dir}" Main

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

if [ $(is_macos) ]; then
    dylib_ext="dylib"
else
    dylib_ext="so"
fi

dylib="lib$(cat "${base_dir}/../Cargo.toml" | grep name | awk 'END{print}' | awk -F '"' '{print $2}').${dylib_ext}"

for java_home in ${java_home_array[@]}; do
    once "${java_home}"
done
