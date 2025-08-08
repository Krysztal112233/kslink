use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::Ok;
use clap::Parser;
use im::HashMap;
use inquire::{Confirm, Select, Text};
use kslink_rules::{
    cli::{self, CheckCmd, FmtCmd},
    meta::{RuleContent, RuleMeta, RuleType},
    RuleSet,
};
use log::{info, warn};
use rayon::prelude::*;
use tracing::level_filters::LevelFilter;
use walkdir::WalkDir;

const BASE_PATH: &str = "./rules";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse().command;
    match cli {
        cli::Commands::Create => create().await,
        cli::Commands::Fmt(cmd) => fmt(cmd).await,
        cli::Commands::Check(cmd) => check(cmd).await,
    }
}

async fn create() -> anyhow::Result<()> {
    let append = Select::new("Append or create rule?", vec!["Create", "Append"]).prompt()?;

    if append == "Append" {
        let rules = WalkDir::new(BASE_PATH)
            .into_iter()
            .par_bridge()
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .filter(|f| {
                File::open(f.path())
                    .map(|mut file| {
                        let mut buf = String::with_capacity(1024);
                        let _ = file.read_to_string(&mut buf);
                        toml::from_str::<RuleMeta>(&buf).is_ok()
                    })
                    .unwrap_or_default()
            })
            .map(|f| f.path().display().to_string())
            .collect::<Vec<_>>();
        let selected = Select::new("Select rules:", rules).prompt()?;
        let meta = append_rule(&selected).await?;

        let mut file = File::create(&selected)?;
        file.write_all(toml::to_string_pretty(&meta)?.as_bytes())?;
    } else {
        let name = Text::new("What's your rule name?").prompt()?;
        let meta = new_rule().await?;

        let mut file = File::create(PathBuf::new().join(BASE_PATH).join(format!("{name}.toml")))?;
        file.write_all(toml::to_string_pretty(&meta)?.as_bytes())?;
    };

    Ok(())
}

async fn new_rule() -> anyhow::Result<RuleMeta> {
    let reg = Text::new("What's your matching regex?").prompt()?;
    let content = RuleContent {
        explain: Text::new("Any description of your param rule?").prompt_skippable()?,
        param: {
            let mut map = HashMap::default();

            loop {
                let param = Text::new("What's param do you want to remove?").prompt()?;
                let desc = Text::new(&format!("Any description for param `{param}`?"))
                    .prompt_skippable()?;

                map.insert(param, desc);

                if !Confirm::new("Do you have more param want to add?")
                    .with_default(true)
                    .prompt()?
                {
                    break;
                }
            }
            map
        },
    };

    let rule = RuleMeta {
        r#type: RuleType::Trim,
        content: HashMap::new().update(reg, content),
        ..Default::default()
    };

    Ok(rule)
}

async fn append_rule<P>(path: P) -> anyhow::Result<RuleMeta>
where
    P: AsRef<Path>,
{
    let mut content = String::with_capacity(1024);
    File::open(path)?.read_to_string(&mut content)?;
    let meta = toml::from_str::<RuleMeta>(&content)?.merge(new_rule().await?);

    Ok(meta)
}

#[allow(unused)]
async fn fmt(cmd: FmtCmd) -> anyhow::Result<()> {
    Ok(())
}

async fn check(cmd: CheckCmd) -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    let rules = WalkDir::new(cmd.directory.unwrap_or("./rules/".to_owned()))
        .into_iter()
        .par_bridge()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .inspect(|f| info!("Walked rule file: {}", f.path().display()))
        .map(|file| File::open(file.path()))
        .filter(|it| {
            it.as_ref()
                .inspect_err(|err| warn!("Failed to open it: {err}"))
                .is_ok()
        })
        .map(Result::unwrap)
        .map(|mut file| {
            let mut buf = String::with_capacity(1024);
            let _ = file.read_to_string(&mut buf);
            toml::from_str::<RuleMeta>(&buf).inspect_err(|err| warn!("Failed to parse: {err}"))
        })
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    let rules = rules
        .clone()
        .into_iter()
        .inspect(|e| {
            e.content
                .iter()
                .for_each(|(e, _)| info!("Compiled rule `{e}`"))
        })
        .map(|e| (e.clone(), RuleSet::from(e)))
        .collect::<Vec<_>>();

    let mut succeed = 0;
    let count = rules
        .iter()
        .filter_map(|(it, _)| it.tests.clone())
        .flat_map(|e| e.into_iter())
        .count();
    for (meta, ruleset) in rules {
        for (input, except) in meta.tests.iter().flatten() {
            let Some(output) = ruleset.prune(input).await else {
                continue;
            };

            if output == *except {
                succeed += 1;
                info!("[PASS]: {except} == {output}")
            } else {
                warn!("[FAIL]: {except} == {output}")
            }
        }
    }

    info!("Test result: [{succeed}/{count}]");

    Ok(())
}
