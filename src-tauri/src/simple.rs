use std::ops::RangeInclusive;

use crate::{
    mei::*,
    theory::{random_pitch_with_octave, PitchNameWithOctave},
};

pub fn quarters_in_quadruple(
    title: String,
    measure_count: u16,
    range: RangeInclusive<PitchNameWithOctave>,
) -> Mei {
    let mut rng = rand::rng();

    let four_pitches_list: Vec<Vec<PitchNameWithOctave>> = (0..measure_count)
        .map(|_| {
            (0..4)
                .map(|_| random_pitch_with_octave(&mut rng, &range))
                .collect()
        })
        .collect();

    let measures = four_pitches_list
        .into_iter()
        .enumerate()
        .map(|(idx, four_pitches)| Measure {
            id: random_id(),
            staff: Some(Staff {
                id: random_id(),
                n: 1,
                layers: vec![Layer {
                    id: random_id(),
                    n: 1,
                    elements: four_pitches
                        .iter()
                        .map(|p| {
                            NoteOrRest::Note(Note {
                                id: random_id(),
                                duration: 4,
                                pitch: p.pitch,
                                octave: p.octave,
                            })
                        })
                        .collect(),
                    ..Layer::default()
                }],
                ..Staff::default()
            }),
            ..Measure::default()
        })
        .enumerate()
        .map(|(idx, mut m)| {
            m.n = idx as u16 + 1;
            m
        });

    return measures_to_mei(title, measures.collect());
}

fn measures_to_mei(title: String, measures: Vec<Measure>) -> Mei {
    Mei {
        header: Some(Header {
            file_descriptor: FileDescriptor {
                title_statement: TitleStatement {
                    titles: vec![Title {
                        type_: "main".to_owned(),
                        value: title,
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
                                    lines_visible: true,
                                    meter_count: 4,
                                    meter_unit: 4,
                                    clef: Option::Some(bass_clef()),
                                },
                            },
                        },
                        section: Section {
                            id: "section1".to_owned(),
                            measures: measures,
                        },
                    },
                },
            },
        }),
    }
}
