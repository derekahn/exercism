use std::collections::HashMap;

#[derive(Debug)]
struct Row {
    team: String,
    played: u8,
    wins: u8,
    draws: u8,
    loss: u8,
    points: u8,
}

impl Row {
    fn new(team: String) -> Self {
        Self {
            team,
            played: 0,
            wins: 0,
            draws: 0,
            loss: 0,
            points: 0,
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let header = format!(
        "{0: <30} | {1: >2} | {2: >2} | {3: >2} | {4: >2} | {5: >2}",
        "Team", "MP", "W", "D", "L", "P"
    );

    if match_results.is_empty() {
        return header;
    }

    let teams: HashMap<&str, Row> = match_results
        .split(|c: char| c == ';' || c == '\n')
        .filter(|&w| {
            !w.is_empty() && !w.contains("win") && !w.contains("loss") && !w.contains("draw")
        })
        .into_iter()
        .fold(HashMap::new(), |mut acc, team| {
            acc.insert(team, Row::new(team.into()));
            acc
        });

    let results = match_results
        .split('\n')
        .into_iter()
        .fold(teams, |mut acc, line| {
            let game: Vec<&str> = line.split(';').map(|word| word).collect();
            let (team1, team2, result) = (game[0], game[1], game[2]);

            match result {
                "win" => {
                    let team1 = acc
                        .get_mut(team1)
                        .expect("[win] Row should exist for team1");
                    team1.played += 1;
                    team1.wins += 1;
                    team1.points += 3;

                    let team2 = acc
                        .get_mut(team2)
                        .expect("[win] Row should exist for team2");
                    team2.played += 1;
                    team2.loss += 1;
                }
                "loss" => {
                    let team1 = acc
                        .get_mut(team1)
                        .expect("[loss] Row should exist for team1");
                    team1.played += 1;
                    team1.loss += 1;

                    let team2 = acc
                        .get_mut(team2)
                        .expect("[loss] Row should exist for team2");
                    team2.played += 1;
                    team2.wins += 1;
                    team2.points += 3;
                }
                "draw" => {
                    let team1 = acc
                        .get_mut(team1)
                        .expect("[draw] Row should exist for team1");
                    team1.played += 1;
                    team1.draws += 1;
                    team1.points += 1;

                    let team2 = acc
                        .get_mut(team2)
                        .expect("[draw] Row should exist for team2");
                    team2.played += 1;
                    team2.draws += 1;
                    team2.points += 1;
                }
                _ => panic!("Should be only a win, loss, or draw!"),
            }
            acc
        });

    let mut order: Vec<(u8, &str)> = results.iter().fold(Vec::new(), |mut acc, (&team, row)| {
        acc.push((row.points, team));
        acc
    });
    order.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(b.1)));
    order.into_iter().fold(header, |acc, (_, team)| {
        let row = results.get(team).expect("team_row must exist");
        let line = format!(
            "{0: <30} | {1: >2} | {2: >2} | {3: >2} | {4: >2} | {5: >2}",
            row.team, row.played, row.wins, row.draws, row.loss, row.points,
        );

        format!("{acc}\n{line}")
    })
}
