use std::ffi::OsStr;
use std::path::Path;
use std::{env, fs, io, path::PathBuf};
use std::env::VarError;

fn add_source<P: AsRef<Path>>(cfg: &mut cc::Build, path: P) -> Result<(), io::Error> {
    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_dir() {
            add_source(cfg, path)?;
        } else if path.extension() == Some(OsStr::new("cpp")) {
            cfg.file(path);
        }
    }
    Ok(())
}

static ASMJIT_SOURCE_PATH: &str = "./asmjit/src";
static BLEND2D_SOURCE_PATH: &str = "./blend2d/src";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // bindings
    let whitelist_regex = "[Bb][Ll].*";
    let bindings = bindgen::Builder::default()
        .header("blend2d/src/blend2d.h")
        .layout_tests(false)
        .generate_comments(false)
        .default_enum_style(bindgen::EnumVariation::NewType{is_bitfield: true, is_global: false})
        .allowlist_function(whitelist_regex)
        .allowlist_type(whitelist_regex)
        .allowlist_var(whitelist_regex)
        .derive_debug(false)
        .clang_arg("-DASMJIT_STATIC")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // lib
    let mut cfg = Build::new(cc::Build::new());
    add_source(&mut cfg, BLEND2D_SOURCE_PATH).unwrap();
    add_source(&mut cfg, ASMJIT_SOURCE_PATH).unwrap();
    cfg.cpp(true)
        .warnings(false)
        .extra_warnings(false)
        .flag_if_supported("-static")
        .flag_if_supported("-std=c++17")
        .flag_if_supported("/std:c++latest")
        .include(ASMJIT_SOURCE_PATH)
        .include(BLEND2D_SOURCE_PATH)
        .define("ASMJIT_STATIC", None);

    let arch = get_env("CARGO_CFG_TARGET_ARCH").unwrap();


    if get_env("PROFILE").is_some_and(|s| s != "debug") {
        cfg.define("NDEBUG", None);
    }


    match arch.as_str() {
        "x86_64" | "i686" => {
            // SSE2
            cfg.define("BL_BUILD_OPT_SSE2", None);
            cfg.mflag("-msse2", "/arch:SSE2");

            // SSE3
            cfg.define("BL_BUILD_OPT_SSE3", None);
            cfg.mflag("-msse3", None);
            if cfg.is_msvc {
                cfg.define("__SSE3__", None);
            }

            // SSSE3
            cfg.define("BL_BUILD_OPT_SSSE3", None);
            cfg.mflag("-mssse3", None);
            if cfg.is_msvc {
                cfg.define("__SSSE3__", None);
            }

            // SSE4.1
            cfg.define("BL_BUILD_OPT_SSE4_1", None);
            cfg.mflag("-msse4.1", None);
            if cfg.is_msvc {
                cfg.define("__SSE4_1__", None);
            }

            // SSE4.2
            cfg.define("BL_BUILD_OPT_SSE4_2", None);
            cfg.mflags(["-mpopcnt", "-mpclmul", "-msse4.2"], "/arch:SSE4.2");

            // AVX
            cfg.define("BL_BUILD_OPT_AVX", None);
            cfg.mflags(["-mpopcnt", "-mpclmul", "-mavx"], "/arch:AVX");

            // AVX2
            cfg.define("BL_BUILD_OPT_AVX2", None);
            cfg.mflags(["-mpopcnt", "-mpclmul", "-mbmi", "-mbmi2", "-mavx2"], "/arch:AVX2");

            // AVX512 (Disabled for now, breaks runtime detection)
            // cfg.define("BL_BUILD_OPT_AVX512", None);
            // cfg.mflags(["-mpopcnt", "-mpclmul", "-mbmi", "-mbmi2", "-mavx512f", "-mavx512bw", "-mavx512dq", "-mavx512cd", "-mavx512vl"], "/arch:AVX512");
        }
        "aarch64" | "arm" => {
            // TODO: transfer flags from CMakeLists.txt
            let is_aarch64 = arch == "aarch64";

            if cfg.is_msvc {
                cfg.define("__ARM_NEON__", None);
            }
            cfg.mflag(
                if is_aarch64 {
                    "-march=armv8-a+crc+simd"
                } else {
                    "-mfpu=neon"
                },
                None,
            );
        }
        _ => {}
    }

    if cfg.is_msvc {
        cfg.flag("-MP")
            .flag("-GR-")
            .flag("-GF")
            .flag("-Zc:__cplusplus")
            .flag("-Zc:inline")
            .flag("-Zc:strictStrings")
            .flag("-Zc:threadSafeInit-");
    } else {
        cfg.flag("-fvisibility=hidden")
            .flag("-fno-exceptions")
            .flag("-fno-rtti")
            .flag("-fno-math-errno")
            .flag("-fno-semantic-interposition")
            .flag("-fno-threadsafe-statics")
            .flag("-fmerge-all-constants")
            .flag("-ftree-vectorize");
    }

    cfg.compile("blend2d");
    match get_env("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "windows" => {
            println!("cargo:rustc-link-lib=user32");
            println!("cargo:rustc-link-lib=uuid");
            println!("cargo:rustc-link-lib=shell32");
        },
        "linux" => {
            println!("cargo:rustc-link-lib=c");
            println!("cargo:rustc-link-lib=m");
            println!("cargo:rustc-link-lib=pthread");
            println!("cargo:rustc-link-lib=rt");
        },
        "macos" => {
            println!("cargo:rustc-link-lib=c");
            println!("cargo:rustc-link-lib=m");
            println!("cargo:rustc-link-lib=pthread");
        },
        _ => (),
    }
}

fn get_env(name: &str) -> Option<String> {
    println!("cargo:rerun-if-env-changed={name}");
    match env::var(name) {
        Ok(s) => Some(s),
        Err(VarError::NotPresent) => None,
        Err(VarError::NotUnicode(s)) => {
            panic!("unrecognize env var of {name}: {:?}", s.to_string_lossy());
        }
    }
}

// Based on rust-lang/libz-sys/zng/cc.rs
// Apache 2.0 or MIT Alex Crichton, Josh Triplett, Sebastian Thiel
struct Build {
    cfg: cc::Build,
    is_msvc: bool,
}

impl Build {
    fn new(cfg: cc::Build) -> Self {
        let is_msvc = cfg.try_get_compiler().unwrap().is_like_msvc();
        Self { cfg, is_msvc }
    }

    fn mflag(
        &mut self,
        non_msvc: impl Into<Option<&'static str>>,
        msvc: impl Into<Option<&'static str>>,
    ) {
        let Some(flag) = (if self.is_msvc {
            msvc.into()
        } else {
            non_msvc.into()
        }) else {
            return;
        };
        self.cfg.flag(flag);
    }

    pub fn mflags<Iter>(&mut self, non_msvc: Iter, msvc: impl Into<Option<&'static str>>)
    where
        Iter: IntoIterator,
        Iter::Item: AsRef<OsStr>,
    {
        if self.is_msvc {
            if let Some(flag) = msvc.into() {
                self.cfg.flag(flag);
            }
        } else {
            for flag in non_msvc {
                self.cfg.flag(flag);
            }
        }
    }
}

impl std::ops::Deref for Build {
    type Target = cc::Build;

    fn deref(&self) -> &Self::Target {
        &self.cfg
    }
}

impl std::ops::DerefMut for Build {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cfg
    }
}