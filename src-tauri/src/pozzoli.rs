use std::{time::Duration, usize};

use rand::distr::Iter;
use serde_json::map;

use crate::mei::*;

struct Proposition {
    measures: Vec<Measure>,
}
impl Proposition {
    fn into_iter(self) -> Vec<Measure> {
        let count = self.measures.len();
        let x = self
            .measures
            .into_iter()
            .enumerate()
            .map(|(i, m)| {
                if i == 0 {
                    Measure {
                        left_bar: Some(Bar::Double),
                        right_bar: Some(Bar::Single),
                        ..m
                    }
                } else if i == count - 1 {
                    Measure {
                        left_bar: Some(Bar::Single),
                        right_bar: Some(Bar::Double),
                        ..m
                    }
                } else {
                    m
                }
            })
            .collect();
        return x;
    }
}
struct Exercise {
    name: String,
    propositions: Vec<Proposition>,
}

impl Exercise {
    fn into_mei(self) -> Mei {
        let mut counter = 0;
        let measures = self
            .propositions
            .into_iter()
            .flat_map(|p| p.into_iter())
            .map(|mut m: Measure| {
                counter += 1;
                m.n = counter;
                m
            });

        Mei {
            header: Some(Header {
                file_descriptor: FileDescriptor {
                    title_statement: TitleStatement {
                        titles: vec![Title {
                            type_: "main".to_owned(),
                            value: self.name,
                        }],
                    },
                },
            }),
            music: Some(Music {
                body: Body {
                    mdiv: Mdiv {
                        score: Score {
                            score_definition: ScoreDefinition {
                                measure_numbers: Some(true),
                                staff_group: StaffGroup {
                                    staff_definition: StaffDefinition {
                                        n: 1,
                                        lines: 5,
                                        lines_visible: false,
                                        meter_count: 2,
                                        meter_unit: 4,
                                    },
                                },
                            },
                            section: Section {
                                id: "section1".to_owned(),
                                measures: measures.collect(),
                            },
                        },
                    },
                },
            }),
        }
    }
}

fn notes_from_durations(durations: Vec<i8>) -> Vec<NoteOrRest> {
    return durations
        .iter()
        .map(|&d| {
            if d < 0 {
                NoteOrRest::Rest(Rest {
                    duration: (-d) as u8,
                    id: random_id(),
                })
            } else {
                NoteOrRest::Note(Note {
                    id: random_id(),
                    duration: d as u8,
                    pitch: PitchName::E,
                    octave: 5,
                })
            }
        })
        .collect();
}
fn measure_from_durations(durations: Vec<i8>) -> Measure {
    Measure {
        id: random_id(),
        staff: Some(Staff {
            id: random_id(),
            n: 1,
            layers: vec![Layer {
                id: random_id(),
                n: 1,
                elements: notes_from_durations(durations),
                ..Layer::default()
            }],
            ..Staff::default()
        }),
        ..Measure::default()
    }
}
fn measures_from_durations(durations: Vec<i8>, max: f32) -> Vec<Measure> {
    SplitDurations { max, durations }
        .map(measure_from_durations)
        .collect()
}

// fn propositions_from_durations(durations: Vec<i8>, max: f32) -> Vec<Measure> {
//     SplitDurations { max, durations }
//         .map(measure_from_durations)
//         .chunks(2)
//         .collect()
// }

struct SplitDurations {
    max: f32,
    durations: Vec<i8>,
}
impl Iterator for SplitDurations {
    type Item = Vec<i8>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.durations.len() == 0 {
            return None;
        }
        let mut accumulator: f32 = 0.0;
        for (idx, val) in self
            .durations
            .iter()
            .filter(|&&d| d != 0)
            .map(|d| 1.0 / d.abs() as f32)
            .enumerate()
        {
            accumulator += val;
            if accumulator >= self.max {
                return Some(self.durations.drain(..=idx).collect());
            }
        }
        Some(self.durations.drain(..).collect())
    }
}

pub fn series1_time2() -> Mei {
    Exercise {
        name: "Série 1: 2/4".to_owned(),
        propositions: vec![
            Proposition {
                measures: vec![
                    measure_from_durations(vec![4, 4]),
                    measure_from_durations(vec![4, -4]),
                ],
            },
            Proposition {
                measures: vec![
                    measure_from_durations(vec![4, 8, 8]),
                    measure_from_durations(vec![4, -4]),
                ],
            },
        ],
    }
    .into_mei()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_eq_text;

    #[test]
    fn split_durations() {
        let expected: Vec<Vec<i8>> = vec![
            vec![4, 4],
            vec![4, -4],
            vec![4, 8, 8],
            vec![4, -4],
            vec![8, 8, 4],
            vec![4, -4],
            vec![8, 8, 8, 8],
            vec![8, -8, -4],
            vec![8, 8, 8, 8],
            vec![4, 8, -8],
            vec![4, 8, 8],
            vec![8, 8, 8, -8],
            vec![8, 8, 4],
            vec![4, 8, -8],
            vec![8, 8, 8, 8],
            vec![8, 8, 8, -8],
            vec![4, 8, 8],
            vec![4, 8, -8],
            vec![2],
            vec![8, 8, 8, -8],
        ];

        let mut iterator = SplitDurations {
            max: 2.0 / 4.0,
            durations: expected.iter().flatten().map(|x| x.to_owned()).collect(),
        };

        for (i, e) in expected.into_iter().enumerate() {
            let got = iterator.next();
            let expected = Some(e);
            println!("Iteration {}: got {:?} expecting {:?}", i, got, expected);
            assert_eq!(got, expected);
        }
        let got = iterator.next();
        println!("Last iteration: got {:?} expecting None", got);
        assert_eq!(got, None);
    }
}
