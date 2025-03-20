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
    header: Option<MeiHead>,
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
struct MeiHead {
    #[serde(rename = "fileDesc")]
    file_description: String,
}

#[derive(Serialize)]
struct TitleStatement {
    title: Title,
}
#[derive(Serialize)]
struct Title {
    #[serde(rename = "@type")]
    type_: String,
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize)]
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
    #[serde(rename = "staffGrp")]
    staff_group: StaffGrp,
}
#[derive(Serialize)]
struct StaffGrp {
    #[serde(rename = "staffDef")]
    staff_definition: StaffDefinition,
}
#[derive(Serialize)]
struct StaffDefinition {
    #[serde(rename = "@n")]
    number: String,
    #[serde(rename = "@lines")]
    lines: String,
    #[serde(rename = "@lines.visible")]
    lines_visible: String,
    #[serde(rename = "@meter.count")]
    meter_count: String,
    #[serde(rename = "@meter.unit")]
    meter_unit: String,
}
#[derive(Serialize)]
struct Section {}

#[derive(Debug, Serialize)]
#[serde(rename = "measure")]
pub struct Measure {
    #[serde(rename = "@xml:id")]
    id: String,
    #[serde(rename = "@left")]
    left: BarRendition,
    #[serde(rename = "@right")]
    right: BarRendition,
    #[serde(rename = "@n")]
    n: u16,

    #[serde(rename = "mNum")]
    number: u16,
    staff: Staff,
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
    number: u16,
    #[serde(rename = "note")]
    notes: Vec<Note>,
}
#[derive(Debug, Serialize)]

struct Note {
    #[serde(rename = "@xml:id")]
    id: String,
    #[serde(rename = "@dur")]
    dur: u16,
    #[serde(rename = "@pname")]
    pname: PitchName,
    #[serde(rename = "@oct")]
    oct: u8,
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
  <mNum>1</mNum>
  <staff xml:id="m1s1" n="1">
    <layer xml:id="m1s1l1" n="1">
      <note xml:id="n14c3kqh" dur="4" pname="e" oct="5"/>
      <note xml:id="n16dpotb" dur="4" pname="e" oct="5"/>
    </layer>
  </staff>
</measure>"##;

        let measure = Measure {
            id: "m42j4hb".to_owned(),
            left: BarRendition::Double,
            right: BarRendition::Single,
            n: 1,
            number: 1,
            staff: Staff {
                id: "m1s1".to_owned(),
                n: 1,
                layers: vec![Layer {
                    id: "m1s1l1".to_owned(),
                    number: 1,
                    notes: vec![
                        Note {
                            id: "n14c3kqh".to_owned(),
                            dur: 4,
                            pname: PitchName::E,
                            oct: 5,
                        },
                        Note {
                            id: "n16dpotb".to_owned(),
                            dur: 4,
                            pname: PitchName::E,
                            oct: 5,
                        },
                    ],
                }],
            },
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
}
