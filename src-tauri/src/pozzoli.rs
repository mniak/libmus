use crate::iter::IntoGroupsExt;
use crate::mei;
use itertools::*;
use rand::distr::Iter;
use serde_json::map;
use std::{time::Duration, usize};

struct Proposition {
    measures: Vec<mei::Measure>,
}
impl Proposition {
    fn measures(self) -> Vec<mei::Measure> {
        let count = self.measures.len();
        let x = self
            .measures
            .into_iter()
            .enumerate()
            .map(|(i, mut m)| {
                if i == 0 {
                    m.left_bar = Some(mei::Bar::Double);
                    m.right_bar = Some(mei::Bar::Single);
                } else if i == count - 1 {
                    m.left_bar = Some(mei::Bar::Single);
                    m.right_bar = Some(mei::Bar::Double);
                }
                m
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
    fn into_mei(self) -> mei::Mei {
        let mut counter = 0;
        let measures = self
            .propositions
            .into_iter()
            .enumerate()
            .map(|(idx, prop)| {
                let mut measures = prop.measures();
                measures[0].number = Some(idx as u16 + 1);
                return measures;
            })
            .flatten()
            .enumerate()
            .map(|(idx, mut m)| {
                m.n = idx as u16 + 1;
                m
            });

        mei::Mei {
            header: Some(mei::Header {
                file_descriptor: mei::FileDescriptor {
                    title_statement: mei::TitleStatement {
                        titles: vec![mei::Title {
                            type_: "main".to_owned(),
                            value: self.name,
                        }],
                    },
                },
            }),
            music: Some(mei::Music {
                body: mei::Body {
                    mdiv: mei::Mdiv {
                        score: mei::Score {
                            score_definition: mei::ScoreDefinition {
                                measure_numbers: Some(true),
                                staff_group: mei::StaffGroup {
                                    staff_definition: mei::StaffDefinition {
                                        n: 1,
                                        lines: 5,
                                        lines_visible: false,
                                        meter_count: 2,
                                        meter_unit: 4,
                                    },
                                },
                            },
                            section: mei::Section {
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

fn propositions_from_durations(elements: Vec<Element>, threshold: f32) -> Vec<Proposition> {
    elements
        .into_iter()
        .into_groups(threshold, |el| el.duration())
        .map(|elements| mei::Measure {
            id: mei::random_id(),
            staff: Some(mei::Staff {
                id: mei::random_id(),
                n: 1,
                layers: vec![mei::Layer {
                    id: mei::random_id(),
                    n: 1,
                    elements: elements
                        .iter()
                        .map(|el| match el {
                            Element::Note(n) => mei::NoteOrRest::Note(mei::Note {
                                id: mei::random_id(),
                                duration: *n,
                                pitch: mei::PitchName::E,
                                octave: 5,
                            }),
                            Element::Rest(r) => mei::NoteOrRest::Rest(mei::Rest {
                                //
                                duration: *r,
                                id: mei::random_id(),
                            }),
                            Element::Beam(notes) => mei::NoteOrRest::Beam(mei::Beam {
                                notes: notes
                                    .iter()
                                    .map(|n| mei::Note {
                                        id: mei::random_id(),
                                        duration: *n,
                                        pitch: mei::PitchName::E,
                                        octave: 5,
                                    })
                                    .collect(),
                            }),
                        })
                        .collect(),
                    ..mei::Layer::default()
                }],
                ..mei::Staff::default()
            }),
            ..mei::Measure::default()
        })
        .chunks(2)
        .into_iter()
        .map(|ms| Proposition {
            measures: ms.collect_vec(),
        })
        .collect()
}

fn split_durations<I: Iterator<Item = i8>>(
    threshold: f32,
    iter: I,
) -> impl Iterator<Item = Vec<i8>> {
    iter.into_groups(threshold, |d| 1.0 / d.abs() as f32)
}
#[derive(Debug, PartialEq)]
enum Element {
    Note(u8),
    Rest(u8),
    Beam(Vec<u8>),
}

impl Element {
    fn duration(&self) -> f32 {
        match self {
            Element::Note(d) => 1.0 / *d as f32,
            Element::Rest(d) => 1.0 / *d as f32,
            Element::Beam(d) => d.into_iter().map(|x| 1.0 / (*x as f32)).sum::<f32>(),
        }
    }
}

pub fn series1_time2() -> mei::Mei {
    let durations: Vec<Element> = vec![
        Element::Note(4),
        Element::Note(4),
        Element::Note(4),
        Element::Rest(4),
        //
        Element::Note(4),
        Element::Beam(vec![8, 8]),
        Element::Note(4),
        Element::Rest(4),
        //
        Element::Beam(vec![8, 8]),
        Element::Note(4),
        Element::Note(4),
        Element::Rest(4),
        //
        Element::Beam(vec![8, 8]),
        Element::Beam(vec![8, 8]),
        Element::Note(8),
        Element::Rest(8),
        Element::Rest(4),
        //
        Element::Beam(vec![8, 8]),
        Element::Beam(vec![8, 8]),
        Element::Note(4),
        Element::Note(8),
        Element::Rest(8),
        //
        Element::Note(4),
        Element::Beam(vec![8, 8]),
        Element::Beam(vec![8, 8]),
        Element::Note(8),
        Element::Rest(8),
        //
        Element::Beam(vec![8, 8]),
        Element::Note(4),
        Element::Note(4),
        Element::Note(8),
        Element::Rest(8),
        //
        Element::Beam(vec![8, 8]),
        Element::Beam(vec![8, 8]),
        Element::Beam(vec![8, 8]),
        Element::Note(8),
        Element::Rest(8),
        //
        Element::Note(4),
        Element::Beam(vec![8, 8]),
        Element::Note(4),
        Element::Note(8),
        Element::Rest(8),
        //
        Element::Note(2),
        Element::Beam(vec![8, 8]),
        Element::Note(8),
        Element::Rest(8),
    ];
    let propositions = propositions_from_durations(durations, 2.0 / 4.0);

    Exercise {
        name: "Série 1: 2/4".to_owned(),
        propositions,
    }
    .into_mei()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_durations_1() {
        let input: Vec<Element> = vec![
            Element::Note(4),
            Element::Note(4),
            Element::Note(4),
            Element::Rest(4),
            Element::Note(4),
            Element::Beam(vec![8, 8]),
            Element::Note(4),
            Element::Rest(4),
            Element::Beam(vec![8, 8]),
            Element::Note(4),
            Element::Note(4),
            Element::Rest(4),
        ];

        let expected: Vec<Vec<Element>> = vec![
            vec![
                Element::Note(4),
                Element::Note(4),
                Element::Note(4),
                Element::Rest(4),
            ],
            vec![
                Element::Note(4),
                Element::Beam(vec![8, 8]),
                Element::Note(4),
                Element::Rest(4),
            ],
            vec![
                Element::Beam(vec![8, 8]),
                Element::Note(4),
                Element::Note(4),
                Element::Rest(4),
            ],
        ];

        let mut iter = input.into_iter().into_groups(2.0 / 4.0, |el| el.duration());

        for (i, e) in expected.into_iter().enumerate() {
            let got = iter.next();
            let expected = Some(e);
            println!("Iteration {}: expecting {:?} got {:?} ", i, expected, got);
            assert_eq!(expected, got);
        }
        let got = iter.next();
        println!("Last iteration: expecting None got {:?} ", got);
        assert_eq!(None, got);
    }

    #[test]
    fn split_durations_2() {
        let input: Vec<Element> = vec![
            Element::Note(4),
            Element::Beam(vec![8, 8]),
            Element::Note(4),
            Element::Rest(4),
        ];

        let expected: Vec<Vec<Element>> = vec![
            vec![Element::Note(4), Element::Beam(vec![8, 8])],
            vec![Element::Note(4), Element::Rest(4)],
        ];

        let mut iter = input.into_iter().into_groups(2.0 / 4.0, |el| el.duration());

        for (i, e) in expected.into_iter().enumerate() {
            let got = iter.next();
            let expected = Some(e);
            println!("Iteration {}: expecting {:?} got {:?} ", i, expected, got);
            assert_eq!(expected, got);
        }
        let got = iter.next();
        println!("Last iteration: expecting None got {:?} ", got);
        assert_eq!(None, got);
    }
}
