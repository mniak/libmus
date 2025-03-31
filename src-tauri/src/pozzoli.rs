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
// fn measures_from_durations(durations: Vec<i8>, max: f32) -> Vec<Measure> {
//     SplitDurations { max, durations }
//         .map(measure_from_durations)
//         .collect()
// }

// fn propositions_from_durations(durations: Vec<i8>, max: f32) -> Vec<Measure> {
//     SplitDurations { max, durations }
//         .map(measure_from_durations)
//         .chunks(2)
//         .collect()
// }

fn split_durations<I: Iterator<Item = i8>>(
    threshold: f32,
    iter: I,
) -> impl Iterator<Item = Vec<i8>> {
    iter.into_groups(threshold, |d| 1.0 / d.abs() as f32)
}

trait IntoGroupsExt<T>: Iterator<Item = T> + Sized {
    fn into_groups<F>(self, threshold: f32, get_value: F) -> IntoGroups<T, Self, F>
    where
        F: Fn(&T) -> f32,
    {
        IntoGroups {
            iter: self,
            threshold,
            get_value,
        }
    }
}

impl<T, I> IntoGroupsExt<T> for I where I: Iterator<Item = T> {}

struct IntoGroups<T, I, F>
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> f32,
{
    iter: I,
    threshold: f32,
    get_value: F,
}

impl<T, I, F> Iterator for IntoGroups<T, I, F>
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> f32,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut accumulator: f32 = 0.0;
        let mut group = Vec::new();

        while let Some(d) = self.iter.next() {
            accumulator += (&self.get_value)(&d);
            group.push(d);

            if accumulator >= self.threshold {
                return Some(group);
            }
        }

        match group.is_empty() {
            true => None,
            false => Some(group),
        }
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

        let mut iter = expected
            .clone()
            .into_iter()
            .flatten()
            .into_groups(2.0 / 4.0, |d| 1.0 / d.abs() as f32);

        // let mut iter = super::split_durations(2.0 / 4.0, expected.clone().into_iter().flatten());
        for (i, e) in expected.into_iter().enumerate() {
            let got = iter.next();
            let expected = Some(e);
            println!("Iteration {}: got {:?} expecting {:?}", i, got, expected);
            assert_eq!(got, expected);
        }
        let got = iter.next();
        println!("Last iteration: got {:?} expecting None", got);
        assert_eq!(got, None);
    }
}
