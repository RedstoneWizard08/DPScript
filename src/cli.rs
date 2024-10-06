use clap::{Parser, Subcommand};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use ron::ser::PrettyConfig;
use std::{fs, path::PathBuf};
use walkdir::WalkDir;

use crate::{dump_ast_part, Lexer, PackToml, Result, Tokenizer, Validator, AST};

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    /// Build a project
    Build {
        #[arg(short, long = "config", default_value_os_t = PathBuf::from("./pack.toml"))]
        config_path: PathBuf,

        #[arg(short, long = "output", default_value_os_t = PathBuf::from("./out"))]
        out_dir: PathBuf,

        #[arg(short = 'A', long)]
        dump_ast: bool,

        #[arg(short = 'T', long)]
        dump_tokens: bool,
    },

    /// Compile a single file
    Compile {
        file: PathBuf,

        #[arg(short, long = "output", default_value_os_t = PathBuf::from("."))]
        out_dir: PathBuf,

        #[arg(short = 'A', long)]
        dump_ast: bool,

        #[arg(short = 'T', long)]
        dump_tokens: bool,
    },
}

impl Cli {
    pub fn start() -> Result<()> {
        Self::parse().run()
    }

    pub fn run(&self) -> Result<()> {
        self.command.run()
    }
}

impl Commands {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Build {
                config_path,
                dump_ast,
                dump_tokens,
                out_dir,
            } => {
                let root = config_path.canonicalize()?.parent().unwrap().join("src");
                let config = fs::read_to_string(config_path)?;
                let _config = toml::from_str::<PackToml>(&config)?;

                debug!("Source Root: {:?}", root);

                // TODO: Something with the config

                let walk = WalkDir::new(&root)
                    .into_iter()
                    .filter_map(|v| v.ok())
                    .filter(|v| v.file_name().to_str().unwrap().ends_with(".dps"))
                    .collect::<Vec<_>>();

                let bar = ProgressBar::new(walk.len() as u64).with_style(
                    ProgressStyle::with_template("[{bar:40.cyan/blue}] {pos:.blue} of {len:.blue}")
                        .unwrap()
                        .progress_chars("## "),
                );

                let mut asts = Vec::new();

                for entry in walk {
                    let path = entry.path();
                    let path_str = path.strip_prefix(&root).unwrap().to_str().unwrap();

                    bar.println(format!(
                        "{} {}",
                        style("[parse]").red(),
                        style(path_str).magenta()
                    ));

                    asts.push(Self::create_ast(
                        &PathBuf::from(path),
                        out_dir,
                        *dump_tokens,
                        *dump_ast,
                    )?);

                    bar.inc(1);
                }

                bar.finish();

                if asts.is_empty() {
                    warn!("No source files found!");

                    return Ok(());
                }

                let mut ast = asts.remove(0);

                for item in asts {
                    ast.merge(item);
                }

                ast.index_modules()?;

                if *dump_ast {
                    let dump_file = out_dir.join("ast_merged.ron");

                    fs::write(
                        dump_file,
                        ron::ser::to_string_pretty(&ast, PrettyConfig::new())?,
                    )?;

                    let merged_dir = out_dir.join("merged");

                    if !merged_dir.exists() {
                        fs::create_dir_all(&merged_dir)?;
                    }

                    dump_ast_part!(ast.top_level => merged_dir);
                    dump_ast_part!(ast.imports => merged_dir);
                    dump_ast_part!(ast.funcs => merged_dir);
                    dump_ast_part!(ast.vars => merged_dir);
                    dump_ast_part!(ast.blocks => merged_dir);
                    dump_ast_part!(ast.enums => merged_dir);
                    dump_ast_part!(ast.objectives => merged_dir);
                    dump_ast_part!(ast.modules => merged_dir);
                }

                Validator::new(ast).validate()?;
            }

            Self::Compile {
                file,
                dump_ast,
                dump_tokens,
                out_dir,
            } => {
                Self::compile(file, out_dir, *dump_tokens, *dump_ast)?;
            }
        }

        Ok(())
    }

    fn compile_ast(_file: &PathBuf, _out_dir: &PathBuf, _ast: &AST) -> Result<()> {
        // TODO: Implement

        Ok(())
    }

    fn compile(file: &PathBuf, out_dir: &PathBuf, dump_tokens: bool, dump_ast: bool) -> Result<()> {
        Self::compile_ast(
            file,
            out_dir,
            &Self::create_ast(file, out_dir, dump_tokens, dump_ast)?,
        )?;

        Ok(())
    }

    fn create_ast(
        file: &PathBuf,
        out_dir: &PathBuf,
        dump_tokens: bool,
        dump_ast: bool,
    ) -> Result<AST> {
        let file_name = file.to_str().unwrap();
        let data = fs::read_to_string(&file)?;
        let tokens = Tokenizer::new(&file_name, data.clone())
            .tokenize()?
            .tokens();

        if !out_dir.exists() {
            fs::create_dir_all(out_dir)?;
        }

        if dump_tokens {
            let tokens_dir = out_dir.join("tokens");

            if !tokens_dir.exists() {
                fs::create_dir_all(&tokens_dir)?;
            }

            let dump_file =
                tokens_dir.join(file.with_extension("dps.tokens.ron").file_name().unwrap());

            fs::write(
                dump_file,
                ron::ser::to_string_pretty(&tokens, PrettyConfig::new())?,
            )?;
        }

        let ast = Lexer::new(&file_name, data, tokens).run()?.ast();

        if dump_ast {
            let ast_dir = out_dir.join("ast");

            if !ast_dir.exists() {
                fs::create_dir_all(&ast_dir)?;
            }

            let dump_file = ast_dir.join(file.with_extension("dps.ast.ron").file_name().unwrap());

            fs::write(
                dump_file,
                ron::ser::to_string_pretty(&ast, PrettyConfig::new())?,
            )?;
        }

        Ok(ast)
    }
}
