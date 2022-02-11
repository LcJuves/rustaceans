#[macro_use]
mod rbind;
mod platf;

pub use rbind::*;

#[cfg(test)]
mod tests {

    use crate::platf::*;

    use std::{
        env,
        fs::{read_to_string, remove_file},
        path::Path,
        process::{Command, Stdio},
    };

    use toml::Value;

    fn jcomp_and_run(jcalls_dir: &Path, javac_path: &Path, java_path: &Path, dylib_dir: &str) {
        assert!(Command::new(javac_path)
            .arg("Main.java")
            .current_dir(jcalls_dir)
            .status()
            .unwrap()
            .success());
        assert!(Command::new(java_path)
            .arg(format!("-Djava.library.path={dylib_dir}"))
            .arg("Main")
            .current_dir(jcalls_dir)
            .status()
            .unwrap()
            .success());
    }

    #[test]
    fn it_works() {
        let profile = env!("PROFILE");
        dbg!(&profile);

        let cargo_manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let cargo_build_dir = cargo_manifest_dir.join("test");

        if profile == "release" {
            assert!(Command::new("cargo")
                .arg("build")
                .arg(format!("--{profile}"))
                .current_dir(&cargo_build_dir)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .unwrap()
                .success());
        } else {
            assert!(Command::new("cargo")
                .arg("build")
                .current_dir(&cargo_build_dir)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .unwrap()
                .success());
        }

        let out_dir = env!("OUT_DIR");
        let dylib_dir = &out_dir[..(out_dir.rfind(profile).unwrap() + profile.len())];
        dbg!(&dylib_dir);

        let cargo_toml_content = read_to_string(cargo_build_dir.join("Cargo.toml")).unwrap();
        let value = cargo_toml_content.parse::<Value>().unwrap();
        let dylib_name = value["lib"]["name"].as_str().unwrap();
        dbg!(&dylib_name);
        let dylib_ext = if Platform::IS_MACOS {
            "dylib"
        } else if Platform::IS_LINUX {
            "so"
        } else {
            "dll"
        };
        dbg!(&dylib_ext);

        #[cfg(windows)]
        let dylib = format!("{dylib_name}.{dylib_ext}");
        #[cfg(any(unix, target_os = "hermit"))]
        let dylib = format!("lib{dylib_name}.{dylib_ext}");

        let dylib_full_path = Path::new(dylib_dir).join(dylib);

        let clang_dir = cargo_build_dir.join("tests").join("clang");

        let jcalls_dir = cargo_build_dir.join("tests").join("jcalls");
        let env_java_home = env::var("JAVA_HOME").unwrap();
        let java_home_paths = vec![Path::new(&env_java_home)];
        for java_home in java_home_paths {
            if java_home.exists() {
                let javac_path = java_home.join("bin").join("javac");
                let java_path = java_home.join("bin").join("java");

                println!();
                assert!(Command::new(&java_path).arg("-version").status().unwrap().success());
                println!("============================================================");
                println!("============================================================");
                println!();

                jcomp_and_run(&jcalls_dir, &javac_path, &java_path, &dylib_dir);

                println!();
                println!("###############################################################");
                println!();

                remove_file(&dylib_full_path).unwrap_or(());
                remove_file(&clang_dir.join("CallJNI.h")).unwrap_or(());

                // "${javac_path}" -encoding utf8 -h "${base_dir}" CallJNI.java
                assert!(Command::new(&javac_path)
                    .arg("-encoding")
                    .arg("utf8")
                    .arg("-h")
                    .arg(&clang_dir)
                    .arg("CallJNI.java")
                    .current_dir(&jcalls_dir)
                    .status()
                    .unwrap()
                    .success());

                if Platform::IS_MACOS {
                    // gcc -dynamiclib -I "${base_dir}" "${base_dir}/main.c" -o "${base_dir}/${dylib}"
                    assert!(Command::new("gcc")
                        .arg("-dynamiclib")
                        .arg("-I")
                        .arg(&clang_dir)
                        .arg(&clang_dir.join("main.c"))
                        .arg("-o")
                        .arg(&dylib_full_path)
                        .status()
                        .unwrap()
                        .success());
                } else {
                    // gcc -I "${base_dir}" -fPIC -shared -o "${base_dir}/${dylib}" "${base_dir}/main.c"
                    assert!(Command::new("gcc")
                        .arg("-I")
                        .arg(&clang_dir)
                        .arg("-fPIC")
                        .arg("-shared")
                        .arg("-o")
                        .arg(&dylib_full_path)
                        .arg(&clang_dir.join("main.c"))
                        .status()
                        .unwrap()
                        .success());
                }

                jcomp_and_run(&jcalls_dir, &javac_path, &java_path, &dylib_dir);

                println!();
                println!("///////////////////////////////////////////////////////////////");
                println!("///////////////////////////////////////////////////////////////");
                println!("///////////////////////////////////////////////////////////////");
            }
        }

        println!();
    }
}
