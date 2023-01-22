use std::collections::HashMap;

#[derive(Default)]
struct Results {
    won: i8,
    loss: i8,
    drawn: i8,
}

impl Results {
    fn score(&self) -> i8 {
        self.won * 3 + self.drawn
    }

    fn to_string(&self, team: &str) -> String {
        format!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            team,
            self.won + self.drawn + self.loss,
            self.won,
            self.drawn,
            self.loss,
            self.score()
        )
    }
}

pub fn tally(match_results: &str) -> String {
    let mut teams: Vec<(&str, Results)> = match_results
        .lines()
        .into_iter()
        .fold(HashMap::<&str, Results>::new(), |mut team, line| {
            let frag: Vec<&str> = line.split(';').collect();
            let (left, right, result) = (frag[0], frag[1], frag[2]);

            match result {
                "win" => {
                    team.entry(left).or_default().won += 1;
                    team.entry(right).or_default().loss += 1;
                }
                "loss" => {
                    team.entry(left).or_default().loss += 1;
                    team.entry(right).or_default().won += 1;
                }
                "draw" => {
                    team.entry(left).or_default().drawn += 1;
                    team.entry(right).or_default().drawn += 1;
                }
                _ => unreachable!(),
            }
            team
        })
        .into_iter()
        .collect();

    teams.sort_by_key(|team| (-team.1.score(), team.0));

    let header = format!(
        "{: <30} | {: >2} | {: >2} | {: >2} | {: >2} | {: >2}",
        "Team", "MP", "W", "D", "L", "P"
    );

    vec![header]
        .into_iter()
        .chain(teams.iter().map(|row| row.1.to_string(row.0)))
        .collect::<Vec<String>>()
        .join("\n")
}
