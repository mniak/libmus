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

pub struct Mei {
    pub header: Option<Header>,
    pub music: Option<Music>,
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
    pub fn to_xml(self) -> Result<String, SeError> {
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
pub struct Header {
    #[serde(rename = "fileDesc")]
    pub file_descriptor: FileDescriptor,
}

#[derive(Serialize)]
pub struct FileDescriptor {
    #[serde(rename = "titleStmt")]
    pub title_statement: TitleStatement,
}
#[derive(Serialize)]
pub struct TitleStatement {
    #[serde(rename = "title")]
    pub titles: Vec<Title>,
}
#[derive(Serialize)]
pub struct Title {
    #[serde(rename = "@type")]
    pub type_: String,
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Serialize)]
#[serde(rename = "music")]
pub struct Music {
    pub body: Body,
}
#[derive(Serialize)]
pub struct Body {
    pub mdiv: Mdiv,
}
#[derive(Serialize)]
pub struct Mdiv {
    pub score: Score,
}
#[derive(Serialize)]
pub struct Score {
    #[serde(rename = "scoreDef")]
    pub score_definition: ScoreDefinition,
    pub section: Section,
}
#[derive(Serialize)]
pub struct ScoreDefinition {
    #[serde(rename = "@measureNumbers", skip_serializing_if = "Option::is_none")]
    pub measure_numbers: Option<bool>,
    #[serde(rename = "staffGrp")]
    pub staff_group: StaffGroup,
}
#[derive(Serialize)]
pub struct StaffGroup {
    #[serde(rename = "staffDef")]
    pub staff_definition: StaffDefinition,
}
#[derive(Serialize)]
pub struct StaffDefinition {
    #[serde(rename = "@n")]
    pub n: u16,
    #[serde(rename = "@lines")]
    pub lines: u8,
    #[serde(rename = "@lines.visible")]
    pub lines_visible: bool,
    #[serde(rename = "@meter.count")]
    pub meter_count: u8,
    #[serde(rename = "@meter.unit")]
    pub meter_unit: u8,
}
#[derive(Serialize)]
pub struct Section {
    #[serde(rename = "@xml:id")]
    pub id: String,
    #[serde(rename = "measure")]
    pub measures: Vec<Measure>,
}

#[derive(Serialize, Default)]
#[serde(rename = "measure")]
pub struct Measure {
    #[serde(rename = "@xml:id")]
    pub id: String,
    #[serde(rename = "@n")]
    pub n: u16,

    #[serde(rename = "mNum", skip_serializing_if = "Option::is_none")]
    pub number: Option<u16>,
    #[serde(rename = "@left", skip_serializing_if = "Option::is_none")]
    pub left_bar: Option<Bar>,
    #[serde(rename = "@right", skip_serializing_if = "Option::is_none")]
    pub right_bar: Option<Bar>,
    #[serde(rename = "staff", skip_serializing_if = "Option::is_none")]
    pub staff: Option<Staff>,
}
impl Measure {
    pub fn new<S: Into<String>>(id: S, n: u16) -> Self {
        Self {
            id: id.into(),
            n,
            ..Measure::default()
        }
    }

    pub fn with_number<I: Into<u16>>(&mut self, number: I) -> &mut Self {
        self.number = Some(number.into());
        return self;
    }
    pub fn with_left_bar(&mut self, left_bar: Bar) -> &mut Self {
        self.left_bar = Some(left_bar);
        return self;
    }
    pub fn with_right_bar(&mut self, right_bar: Bar) -> &mut Self {
        self.right_bar = Some(right_bar);
        return self;
    }
    pub fn with_staff(&mut self, staff: Staff) -> &mut Self {
        self.staff = Some(staff);
        return self;
    }
}

#[derive(Serialize)]
pub enum Bar {
    #[serde(rename = "dbl")]
    Double,
    #[serde(rename = "single")]
    Single,
}

#[derive(Serialize, Default)]
pub struct Staff {
    #[serde(rename = "@xml:id")]
    pub id: String,
    #[serde(rename = "@n")]
    pub n: u16,
    #[serde(rename = "layer")]
    pub layers: Vec<Layer>,
}

#[derive(Serialize, Default)]
pub struct Layer {
    #[serde(rename = "@xml:id")]
    pub id: String,
    #[serde(rename = "@n")]
    pub n: u16,
    // #[serde(rename = "note")]
    #[serde(rename = "$value")]
    pub elements: Vec<NoteOrRest>,
}

#[derive(Serialize)]
pub enum NoteOrRest {
    #[serde(rename = "note")]
    Note(Note),
    #[serde(rename = "rest")]
    Rest(Rest),
}

#[derive(Serialize)]
pub struct Note {
    #[serde(rename = "@xml:id")]
    pub id: String,
    #[serde(rename = "@dur")]
    pub duration: u8,
    #[serde(rename = "@pname")]
    pub pitch: PitchName,
    #[serde(rename = "@oct")]
    pub octave: u8,
}

impl Default for Note {
    fn default() -> Self {
        Self {
            id: Default::default(),
            duration: Default::default(),
            pitch: PitchName::A,
            octave: Default::default(),
        }
    }
}

#[derive(Serialize, Default)]
pub struct Rest {
    #[serde(rename = "@xml:id")]
    pub id: String,
    #[serde(rename = "@dur")]
    pub duration: u8,
}

#[derive(Serialize)]
pub enum PitchName {
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
    fn measure_new() {
        let mut m = Measure::new("m1", 1);
        assert_eq!("m1", m.id);
        assert_eq!(1, m.n);
    }

    #[test]
    fn measure_serialize() {
        let expected = r##"<measure xml:id="m42j4hb" n="1" left="dbl" right="single">
  <mNum>123</mNum>
  <staff xml:id="m1s1" n="1">
    <layer xml:id="m1s1l1" n="1">
      <note xml:id="n14c3kqh" dur="4" pname="e" oct="5"/>
      <rest xml:id="r11z73rv" dur="4"/>
      <note xml:id="n16dpotb" dur="4" pname="e" oct="5"/>
    </layer>
  </staff>
</measure>"##;

        let mut m = Measure::new("m42j4hb", 1);
        m.left_bar = Some(Bar::Double);
        m.right_bar = Some(Bar::Single);
        m.number = Some(123);
        m.staff = Some(Staff {
            id: "m1s1".to_owned(),
            n: 1,
            layers: vec![Layer {
                id: "m1s1l1".to_owned(),
                n: 1,
                elements: vec![
                    NoteOrRest::Note(Note {
                        id: "n14c3kqh".to_owned(),
                        duration: 4,
                        pitch: PitchName::E,
                        octave: 5,
                    }),
                    NoteOrRest::Rest(Rest {
                        id: "r11z73rv".to_owned(),
                        duration: 4,
                    }),
                    NoteOrRest::Note(Note {
                        id: "n16dpotb".to_owned(),
                        duration: 4,
                        pitch: PitchName::E,
                        octave: 5,
                    }),
                ],
            }],
        });

        let result = pretty_xml(m).unwrap();
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
