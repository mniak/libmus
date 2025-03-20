use quick_xml::se::Serializer;
use quick_xml::{
    events::{BytesStart, Event},
    Writer,
};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Deserialize)]
struct Mei {
    meiHead: MeiHead,
    music: Music,
}
#[derive(Deserialize)]
struct MeiHead {
    fileDesc: String,
}

#[derive(Deserialize)]
struct TitleStmt {
    title: Title,
}
#[derive(Deserialize)]
struct Title {
    #[serde(rename = "@type")]
    type_: String,
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Deserialize)]
struct Music {
    body: Body,
}
#[derive(Deserialize)]
struct Body {
    mdiv: Mdiv,
}
#[derive(Deserialize)]
struct Mdiv {
    score: Score,
}
#[derive(Deserialize)]
struct Score {
    scoreDef: ScoreDef,
    section: Section,
}
#[derive(Deserialize)]
struct ScoreDef {
    staffGrp: StaffGrp,
}
#[derive(Deserialize)]
struct StaffGrp {
    staffDef: StaffDef,
}
#[derive(Deserialize)]
struct StaffDef {
    #[serde(rename = "@n")]
    n: String,
    #[serde(rename = "@lines")]
    lines: String,
    #[serde(rename = "@lines.visible")]
    linesVisible: String,
    #[serde(rename = "@meter.count")]
    meterCount: String,
    #[serde(rename = "@meter.unit")]
    meterUnit: String,
}
#[derive(Deserialize)]
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

    mNum: u16,
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
    n: u16,
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

    fn pretty_xml<T: Serialize>(item: T) -> Result<String, String> {
        let mut buffer = String::new();
        let mut ser = Serializer::new(&mut buffer);
        ser.indent(' ', 2);
        item.serialize(ser).map_err(|e| e.to_string())?;
        Ok(buffer)
    }

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
            mNum: 1,
            staff: Staff {
                id: "m1s1".to_owned(),
                n: 1,
                layers: vec![Layer {
                    id: "m1s1l1".to_owned(),
                    n: 1,
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
}
