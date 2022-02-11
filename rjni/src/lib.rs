#[macro_use]
mod rbind;
mod platf;

pub use rbind::*;

#[cfg(test)]
mod tests {

    // use crate::platf::*;

    use std::{
        env,
        path::Path,
        process::{Command, Stdio},
    };

    // use toml::Value;

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

                assert!(Command::new(&javac_path)
                    .arg("Main.java")
                    .current_dir(&jcalls_dir)
                    .status()
                    .unwrap()
                    .success());
                assert!(Command::new(&java_path)
                    .arg(format!("-Djava.library.path={dylib_dir}"))
                    .arg("Main")
                    .current_dir(&jcalls_dir)
                    .status()
                    .unwrap()
                    .success());

                println!();
                println!("///////////////////////////////////////////////////////////////");
                println!("///////////////////////////////////////////////////////////////");
                println!("///////////////////////////////////////////////////////////////");
            }
        }

        // let cargo_toml_content = read_to_string(cargo_build_dir.join("Cargo.toml")).unwrap();
        // let value = cargo_toml_content.parse::<Value>().unwrap();
        // let dylib_name = value["lib"]["name"].as_str().unwrap();
        // let dylib_ext = if Platform::IS_MACOS {
        //     "dylib"
        // } else if Platform::IS_LINUX {
        //     "so"
        // } else {
        //     "dll"
        // };
        // let dylib = format!("{dylib_name}.{dylib_ext}");

        println!();
    }
}
