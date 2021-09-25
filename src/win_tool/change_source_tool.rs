const TUNA_SOURCE_RUST: &str = r#"[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'tuna'
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
"#;

const SJTU_SOURCE_RUST: &str = r#"[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'sjtu'
[source.sjtu]
registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"
"#;

const USTC_SOURCE_RUST: &str = r#"[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
"#;

const RUSTCC_SOURCE_RUST: &str = r#"[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'rustcc'
[source.rustcc]
registry = "git://crates.rustcc.cn/crates.io-index"
"#;

const NODE_SOURCE: &str = "https://registry.npm.taobao.org/";

const TUNA_SOURCE_PYTHON: &str = r#"[global]
index-url = https://pypi.mirrors.ustc.edu.cn/simple/
[install]
trusted-host = mirrors.ustc.edu.cn
"#;

const TUNA_SOURCE_CONDA: &str = r#"channels:
- https://mirrors.tuna.tsinghua.edu.cn/anaconda/pkgs/main/
- https://mirrors.tuna.tsinghua.edu.cn/anaconda/pkgs/free/
- https://mirrors.tuna.tsinghua.edu.cn/anaconda/cloud/conda-forge/
- https://mirrors.tuna.tsinghua.edu.cn/anaconda/cloud/pytorch/
ssl_verify: true
"#;

const USTC_SOURCE_CONDA: &str = r#"channels:
- https://mirrors.ustc.edu.cn/anaconda/pkgs/main/
- https://mirrors.ustc.edu.cn/anaconda/pkgs/free/
- https://mirrors.ustc.edu.cn/anaconda/cloud/conda-forge/
ssl_verify: true
"#;

const SJTUG_SOURCE_CONDA: &str = r#"channels:
- https://mirrors.sjtug.sjtu.edu.cn/anaconda/pkgs/main/
- https://mirrors.sjtug.sjtu.edu.cn/anaconda/pkgs/free/
- https://mirrors.sjtug.sjtu.edu.cn/anaconda/cloud/conda-forge/
ssl_verify: true
"#;

const USTC_SOURCE_CONDA: &str = r#""#;
const USTC_SOURCE_CONDA: &str = r#""#;
const USTC_SOURCE_CONDA: &str = r#""#;
const USTC_SOURCE_CONDA: &str = r#""#;
const USTC_SOURCE_CONDA: &str = r#""#;
const USTC_SOURCE_CONDA: &str = r#""#;

const MAVEN_SOURCE: &str = r#"<mirrors>
<mirror>  
    <id>alimaven</id>  
    <name>aliyun maven</name>  
    <url>http://maven.aliyun.com/nexus/content/groups/public/</url>  
    <mirrorOf>central</mirrorOf>          
</mirror>  
</mirrors>
"#;

pub mod change_source_tool {
    fn change_rust_crate_source() {}
    fn change_npm_source() {}
    fn change_yarn_source() {}
    fn change_python_source() {}
    fn change_conda_source() {}
    fn change_maven_source() {}
}
