use std::{time::Duration, usize};

use rand::distr::Iter;
use serde_json::map;

use crate::mei::*;

struct Proposition {
    measures: Vec<Measure>,
}
impl Proposition {
    // fn from_measures(measures: Vec<Measure>) -> Proposition {
    //     durations.iter().map(|durations|{

    //     })
    // }

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

struct Durations<'a> {
    durs: &'a [i8],
}
impl<'a> Iterator for Durations<'a> {
    type Item = &'a [i8];
    fn next(&mut self) -> Option<Self::Item> {
        if self.durs.len() == 0 {
            return None;
        }
        let accumulator: f32 = 0.0;
        let counter = 0;
        for d in self.durs {
            // let absolute = if d > 0 { d } else { -d };
            // let inverse = 1 as f32 / absolute as f32;
            // accumulator += inverse;
            // if accumulator > 1 {
            //     result.append(current_measure);
            // }
        }
        let (left, right) = self.durs.split_at(counter);
        self.durs = right;
        return Some(left);
    }
}

// fn measures_from_durations(durations: Vec<i8>) -> Iter {}

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
