use quick_xml::se::Serializer;
use quick_xml::SeError;
use serde::ser::SerializeStruct;
use serde::Serialize;

fn pretty_xml<T: Serialize>(item: T) -> Result<String, SeError> {
    let mut buffer = String::new();
    let mut ser = Serializer::new(&mut buffer);
    ser.indent(' ', 2);
    item.serialize(ser)?;
    Ok(buffer)
}

struct Mei {
    header: Option<Header>,
    music: Option<Music>,
}

impl Serialize for Mei {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("mei", 3)?;
        s.serialize_field("@xmlns", "http://www.music-encoding.org/ns/mei")?;
        s.serialize_field("@meiversion", "5.1")?;
        s.serialize_field("meiHead", &self.header)?;
        s.serialize_field("music", &self.music)?;
        s.end()
    }
}

impl Mei {
    fn to_xml(self) -> Result<String, SeError> {
        let serialized = pretty_xml(self)?;
        let mut result = String::new();
        result += "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
        result += "<?xml-model href=\"https://music-encoding.org/schema/5.1/mei-basic.rng\" type=\"application/xml\" schematypens=\"http://relaxng.org/ns/structure/1.1\"?>\n";
        result += "<?xml-model href=\"https://music-encoding.org/schema/5.1/mei-basic.rng\" type=\"application/xml\" schematypens=\"http://purl.oclc.org/dsdl/schematron\"?>\n";
        result += &serialized;
        Ok(result)
    }
}

#[derive(Serialize)]
#[serde(rename = "meiHead")]
struct Header {
    #[serde(rename = "fileDesc")]
    file_descriptor: FileDescriptor,
}

#[derive(Serialize)]
struct FileDescriptor {
    #[serde(rename = "titleStmt")]
    title_statement: TitleStatement,
}
#[derive(Serialize)]
struct TitleStatement {
    #[serde(rename = "title")]
    titles: Vec<Title>,
}
#[derive(Serialize)]
struct Title {
    #[serde(rename = "@type")]
    type_: String,
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize)]
#[serde(rename = "music")]
struct Music {
    body: Body,
}
#[derive(Serialize)]
struct Body {
    mdiv: Mdiv,
}
#[derive(Serialize)]
struct Mdiv {
    score: Score,
}
#[derive(Serialize)]
struct Score {
    #[serde(rename = "scoreDef")]
    score_definition: ScoreDefinition,
    section: Section,
}
#[derive(Serialize)]
struct ScoreDefinition {
    #[serde(rename = "@measureNumbers", skip_serializing_if = "Option::is_none")]
    measure_numbers: Option<bool>,
    #[serde(rename = "staffGrp")]
    staff_group: StaffGroup,
}
#[derive(Serialize)]
struct StaffGroup {
    #[serde(rename = "staffDef")]
    staff_definition: StaffDefinition,
}
#[derive(Serialize)]
struct StaffDefinition {
    #[serde(rename = "@n")]
    n: u16,
    #[serde(rename = "@lines")]
    lines: u8,
    #[serde(rename = "@lines.visible")]
    lines_visible: bool,
    #[serde(rename = "@meter.count")]
    meter_count: u8,
    #[serde(rename = "@meter.unit")]
    meter_unit: u8,
}
#[derive(Serialize)]
struct Section {
    #[serde(rename = "@xml:id")]
    id: String,
    #[serde(rename = "measure")]
    measures: Vec<Measure>,
}

#[derive(Debug, Serialize)]
#[serde(rename = "measure")]
pub struct Measure {
    #[serde(rename = "@xml:id")]
    id: String,
    #[serde(rename = "@left", skip_serializing_if = "Option::is_none")]
    left_bar: Option<BarRendition>,
    #[serde(rename = "@right", skip_serializing_if = "Option::is_none")]
    right_bar: Option<BarRendition>,
    #[serde(rename = "@n")]
    n: u16,

    #[serde(rename = "mNum", skip_serializing_if = "Option::is_none")]
    number: Option<u16>,
    #[serde(rename = "staff", skip_serializing_if = "Option::is_none")]
    staff: Option<Staff>,
}
#[derive(Debug, Serialize)]
enum BarRendition {
    #[serde(rename = "dbl")]
    Double,
    #[serde(rename = "single")]
    Single,
}

#[derive(Debug, Serialize)]
struct Staff {
    #[serde(rename = "@xml:id")]
    id: String,
    #[serde(rename = "@n")]
    n: u16,
    #[serde(rename = "layer")]
    layers: Vec<Layer>,
}
#[derive(Debug, Serialize)]

struct Layer {
    #[serde(rename = "@xml:id")]
    id: String,
    #[serde(rename = "@n")]
    n: u16,
    #[serde(rename = "note")]
    notes: Vec<Note>,
}
#[derive(Debug, Serialize)]

struct Note {
    #[serde(rename = "@xml:id")]
    id: String,
    #[serde(rename = "@dur")]
    duration: u16,
    #[serde(rename = "@pname")]
    pitch: PitchName,
    #[serde(rename = "@oct")]
    octave: u8,
}

#[derive(Debug, Serialize)]
enum PitchName {
    #[serde(rename = "a")]
    A,
    #[serde(rename = "b")]
    B,
    #[serde(rename = "c")]
    C,
    #[serde(rename = "d")]
    D,
    #[serde(rename = "e")]
    E,
    #[serde(rename = "f")]
    F,
    #[serde(rename = "g")]
    G,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_eq_text;

    #[test]
    fn measure_serialize() {
        let expected = r##"<measure xml:id="m42j4hb" left="dbl" right="single" n="1">
  <mNum>123</mNum>
  <staff xml:id="m1s1" n="1">
    <layer xml:id="m1s1l1" n="1">
      <note xml:id="n14c3kqh" dur="4" pname="e" oct="5"/>
      <note xml:id="n16dpotb" dur="4" pname="e" oct="5"/>
    </layer>
  </staff>
</measure>"##;

        let measure = Measure {
            id: "m42j4hb".to_owned(),
            left_bar: Some(BarRendition::Double),
            right_bar: Some(BarRendition::Single),
            n: 1,
            number: Some(123),
            staff: Some(Staff {
                id: "m1s1".to_owned(),
                n: 1,
                layers: vec![Layer {
                    id: "m1s1l1".to_owned(),
                    n: 1,
                    notes: vec![
                        Note {
                            id: "n14c3kqh".to_owned(),
                            duration: 4,
                            pitch: PitchName::E,
                            octave: 5,
                        },
                        Note {
                            id: "n16dpotb".to_owned(),
                            duration: 4,
                            pitch: PitchName::E,
                            octave: 5,
                        },
                    ],
                }],
            }),
        };

        let result = pretty_xml(measure).unwrap();
        assert_eq_text!(expected, &result);
    }
    #[test]
    fn mei_serialize() {
        let expected = r##"<?xml version="1.0" encoding="UTF-8"?>
<?xml-model href="https://music-encoding.org/schema/5.1/mei-basic.rng" type="application/xml" schematypens="http://relaxng.org/ns/structure/1.1"?>
<?xml-model href="https://music-encoding.org/schema/5.1/mei-basic.rng" type="application/xml" schematypens="http://purl.oclc.org/dsdl/schematron"?>
<mei xmlns="http://www.music-encoding.org/ns/mei" meiversion="5.1">
  <meiHead/>
  <music/>
</mei>"##;

        let mei = Mei {
            header: None,
            music: None,
        };

        let result = mei.to_xml().unwrap();
        assert_eq_text!(expected, &result);
    }
    #[test]
    fn mei_head_serialize() {
        let expected = r##"<meiHead>
  <fileDesc>
    <titleStmt>
      <title type="main">My document</title>
    </titleStmt>
  </fileDesc>
</meiHead>"##;

        let mei_head = Header {
            file_descriptor: FileDescriptor {
                title_statement: TitleStatement {
                    titles: vec![Title {
                        type_: "main".to_owned(),
                        value: "My document".to_owned(),
                    }],
                },
            },
        };

        let result = pretty_xml(mei_head).unwrap();
        assert_eq_text!(expected, &result);
    }

    #[test]
    fn music_serialize() {
        let expected = r##"<music>
  <body>
    <mdiv>
      <score>
        <scoreDef measureNumbers="true">
          <staffGrp>
            <staffDef n="1" lines="5" lines.visible="true" meter.count="2" meter.unit="4"/>
          </staffGrp>
        </scoreDef>
        <section xml:id="section1">
          <measure xml:id="measure1" n="8"/>
          <measure xml:id="measure2" n="12"/>
        </section>
      </score>
    </mdiv>
  </body>
</music>"##;

        let music = Music {
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
                                    meter_count: 2,
                                    meter_unit: 4,
                                },
                            },
                        },
                        section: Section {
                            id: "section1".to_owned(),
                            measures: vec![
                                Measure {
                                    id: "measure1".to_owned(),
                                    n: 8,
                                    left_bar: None,
                                    right_bar: None,
                                    number: None,
                                    staff: None,
                                },
                                Measure {
                                    id: "measure2".to_owned(),
                                    n: 12,
                                    left_bar: None,
                                    right_bar: None,
                                    number: None,
                                    staff: None,
                                },
                            ],
                        },
                    },
                },
            },
        };

        let result = pretty_xml(music).unwrap();
        assert_eq_text!(expected, &result);
    }
}
