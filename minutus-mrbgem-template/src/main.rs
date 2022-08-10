use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use maplit::hashmap;
use minutus::build_simple_evaluator;
use minutus::mruby::*;
use minutus::types::*;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::path::Path;

minutus::define_funcall!(
    fn new(&self, params: HashMap<RSymbol, minu_value>) -> MinuValue;
    fn create(&self);
);

lazy_static::lazy_static! {
    pub static ref TEMPLATES: tera::Tera = {
        let mut tera = tera::Tera::default();
        tera.add_raw_templates(
            vec![
                ("lib.rs", include_str!("templates/lib.rs")),
                ("mrbgem.rake", include_str!("templates/mrbgem.rake")),
                ("test.rb", include_str!("templates/test.rb")),
            ]
        ).expect("failed to load template");
        tera
    };
}

#[argopt::cmd]
fn main(
    /// Set LISENCE
    #[opt(short, long, default_value = "MIT")]
    license: String,

    /// Set user name on github
    #[opt(short, long)]
    github_user: Option<String>,

    /// Set class name
    #[opt(short, long)]
    class_name: Option<String>,

    /// Set the author of this mgem
    #[opt(short, long)]
    author: Option<String>,

    /// Set target mruby version
    #[opt(short, long, default_value = "3.1.0")]
    mruby_version: String,

    /// Set and generate binary tools
    #[opt(short = 'B', long)]
    bin_name: Option<String>,

    mrbgem_name: String,
) -> Result<()> {
    initialize_by_mrbgem_template(
        license,
        github_user,
        class_name.clone(),
        author,
        mruby_version.clone(),
        bin_name,
        mrbgem_name.clone(),
    )?;

    let class_name = class_name.unwrap_or(parse_class_name(mrbgem_name.clone()));

    let mrbgem_dir = current_dir()?.join(&mrbgem_name);
    run_command(&mrbgem_dir, &["cargo", "init", "--lib", "."])?;

    let mut cargo_toml = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(mrbgem_dir.join("Cargo.toml"))?;
    writeln!(
        cargo_toml,
        "
# workaround ignoring minutus's root workspace
# see: https://github.com/rust-lang/cargo/issues/7895        
[workspace]

[lib]
crate-type = [\"staticlib\"]"
    )?;

    let feature_mruby = format!("mruby_{}", mruby_version.replace(".", "_"));
    if std::env::var("MRBGEM_TEMPLATE_DEBUG").is_ok() {
        run_command(
            &mrbgem_dir,
            &[
                "cargo",
                "add",
                "minutus",
                "--path",
                "../../../minutus",
                "--features",
                &feature_mruby,
            ],
        )?;
    } else {
        run_command(
            &mrbgem_dir,
            &["cargo", "add", "minutus", "--features", &feature_mruby],
        )?;
    }

    run_command(&mrbgem_dir, &["sh", "-c", "rm src/*.h src/*.c"])?;
    run_command(
        &mrbgem_dir,
        &[
            "sh",
            "-c",
            "echo '// In order to make mruby to load mrbgem (workaround)' > src/dummy.c",
        ],
    )?;

    let mrbgem_rake = {
        let mut context = tera::Context::new();
        context.insert("mrbgem_name", &mrbgem_name);
        context.insert("library_name", &mrbgem_name.replace("-", "_"));
        TEMPLATES.render("mrbgem.rake", &context)?
    };
    std::fs::write(mrbgem_dir.join("mrbgem.rake"), mrbgem_rake)?;

    let src_lib = {
        let mut context = tera::Context::new();
        context.insert("class_name", &class_name);
        context.insert("underscored_package_name", &mrbgem_name.replace("-", "_"));
        TEMPLATES.render("lib.rs", &context)?
    };
    std::fs::write(mrbgem_dir.join("src/lib.rs"), src_lib)?;

    run_command(&mrbgem_dir, &["sh", "-c", "rm test/*.rb"])?;
    let test_rb = {
        let mut context = tera::Context::new();
        context.insert("class_name", &class_name);
        TEMPLATES.render("test.rb", &context)?
    };
    std::fs::write(
        mrbgem_dir.join(format!("test/mrb_{}.rb", class_name.to_case(Case::Snake))),
        test_rb,
    )?;

    Ok(())
}

fn initialize_by_mrbgem_template(
    license: String,
    github_user: Option<String>,
    class_name: Option<String>,
    author: Option<String>,
    mruby_version: String,
    bin_name: Option<String>,
    mrbgem_name: String,
) -> Result<()> {
    let runtime = build_simple_evaluator();

    // for convenience
    let mrb = runtime.mrb();

    // Supress logs
    runtime
        .evaluate("def puts(_); end")
        .map_err(|e| anyhow!(e))?;

    let mrbgem_template = runtime.evaluate("MrbgemTemplate").map_err(|e| anyhow!(e))?;
    let class_name = class_name.unwrap_or_else(|| parse_class_name(mrbgem_name.clone()));
    let c = mrbgem_template.new(hashmap! {
        "license".to_sym(mrb) => license.into_mrb(mrb),
        "github_user".to_sym(mrb) => github_user.unwrap_or_else(detect_github_user).into_mrb(mrb),
        "mrbgem_name".to_sym(mrb) => mrbgem_name.clone().into_mrb(mrb),
        "mrbgem_prefix".to_sym(mrb) => ".".into_mrb(mrb),
        "class_name".to_sym(mrb) => class_name.into_mrb(mrb),
        "author".to_sym(mrb) => author.unwrap_or_else(detect_author).into_mrb(mrb),
        "mruby_version".to_sym(mrb) => mruby_version.into_mrb(mrb),
        "ci".to_sym(mrb) => unsafe { minu_true_value() },
        "local_builder".to_sym(mrb) => unsafe { minu_true_value() },
        "bin_name".to_sym(mrb) => match bin_name {
            Some(v) => v.into_mrb(mrb),
            None => unsafe  { minu_nil_value() },
        }
    });
    c.create();
    Ok(())
}

fn parse_class_name(mrbgem_name: String) -> String {
    mrbgem_name
        .strip_prefix("mruby-")
        .unwrap()
        .to_case(Case::UpperCamel)
}

fn detect_github_user() -> String {
    let github_user = github_config("github.user");

    if let Some(val) = github_user {
        val
    } else {
        eprintln!("github-user is not specified.");
        std::process::exit(1)
    }
}

fn detect_author() -> String {
    let github_user = github_config("user.name");

    if let Some(val) = github_user {
        val
    } else {
        eprintln!("author is not specified.");
        std::process::exit(1)
    }
}

fn github_config(conf: &str) -> Option<String> {
    let val = String::from_utf8(
        std::process::Command::new("git")
            .args(["config", conf])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .trim()
    .to_string();

    if val.is_empty() {
        None
    } else {
        Some(val)
    }
}

fn run_command(current_dir: &Path, cmd: &[&str]) -> Result<String> {
    let output = std::process::Command::new(cmd[0])
        .args(&cmd[1..])
        .current_dir(current_dir)
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(anyhow!(format!(
            "Executing {:?} failed\n{}",
            cmd,
            String::from_utf8_lossy(&output.stderr).to_string()
        )))
    }
}
