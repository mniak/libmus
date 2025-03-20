// use std::io::Cursor;

// use quick_xml::{
//     events::{BytesStart, Event},
//     Writer,
// };
// use serde::Deserialize;

// #[derive(Deserialize)]
// struct Mei {
//     meiHead: MeiHead,
//     music: Music,
// }
// #[derive(Deserialize)]
// struct MeiHead {
//     fileDesc: String,
// }

// #[derive(Deserialize)]
// struct TitleStmt {
//     title: Title,
// }
// #[derive(Deserialize)]
// struct Title {
//     #[serde(rename = "@type")]
//     type_: String,
//     #[serde(rename = "$value")]
//     value: String,
// }
// #[derive(Deserialize)]
// struct Music {
//     body: Body,
// }
// #[derive(Deserialize)]
// struct Body {
//     mdiv: Mdiv,
// }
// #[derive(Deserialize)]
// struct Mdiv {
//     score: Score,
// }
// #[derive(Deserialize)]
// struct Score {
//     scoreDef: ScoreDef,
//     section: Section,
// }
// #[derive(Deserialize)]
// struct ScoreDef {
//     staffGrp: StaffGrp,
// }
// #[derive(Deserialize)]
// struct StaffGrp {
//     staffDef: StaffDef,
// }
// #[derive(Deserialize)]
// struct StaffDef {
//     #[serde(rename = "@n")]
//     n: String,
//     #[serde(rename = "@lines")]
//     lines: String,
//     #[serde(rename = "@lines.visible")]
//     linesVisible: String,
//     #[serde(rename = "@meter.count")]
//     meterCount: String,
//     #[serde(rename = "@meter.unit")]
//     meterUnit: String,
// }
// #[derive(Deserialize)]
// struct Section {}

// // <measure xml:id="m42j4hb" left="dbl" right="single" n="1">
// //   <mNum>1</mNum>
// //   <staff xml:id="m1s1" n="1">
// //     <layer xml:id="m1s1l1" n="1">
// //       <note xml:id="n14c3kqh" dur="4" pname="e" oct="5" />
// //       <note xml:id="n16dpotb" dur="4" pname="e" oct="5" />
// //     </layer>
// //   </staff>
// // </measure>
// struct Measure {
//   mNum: u16,
//   staff: Staff,
// }
// struct Staff {

// }

// pub fn generate1_2_4() -> Result<Measure, String> {
//     Ok(Measure {
//         mNum: todo!(),
//         staff: todo!(),
//     })
//     // let mut writer = Writer::new(Cursor::new(Vec::new()));
//     // let mut mei = BytesStart::new("mei");
//     // mei.push_attribute(("xmlns", "http://www.music-encoding.org/ns/mei"));
//     // mei.push_attribute(("meiversion", "5.1"));
//     // writer
//     //     .write_event(Event::Start(mei))
//     //     .map_err(|e| e.to_string())?;
//     // return String::from_utf8(writer.into_inner().into_inner()).map_err(|e| e.to_string());
// }

// #[cfg(test)]
// mod tests {
//     use crate::testutils;

//     use super::*;

//     #[test]
//     fn generate() {
//         //         let expected = r###"<?xml version="1.0" encoding="UTF-8"?>
//         // <?xml-model href="https://music-encoding.org/schema/5.1/mei-basic.rng" type="application/xml" schematypens="http://relaxng.org/ns/structure/1.1"?>
//         // <?xml-model href="https://music-encoding.org/schema/5.1/mei-basic.rng" type="application/xml" schematypens="http://purl.oclc.org/dsdl/schematron"?>
//         // <mei xmlns="http://www.music-encoding.org/ns/mei" meiversion="5.1">
//         //   <meiHead>
//         //     <fileDesc>
//         //       <titleStmt>
//         //         <title type="main">Primeira série</title>
//         //       </titleStmt>
//         //     </fileDesc>
//         //   </meiHead>
//         //   <music>
//         //     <body>
//         //       <mdiv>
//         //         <score>
//         //           <scoreDef measureNumbers="true">
//         //             <staffGrp>
//         //               <staffDef n="1" lines="5" lines.visible="true" meter.count="2" meter.unit="4">
//         //               </staffDef>
//         //             </staffGrp>
//         //           </scoreDef>
//         //           <section xml:id="s1">
//         //             <measure xml:id="m42j4hb" left="dbl" right="single" n="1">
//         //               <mNum>1</mNum>
//         //               <staff xml:id="m1s1" n="1">
//         //                 <layer xml:id="m1s1l1" n="1">
//         //                   <note xml:id="n14c3kqh" dur="4" pname="e" oct="5" />
//         //                   <note xml:id="n16dpotb" dur="4" pname="e" oct="5" />
//         //                 </layer>
//         //               </staff>
//         //             </measure>
//         //             <measure xml:id="msb0m6f" right="dbl" left="single" n="2">
//         //               <mNum></mNum>
//         //               <staff xml:id="m2s1" n="1">
//         //                 <layer xml:id="m2s1l1" n="1">
//         //                   <note xml:id="n5ve2xh" dur="4" pname="e" oct="5" />
//         //                   <rest xml:id="r11z73rv" dur="4" />
//         //                 </layer>
//         //               </staff>
//         //             </measure>
//         //             <measure xml:id="m1812i3x" left="dbl" right="single" n="3">
//         //               <mNum>2</mNum>
//         //               <staff xml:id="m3s1" n="1">
//         //                 <layer xml:id="m3s1l1" n="1">
//         //                   <note xml:id="nq7gfuh" dur="4" pname="e" oct="5" />
//         //                   <beam>
//         //                     <note xml:id="n22547l" dur="8" pname="e" oct="5" />
//         //                     <note xml:id="nm124p" dur="8" pname="e" oct="5" />
//         //                   </beam>
//         //                 </layer>
//         //               </staff>
//         //             </measure>
//         //             <measure xml:id="m1xeafja" right="dbl" left="single" n="4">
//         //               <mNum></mNum>
//         //               <staff xml:id="m4s1" n="1">
//         //                 <layer xml:id="m4s1l1" n="1">
//         //                   <note xml:id="n1wm5mq1" dur="4" pname="e" oct="5" />
//         //                   <rest xml:id="rz7u014" dur="4" />
//         //                 </layer>
//         //               </staff>
//         //             </measure>
//         //             <measure xml:id="me0t7qy" left="dbl" right="single" n="5">
//         //               <mNum>3</mNum>
//         //               <staff xml:id="m5s1" n="1">
//         //                 <layer xml:id="m5s1l1" n="1">
//         //                   <beam>
//         //                     <note xml:id="n1ublw1k" dur="8" pname="e" oct="5" />
//         //                     <note xml:id="n1psslru" dur="8" pname="e" oct="5" />
//         //                   </beam>
//         //                   <note xml:id="n1lxyy23" dur="4" pname="e" oct="5" />
//         //                 </layer>
//         //               </staff>
//         //             </measure>
//         //             <measure xml:id="mxcs6v2" right="dbl" left="single" n="6">
//         //               <mNum></mNum>
//         //               <staff xml:id="m6s1" n="1">
//         //                 <layer xml:id="m6s1l1" n="1">
//         //                   <note xml:id="n1rdrp60" dur="4" pname="e" oct="5" />
//         //                   <rest xml:id="r8v92vr" dur="4" />
//         //                 </layer>
//         //               </staff>
//         //             </measure>
//         //             <measure xml:id="mslkcwc" n="7" left="dbl" right="single" break="no">
//         //               <mNum>4</mNum>
//         //               <staff xml:id="m7s1" n="1">
//         //                 <layer xml:id="m7s1l1" n="1">
//         //                   <beam>
//         //                     <note xml:id="nobjg0x" dur="8" pname="e" oct="5" />
//         //                     <note xml:id="n14ink8n" dur="8" pname="e" oct="5" />
//         //                   </beam>
//         //                   <beam>
//         //                     <note xml:id="nueygfa" dur="8" pname="e" oct="5" />
//         //                     <note xml:id="n1t0k81p" dur="8" pname="e" oct="5" />
//         //                   </beam>
//         //                 </layer>
//         //               </staff>
//         //             </measure>
//         //             <measure xml:id="mkj35u9" right="dbl" left="single" n="8">
//         //               <mNum></mNum>
//         //               <staff xml:id="m8s1" n="1">
//         //                 <layer xml:id="m8s1l1" n="1">
//         //                   <note xml:id="n15lthwj" dur="8" pname="e" oct="5" />
//         //                   <rest xml:id="r1sczkh9" type="mscore-beam-none" dur="8" />
//         //                   <rest xml:id="r1d4kz3w" dur="4" />
//         //                 </layer>
//         //               </staff>
//         //             </measure>
//         //             <measure xml:id="mec8j3f" left="dbl" right="single" n="9">
//         //               <mNum>5</mNum>
//         //               <staff xml:id="m9s1" n="1">
//         //                 <layer xml:id="m9s1l1" n="1">
//         //                   <beam>
//         //                     <note xml:id="nob5uxs" dur="8" pname="e" oct="5" />
//         //                     <note xml:id="nc2g9j5" dur="8" pname="e" oct="5" />
//         //                   </beam>
//         //                   <beam>
//         //                     <note xml:id="nfodwtc" dur="8" pname="e" oct="5" />
//         //                     <note xml:id="n10330r0" dur="8" pname="e" oct="5" />
//         //                   </beam>
//         //                 </layer>
//         //               </staff>
//         //             </measure>
//         //             <measure xml:id="mle3gt9" right="dbl" left="single" n="10">
//         //               <mNum></mNum>
//         //               <staff xml:id="m10s1" n="1">
//         //                 <layer xml:id="m10s1l1" n="1">
//         //                   <note xml:id="nq8e51o" dur="4" pname="e" oct="5" />
//         //                   <note xml:id="n1bn585q" dur="8" pname="e" oct="5" />
//         //                   <rest xml:id="r145bzbu" type="mscore-beam-none" dur="8" />
//         //                 </layer>
//         //               </staff>
//         //             </measure>
//         //             <measure xml:id="m1bqezlj" left="dbl" right="single" n="11">
//         //               <mNum>6</mNum>
//         //               <staff xml:id="m11s1" n="1">
//         //                 <layer xml:id="m11s1l1" n="1">
//         //                   <note xml:id="n1szw9pi" dur="4" pname="e" oct="5" />
//         //                   <beam>
//         //                     <note xml:id="nt084g3" dur="8" pname="e" oct="5" />
//         //                     <note xml:id="nexz9xu" dur="8" pname="e" oct="5" />
//         //                   </beam>
//         //                 </layer>
//         //               </staff>
//         //             </measure>
//         //             <measure xml:id="m2jxael" right="dbl" left="single" n="12">
//         //               <mNum></mNum>
//         //               <staff xml:id="m12s1" n="1">
//         //                 <layer xml:id="m12s1l1" n="1">
//         //                   <beam>
//         //                     <note xml:id="ncy3ii7" dur="8" pname="e" oct="5" />
//         //                     <note xml:id="nkdwpst" dur="8" pname="e" oct="5" />
//         //                   </beam>
//         //                   <note xml:id="n93pbp5" dur="8" pname="e" oct="5" />
//         //                   <rest xml:id="r1g16zho" type="mscore-beam-none" dur="8" />
//         //                 </layer>
//         //               </staff>
//         //             </measure>
//         //             <measure xml:id="mlnnt80" left="dbl" right="single" n="13">
//         //               <mNum>7</mNum>
//         //               <staff xml:id="m13s1" n="1">
//         //                 <layer xml:id="m13s1l1" n="1">
//         //                   <beam>
//         //                     <note xml:id="n1sianrb" dur="8" pname="e" oct="5" />
//         //                     <note xml:id="n1donbbp" dur="8" pname="e" oct="5" />
//         //                   </beam>
//         //                   <note xml:id="n10h3nni" dur="4" pname="e" oct="5" />
//         //                 </layer>
//         //               </staff>
//         //             </measure>
//         //             <measure xml:id="m1v0h2ps" right="dbl" left="single" n="14">
//         //               <mNum></mNum>
//         //               <staff xml:id="m14s1" n="1">
//         //                 <layer xml:id="m14s1l1" n="1">
//         //                   <note xml:id="n7qklim" dur="4" pname="e" oct="5" />
//         //                   <note xml:id="n10qhaxr" dur="8" pname="e" oct="5" />
//         //                   <rest xml:id="rnqk535" type="mscore-beam-none" dur="8" />
//         //                 </layer>
//         //               </staff>
//         //             </measure>
//         //             <measure xml:id="m2fg2lx" left="dbl" right="single" n="15">
//         //               <mNum>8</mNum>
//         //               <staff xml:id="m15s1" n="1">
//         //                 <layer xml:id="m15s1l1" n="1">
//         //                   <beam>
//         //                     <note xml:id="n18ao39a" dur="8" pname="e" oct="5" />
//         //                     <note xml:id="n1m9n3tn" dur="8" pname="e" oct="5" />
//         //                   </beam>
//         //                   <beam>
//         //                     <note xml:id="n41h2vo" dur="8" pname="e" oct="5" />
//         //                     <note xml:id="njvu1t2" dur="8" pname="e" oct="5" />
//         //                   </beam>
//         //                 </layer>
//         //               </staff>
//         //             </measure>
//         //             <measure xml:id="m2ssg4h" right="dbl" left="single" n="16">
//         //               <mNum></mNum>
//         //               <staff xml:id="m16s1" n="1">
//         //                 <layer xml:id="m16s1l1" n="1">
//         //                   <beam>
//         //                     <note xml:id="n1m91jix" dur="8" pname="e" oct="5" />
//         //                     <note xml:id="n1doergj" dur="8" pname="e" oct="5" />
//         //                   </beam>
//         //                   <note xml:id="n5yttd6" dur="8" pname="e" oct="5" />
//         //                   <rest xml:id="rroh03i" type="mscore-beam-none" dur="8" />
//         //                 </layer>
//         //               </staff>
//         //             </measure>
//         //             <measure xml:id="m1moresd" left="dbl" right="single" n="17">
//         //               <mNum>9</mNum>
//         //               <staff xml:id="m17s1" n="1">
//         //                 <layer xml:id="m17s1l1" n="1">
//         //                   <note xml:id="nddb11i" dur="4" pname="e" oct="5" />
//         //                   <beam>
//         //                     <note xml:id="n1qhzii" dur="8" pname="e" oct="5" />
//         //                     <note xml:id="nv95qh7" dur="8" pname="e" oct="5" />
//         //                   </beam>
//         //                 </layer>
//         //               </staff>
//         //             </measure>
//         //             <measure xml:id="mh9gvwa" right="dbl" left="single" n="18">
//         //               <mNum></mNum>
//         //               <staff xml:id="m18s1" n="1">
//         //                 <layer xml:id="m18s1l1" n="1">
//         //                   <note xml:id="n601ehm" dur="4" pname="e" oct="5" />
//         //                   <note xml:id="n8glv24" dur="8" pname="e" oct="5" />
//         //                   <rest xml:id="r1t67s13" type="mscore-beam-none" dur="8" />
//         //                 </layer>
//         //               </staff>
//         //             </measure>
//         //             <measure xml:id="mqsmzrr" left="dbl" right="single" n="19">
//         //               <mNum>10</mNum>
//         //               <staff xml:id="m19s1" n="1">
//         //                 <layer xml:id="m19s1l1" n="1">
//         //                   <note xml:id="n1l4er9p" dur="2" pname="e" oct="5" />
//         //                 </layer>
//         //               </staff>
//         //             </measure>
//         //             <measure xml:id="m15xzjv5" right="dbl" left="single" n="20">
//         //               <mNum></mNum>
//         //               <staff xml:id="m20s1" n="1">
//         //                 <layer xml:id="m20s1l1" n="1">
//         //                   <beam>
//         //                     <note xml:id="nwp5f4h" dur="8" pname="e" oct="5" />
//         //                     <note xml:id="n7gy9rg" dur="8" pname="e" oct="5" />
//         //                   </beam>
//         //                   <note xml:id="ntztwt9" dur="8" pname="e" oct="5" />
//         //                   <rest xml:id="r1va9pl5" type="mscore-beam-none" dur="8" />
//         //                 </layer>
//         //               </staff>
//         //             </measure>
//         //           </section>
//         //         </score>
//         //       </mdiv>
//         //     </body>
//         //   </music>
//         // </mei>"###;

//         let mei = generate1_2_4();
//         // testutils::assert_eq_text!(gen.unwrap(), expected);
//       mei.
//     }
// }
