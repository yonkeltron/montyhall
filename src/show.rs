use rand::prelude::*;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Show {
  pub doors: Vec<bool>,
}

impl Show {
  pub fn random(n: usize) -> Self {
    let idx = thread_rng().gen_range(1..n);

    let doors = (1..n).map(|i| i == idx).collect();

    Self { doors }
  }

  pub fn pick(self) -> bool {
    let mut rng = thread_rng();
    let door = self.doors.iter().choose(&mut rng);

    match door {
      Some(val) => *val,
      None => unreachable!("Must choose a value"),
    }
  }
}
