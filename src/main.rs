use clap::{command, value_parser, Arg};
use color_eyre::eyre::{eyre, Result};
use indicatif::{style::ProgressStyle, ParallelProgressIterator, ProgressIterator};
use rayon::prelude::*;

mod show;

fn main() -> Result<()> {
  color_eyre::install()?;

  let matches = command!()
    .arg(
      Arg::new("n")
        .value_parser(value_parser!(usize))
        .index(1)
        .default_value("1000000"),
    )
    .arg(
      Arg::new("doors")
        .short('d')
        .long("doors")
        .default_value("3")
        .value_parser(value_parser!(usize)),
    )
    .get_matches();

  let n = matches
    .get_one::<usize>("n")
    .ok_or_else(|| eyre!("unable to extract 'n' argument from CLI"))
    .map(|val| val.to_owned())?;

  let doors = matches
    .get_one::<usize>("doors")
    .ok_or_else(|| eyre!("unable to extract 'doors' argument from CLI"))
    .map(|val| val.to_owned())?;

  let pbar_style =
    ProgressStyle::with_template("{msg} [{elapsed}] {bar:50.green/white} {pos:>7}/{len:7}")?;

  let shows = (1..n)
    .into_par_iter()
    .progress_with_style(pbar_style.clone())
    .with_message(format!("Generating {n} Shows"))
    .map(|_i| show::Show::random(doors))
    .collect::<Vec<_>>();

  let results = shows.into_iter().map(|show| show.pick());

  let won_car_stayed = results
    .clone()
    .progress_with_style(pbar_style.clone())
    .with_message("Calculating Stay Results")
    .filter(|b| *b)
    .count();

  let won_car_switched = results
    .progress_with_style(pbar_style)
    .with_message("Calculating Switch Results")
    .filter(|b| !*b)
    .count();

  println!("Stayed Result: {won_car_stayed}/{n}");
  println!("Switched Result: {won_car_switched}/{n}");

  Ok(())
}
