use tauri::{Emitter, Manager};

mod mei;
mod pozzoli;
#[cfg(test)]
mod test_utils;

#[derive(Clone, serde::Serialize)]
struct MeiChangedEvent {
    mei: String,
}

#[tauri::command]
async fn action(app: tauri::AppHandle) -> Result<(), String> {
    app.emit(
        "meiChanged",
        MeiChangedEvent {
            // mei: pozzoli::generate1_2_4()?,
            mei: todo!(),
        },
    )
    .map_err(|e| "failed".to_owned())
    //  sleep(time::Duration::from_secs(3));
    //  app.emit(
    //      "meiChanged",
    //      MeiChangedEvent {
    //          mei: MEI_POZZOLI.into(),
    //      },
    //  )
    //  .unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
                window.maximize()?;
                window.set_fullscreen(true)?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![action])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

const MEI_POZZOLI: &str = r###"<?xml version="1.0" encoding="UTF-8"?>
<?xml-model href="https://music-encoding.org/schema/5.1/mei-basic.rng" type="application/xml" schematypens="http://relaxng.org/ns/structure/1.1"?>
<?xml-model href="https://music-encoding.org/schema/5.1/mei-basic.rng" type="application/xml" schematypens="http://purl.oclc.org/dsdl/schematron"?>
<mei xmlns="http://www.music-encoding.org/ns/mei" meiversion="5.1">
  <meiHead>
    <fileDesc>
      <titleStmt>
        <!-- <title type="main">Pozzoli</title> -->
        <!-- <title type="subordinate">Primeira série</title> -->
        <title type="main">Primeira série</title>
      </titleStmt>
    </fileDesc>
  </meiHead>
  <music>
    <body>
      <mdiv>
        <score>
          <scoreDef measureNumbers="true">
            <staffGrp>
              <staffDef n="1" lines="5" lines.visible="true" meter.count="2" meter.unit="4">
              </staffDef>
            </staffGrp>
          </scoreDef>
          <section xml:id="s1">
            <measure xml:id="m42j4hb" left="dbl" right="single" n="1">
              <mNum>1</mNum>
              <staff xml:id="m1s1" n="1">
                <layer xml:id="m1s1l1" n="1">
                  <note xml:id="n14c3kqh" dur="4" pname="e" oct="5" />
                  <note xml:id="n16dpotb" dur="4" pname="e" oct="5" />
                </layer>
              </staff>
            </measure>
            <measure xml:id="msb0m6f" right="dbl" left="single" n="2">
              <mNum></mNum>
              <staff xml:id="m2s1" n="1">
                <layer xml:id="m2s1l1" n="1">
                  <note xml:id="n5ve2xh" dur="4" pname="e" oct="5" />
                  <rest xml:id="r11z73rv" dur="4" />
                </layer>
              </staff>
            </measure>
            <measure xml:id="m1812i3x" left="dbl" right="single" n="3">
              <mNum>2</mNum>
              <staff xml:id="m3s1" n="1">
                <layer xml:id="m3s1l1" n="1">
                  <note xml:id="nq7gfuh" dur="4" pname="e" oct="5" />
                  <beam>
                    <note xml:id="n22547l" dur="8" pname="e" oct="5" />
                    <note xml:id="nm124p" dur="8" pname="e" oct="5" />
                  </beam>
                </layer>
              </staff>
            </measure>
            <measure xml:id="m1xeafja" right="dbl" left="single" n="4">
              <mNum></mNum>
              <staff xml:id="m4s1" n="1">
                <layer xml:id="m4s1l1" n="1">
                  <note xml:id="n1wm5mq1" dur="4" pname="e" oct="5" />
                  <rest xml:id="rz7u014" dur="4" />
                </layer>
              </staff>
            </measure>
            <measure xml:id="me0t7qy" left="dbl" right="single" n="5">
              <mNum>3</mNum>
              <staff xml:id="m5s1" n="1">
                <layer xml:id="m5s1l1" n="1">
                  <beam>
                    <note xml:id="n1ublw1k" dur="8" pname="e" oct="5" />
                    <note xml:id="n1psslru" dur="8" pname="e" oct="5" />
                  </beam>
                  <note xml:id="n1lxyy23" dur="4" pname="e" oct="5" />
                </layer>
              </staff>
            </measure>
            <measure xml:id="mxcs6v2" right="dbl" left="single" n="6">
              <mNum></mNum>
              <staff xml:id="m6s1" n="1">
                <layer xml:id="m6s1l1" n="1">
                  <note xml:id="n1rdrp60" dur="4" pname="e" oct="5" />
                  <rest xml:id="r8v92vr" dur="4" />
                </layer>
              </staff>
            </measure>
            <measure xml:id="mslkcwc" n="7" left="dbl" right="single" break="no">
              <mNum>4</mNum>
              <staff xml:id="m7s1" n="1">
                <layer xml:id="m7s1l1" n="1">
                  <beam>
                    <note xml:id="nobjg0x" dur="8" pname="e" oct="5" />
                    <note xml:id="n14ink8n" dur="8" pname="e" oct="5" />
                  </beam>
                  <beam>
                    <note xml:id="nueygfa" dur="8" pname="e" oct="5" />
                    <note xml:id="n1t0k81p" dur="8" pname="e" oct="5" />
                  </beam>
                </layer>
              </staff>
            </measure>
            <measure xml:id="mkj35u9" right="dbl" left="single" n="8">
              <mNum></mNum>
              <staff xml:id="m8s1" n="1">
                <layer xml:id="m8s1l1" n="1">
                  <note xml:id="n15lthwj" dur="8" pname="e" oct="5" />
                  <rest xml:id="r1sczkh9" type="mscore-beam-none" dur="8" />
                  <rest xml:id="r1d4kz3w" dur="4" />
                </layer>
              </staff>
            </measure>
            <measure xml:id="mec8j3f" left="dbl" right="single" n="9">
              <mNum>5</mNum>
              <staff xml:id="m9s1" n="1">
                <layer xml:id="m9s1l1" n="1">
                  <beam>
                    <note xml:id="nob5uxs" dur="8" pname="e" oct="5" />
                    <note xml:id="nc2g9j5" dur="8" pname="e" oct="5" />
                  </beam>
                  <beam>
                    <note xml:id="nfodwtc" dur="8" pname="e" oct="5" />
                    <note xml:id="n10330r0" dur="8" pname="e" oct="5" />
                  </beam>
                </layer>
              </staff>
            </measure>
            <measure xml:id="mle3gt9" right="dbl" left="single" n="10">
              <mNum></mNum>
              <staff xml:id="m10s1" n="1">
                <layer xml:id="m10s1l1" n="1">
                  <note xml:id="nq8e51o" dur="4" pname="e" oct="5" />
                  <note xml:id="n1bn585q" dur="8" pname="e" oct="5" />
                  <rest xml:id="r145bzbu" type="mscore-beam-none" dur="8" />
                </layer>
              </staff>
            </measure>
            <measure xml:id="m1bqezlj" left="dbl" right="single" n="11">
              <mNum>6</mNum>
              <staff xml:id="m11s1" n="1">
                <layer xml:id="m11s1l1" n="1">
                  <note xml:id="n1szw9pi" dur="4" pname="e" oct="5" />
                  <beam>
                    <note xml:id="nt084g3" dur="8" pname="e" oct="5" />
                    <note xml:id="nexz9xu" dur="8" pname="e" oct="5" />
                  </beam>
                </layer>
              </staff>
            </measure>
            <measure xml:id="m2jxael" right="dbl" left="single" n="12">
              <mNum></mNum>
              <staff xml:id="m12s1" n="1">
                <layer xml:id="m12s1l1" n="1">
                  <beam>
                    <note xml:id="ncy3ii7" dur="8" pname="e" oct="5" />
                    <note xml:id="nkdwpst" dur="8" pname="e" oct="5" />
                  </beam>
                  <note xml:id="n93pbp5" dur="8" pname="e" oct="5" />
                  <rest xml:id="r1g16zho" type="mscore-beam-none" dur="8" />
                </layer>
              </staff>
            </measure>
            <measure xml:id="mlnnt80" left="dbl" right="single" n="13">
              <mNum>7</mNum>
              <staff xml:id="m13s1" n="1">
                <layer xml:id="m13s1l1" n="1">
                  <beam>
                    <note xml:id="n1sianrb" dur="8" pname="e" oct="5" />
                    <note xml:id="n1donbbp" dur="8" pname="e" oct="5" />
                  </beam>
                  <note xml:id="n10h3nni" dur="4" pname="e" oct="5" />
                </layer>
              </staff>
            </measure>
            <measure xml:id="m1v0h2ps" right="dbl" left="single" n="14">
              <mNum></mNum>
              <staff xml:id="m14s1" n="1">
                <layer xml:id="m14s1l1" n="1">
                  <note xml:id="n7qklim" dur="4" pname="e" oct="5" />
                  <note xml:id="n10qhaxr" dur="8" pname="e" oct="5" />
                  <rest xml:id="rnqk535" type="mscore-beam-none" dur="8" />
                </layer>
              </staff>
            </measure>
            <measure xml:id="m2fg2lx" left="dbl" right="single" n="15">
              <mNum>8</mNum>
              <staff xml:id="m15s1" n="1">
                <layer xml:id="m15s1l1" n="1">
                  <beam>
                    <note xml:id="n18ao39a" dur="8" pname="e" oct="5" />
                    <note xml:id="n1m9n3tn" dur="8" pname="e" oct="5" />
                  </beam>
                  <beam>
                    <note xml:id="n41h2vo" dur="8" pname="e" oct="5" />
                    <note xml:id="njvu1t2" dur="8" pname="e" oct="5" />
                  </beam>
                </layer>
              </staff>
            </measure>
            <measure xml:id="m2ssg4h" right="dbl" left="single" n="16">
              <mNum></mNum>
              <staff xml:id="m16s1" n="1">
                <layer xml:id="m16s1l1" n="1">
                  <beam>
                    <note xml:id="n1m91jix" dur="8" pname="e" oct="5" />
                    <note xml:id="n1doergj" dur="8" pname="e" oct="5" />
                  </beam>
                  <note xml:id="n5yttd6" dur="8" pname="e" oct="5" />
                  <rest xml:id="rroh03i" type="mscore-beam-none" dur="8" />
                </layer>
              </staff>
            </measure>
            <measure xml:id="m1moresd" left="dbl" right="single" n="17">
              <mNum>9</mNum>
              <staff xml:id="m17s1" n="1">
                <layer xml:id="m17s1l1" n="1">
                  <note xml:id="nddb11i" dur="4" pname="e" oct="5" />
                  <beam>
                    <note xml:id="n1qhzii" dur="8" pname="e" oct="5" />
                    <note xml:id="nv95qh7" dur="8" pname="e" oct="5" />
                  </beam>
                </layer>
              </staff>
            </measure>
            <measure xml:id="mh9gvwa" right="dbl" left="single" n="18">
              <mNum></mNum>
              <staff xml:id="m18s1" n="1">
                <layer xml:id="m18s1l1" n="1">
                  <note xml:id="n601ehm" dur="4" pname="e" oct="5" />
                  <note xml:id="n8glv24" dur="8" pname="e" oct="5" />
                  <rest xml:id="r1t67s13" type="mscore-beam-none" dur="8" />
                </layer>
              </staff>
            </measure>
            <measure xml:id="mqsmzrr" left="dbl" right="single" n="19">
              <mNum>10</mNum>
              <staff xml:id="m19s1" n="1">
                <layer xml:id="m19s1l1" n="1">
                  <note xml:id="n1l4er9p" dur="2" pname="e" oct="5" />
                </layer>
              </staff>
            </measure>
            <measure xml:id="m15xzjv5" right="dbl" left="single" n="20">
              <mNum></mNum>
              <staff xml:id="m20s1" n="1">
                <layer xml:id="m20s1l1" n="1">
                  <beam>
                    <note xml:id="nwp5f4h" dur="8" pname="e" oct="5" />
                    <note xml:id="n7gy9rg" dur="8" pname="e" oct="5" />
                  </beam>
                  <note xml:id="ntztwt9" dur="8" pname="e" oct="5" />
                  <rest xml:id="r1va9pl5" type="mscore-beam-none" dur="8" />
                </layer>
              </staff>
            </measure>
          </section>
        </score>
      </mdiv>
    </body>
  </music>
</mei>"###;

const MEI_SANGUE: &str = r###"<?xml version="1.0" encoding="UTF-8"?>
<?xml-model href="https://music-encoding.org/schema/5.0/mei-basic.rng" type="application/xml" schematypens="http://relaxng.org/ns/structure/1.0"?>
<?xml-model href="https://music-encoding.org/schema/5.0/mei-basic.rng" type="application/xml" schematypens="http://purl.oclc.org/dsdl/schematron"?>
<mei xmlns="http://www.music-encoding.org/ns/mei" meiversion="5.0+basic">
   <meiHead>
      <fileDesc>
         <titleStmt>
            <title type="main">O Sangue me Libertou</title>
            <title type="subordinate">Subtitle</title>
            <respStmt>
               <persName role="composer">Arranjo: Lucio Rodrigues</persName>
               <persName role="lyricist">Versão: Lucio Rodrigues</persName>
            </respStmt>
         </titleStmt>
         <pubStmt>
            <date isodate="2025-03-09T18:40:17" />
         </pubStmt>
      </fileDesc>
   </meiHead>
   <music>
      <body>
         <mdiv>
            <score>
               <scoreDef>
                  <pgHead>
                     <rend halign="center" valign="top">
                        <rend type="title" fontsize="x-large">O Sangue me Libertou</rend>
                        <lb />
                        <rend type="subtitle" fontsize="large">Quinteto do Robinson</rend>
                     </rend>
                     <rend halign="left" valign="bottom">
                        <rend type="poet">Versão: Lucio Rodrigues</rend>
                     </rend>
                     <rend halign="right" valign="bottom">
                        <rend type="composer">Arranjo: Lucio Rodrigues</rend>
                     </rend>
                  </pgHead>
                  <staffGrp>
                     <staffDef n="1" lines="5" keysig="3s" meter.count="9" meter.unit="8" trans.diat="-7" trans.semi="-12">
                        <label>Bass Guitar</label>
                        <labelAbbr>B. Guit.</labelAbbr>
                        <clef shape="F" line="4" />
                     </staffDef>
                  </staffGrp>
               </scoreDef>
               <section xml:id="s1">
                  <pb />
                  <measure xml:id="m1yah3j1" n="1">
                     <staff xml:id="m1s1" n="1">
                        <layer xml:id="m1s1l1" n="1">
                           <note xml:id="n1pq1afg" dots="1" dur="2" pname="a" oct="3">
                              <verse xml:id="v1axhxsx" n="1">
                                 <syl con="u">Uh</syl>
                              </verse>
                           </note>
                           <note xml:id="n1j65sx5" dots="1" dur="4" pname="e" oct="3" />
                        </layer>
                     </staff>
                     <tempo xml:id="to129ed" type="mscore-infer-from-text" staff="1" tstamp="1.000000" midi.bpm="130.000000">
                        <rend glyph.auth="smufl"></rend> = 130</tempo>
                     <reh xml:id="rl1yg41" startid="#n1pq1afg">A</reh>
                  </measure>
                  <measure xml:id="m1c6hrmq" n="2">
                     <staff xml:id="m2s1" n="1">
                        <layer xml:id="m2s1l1" n="1">
                           <note xml:id="nswqj3k" dur="2" pname="f" oct="3">
                              <accid accid.ges="s" />
                           </note>
                           <rest xml:id="rtfqtjd" dur="4" />
                           <note xml:id="nkt5ulp" dots="1" dur="4" pname="a" oct="3">
                              <verse xml:id="v1bp9ztu" n="1">
                                 <syl con="u">uh</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1so18z9" n="3">
                     <staff xml:id="m3s1" n="1">
                        <layer xml:id="m3s1l1" n="1">
                           <note xml:id="n1yxyvbb" dots="1" dur="2" pname="d" oct="2" />
                           <note xml:id="n89ji0h" dots="1" dur="4" pname="e" oct="2">
                              <verse xml:id="vo3mgo3" n="1">
                                 <syl con="u">uh</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1l2xgna" right="dbl" n="4">
                     <staff xml:id="m4s1" n="1">
                        <layer xml:id="m4s1l1" n="1">
                           <note xml:id="n6uuwoh" dots="1" dur="2" pname="a" oct="2" />
                           <note xml:id="n1tiywmh" dots="1" dur="4" pname="a" oct="2" />
                        </layer>
                     </staff>
                     <tie xml:id="t1kz1vl5" startid="#n6uuwoh" endid="#n1tiywmh" />
                  </measure>
                  <sb type="mscore-manual" />
                  <measure xml:id="m1281yuq" n="5">
                     <staff xml:id="m5s1" n="1">
                        <layer xml:id="m5s1l1" n="1">
                           <note xml:id="nthb1bt" dots="1" dur="2" pname="a" oct="2">
                              <verse xml:id="vnrooff" n="1">
                                 <syl>Uh</syl>
                              </verse>
                           </note>
                           <rest xml:id="reanmr7" dots="1" dur="4" />
                        </layer>
                     </staff>
                     <reh xml:id="r1c77hrp" startid="#nthb1bt">E1</reh>
                  </measure>
                  <measure xml:id="mzkiv0a" n="6">
                     <staff xml:id="m6s1" n="1">
                        <layer xml:id="m6s1l1" n="1">
                           <note xml:id="n4j1hoh" dots="1" dur="2" pname="e" oct="2">
                              <verse xml:id="v8z3gmn" n="1">
                                 <syl>uh</syl>
                              </verse>
                           </note>
                           <rest xml:id="r5fi1sl" dots="1" dur="4" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1p9fhmo" n="7">
                     <staff xml:id="m7s1" n="1">
                        <layer xml:id="m7s1l1" n="1">
                           <note xml:id="nx4oh8h" dur="4" pname="d" oct="2">
                              <verse xml:id="vywhacq" n="1">
                                 <syl>tsum</syl>
                              </verse>
                           </note>
                           <rest xml:id="r13dt2r5" type="mscore-beam-none" dur="8" />
                           <rest xml:id="ryyuh7r" dots="1" dur="4" />
                           <rest xml:id="rikjo8w" dur="4" />
                           <note xml:id="n17tmra2" dur="8" pname="e" oct="2">
                              <verse xml:id="vuy3rd1" n="1">
                                 <syl>tsu</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mh8gzct" n="8">
                     <staff xml:id="m8s1" n="1">
                        <layer xml:id="m8s1l1" n="1">
                           <note xml:id="nug5kn2" dur="4" pname="a" oct="2">
                              <verse xml:id="v1w3l64w" n="1">
                                 <syl>tsum</syl>
                              </verse>
                           </note>
                           <rest xml:id="r1tv9yi9" type="mscore-beam-none" dur="8" />
                           <note xml:id="nsnvhq0" dur="4" pname="b" oct="2">
                              <verse xml:id="v1kllh4v" n="1">
                                 <syl>tsum</syl>
                              </verse>
                           </note>
                           <rest xml:id="r1m72csf" type="mscore-beam-none" dur="8" />
                           <note xml:id="n1vdbdte" dur="4" pname="c" oct="3">
                              <verse xml:id="v97ffm7" n="1">
                                 <syl>tsum</syl>
                              </verse>
                              <accid xml:id="a15znhaz" accid="n" />
                           </note>
                           <rest xml:id="r8pul7j" type="mscore-beam-none" dur="8" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m784oix" n="9">
                     <staff xml:id="m9s1" n="1">
                        <layer xml:id="m9s1l1" n="1">
                           <note xml:id="ni9q610" dur="8" pname="c" oct="3">
                              <verse xml:id="v1ijzm31" n="1">
                                 <syl>tsum</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                           <rest xml:id="rpv33p8" type="mscore-beam-none" dur="8" />
                           <rest xml:id="rryikgf" type="mscore-beam-none" dur="8" />
                           <note xml:id="n1ex633v" dur="8" pname="c" oct="3">
                              <verse xml:id="v1lnrc56" n="1">
                                 <syl>tsum</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                           <rest xml:id="r1wnbolk" type="mscore-beam-none" dur="8" />
                           <rest xml:id="r1uchbdk" type="mscore-beam-none" dur="8" />
                           <note xml:id="n1fpqd9d" dur="8" pname="c" oct="3">
                              <verse xml:id="v1ib6em1" n="1">
                                 <syl>tusm</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                           <rest xml:id="rq8d2kt" type="mscore-beam-none" dur="8" />
                           <rest xml:id="rl1gzye" type="mscore-beam-none" dur="8" />
                        </layer>
                     </staff>
                  </measure>
                  <sb />
                  <measure xml:id="m1fkrky4" n="10">
                     <staff xml:id="m10s1" n="1">
                        <layer xml:id="m10s1l1" n="1">
                           <note xml:id="n12p92pl" dur="4" pname="c" oct="4">
                              <verse xml:id="v6qnztr" n="1">
                                 <syl con="u">E</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                           <note xml:id="n1ytlx1m" dur="8" pname="b" oct="3" />
                           <note xml:id="n1p7jiwg" dur="4" pname="g" oct="3">
                              <verse xml:id="v4f4kk3" n="1">
                                 <syl con="u">le</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                           <note xml:id="nv8q9vt" dur="8" pname="f" oct="3">
                              <accid accid.ges="s" />
                           </note>
                           <note xml:id="nbrzjdx" dur="4" pname="e" oct="3">
                              <verse xml:id="v14fcbhw" n="1">
                                 <syl con="u">mor</syl>
                              </verse>
                           </note>
                           <note xml:id="nqu5s62" dur="8" pname="c" oct="3">
                              <accid accid.ges="s" />
                           </note>
                        </layer>
                     </staff>
                     <slur xml:id="s13f7n1n" startid="#n12p92pl" endid="#n1ytlx1m" />
                     <slur xml:id="s1hfvzab" startid="#n1p7jiwg" endid="#nv8q9vt" />
                     <slur xml:id="sa5xdmk" startid="#nbrzjdx" endid="#nqu5s62" />
                  </measure>
                  <measure xml:id="m1emnhs" n="11">
                     <staff xml:id="m11s1" n="1">
                        <layer xml:id="m11s1l1" n="1">
                           <note xml:id="n19knz3x" dots="1" dur="4" pname="f" oct="3">
                              <verse xml:id="v1z0ky2v" n="1">
                                 <syl con="u">reu</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                           <note xml:id="nx7ld9t" dots="1" dur="4" pname="e" oct="3" />
                           <note xml:id="nffy2js" dots="1" dur="4" pname="c" oct="3">
                              <verse xml:id="vbeftow" n="1">
                                 <syl>por</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                        </layer>
                     </staff>
                     <slur xml:id="s1oygmnb" startid="#n19knz3x" endid="#nx7ld9t" />
                  </measure>
                  <measure xml:id="mno3u7l" n="12">
                     <staff xml:id="m12s1" n="1">
                        <layer xml:id="m12s1l1" n="1">
                           <note xml:id="n1u36e6w" dur="4" pname="b" oct="2">
                              <verse xml:id="v4icuix" n="1">
                                 <syl con="u">mim</syl>
                              </verse>
                           </note>
                           <note xml:id="n63000k" dur="8" pname="a" oct="2" />
                           <note xml:id="n19yz352" dur="4" pname="a" oct="2" />
                           <rest xml:id="r3vg95w" type="mscore-beam-none" dur="8" />
                           <note xml:id="n103mld5" dur="4" pname="a" oct="2">
                              <verse xml:id="vxrw0qc" n="1">
                                 <syl con="d" wordpos="i">e</syl>
                              </verse>
                           </note>
                           <note xml:id="n11yes84" dur="8" pname="f" oct="2">
                              <verse xml:id="vqwih9h" n="1">
                                 <syl con="d" wordpos="m">ra es</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                        </layer>
                     </staff>
                     <slur xml:id="stgvas4" startid="#n1u36e6w" endid="#n63000k" />
                     <tie xml:id="t8vq5a5" startid="#n63000k" endid="#n19yz352" />
                  </measure>
                  <measure xml:id="m15jfepg" n="13">
                     <staff xml:id="m13s1" n="1">
                        <layer xml:id="m13s1l1" n="1">
                           <note xml:id="nku3igm" dur="4" pname="d" oct="2">
                              <verse xml:id="vpg03h8" n="1">
                                 <syl con="d" wordpos="m">cra</syl>
                              </verse>
                           </note>
                           <note xml:id="n1x9b8ed" dur="8" pname="d" oct="2">
                              <verse xml:id="v1t5um2p" n="1">
                                 <syl con="u" wordpos="t">vo</syl>
                              </verse>
                           </note>
                           <note xml:id="n1tjof9p" dots="1" dur="4" pname="d" oct="2" />
                           <note xml:id="nr2jztu" dur="4" pname="a" oct="2">
                              <verse xml:id="vlnxy74" n="1">
                                 <syl>do</syl>
                              </verse>
                           </note>
                           <note xml:id="n7tnt6t" dur="8" pname="f" oct="2">
                              <verse xml:id="v1khlt8c" n="1">
                                 <syl con="d" wordpos="i">pe</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="t156yaj1" startid="#n1x9b8ed" endid="#n1tjof9p" />
                  </measure>
                  <measure xml:id="msmvx2r" n="14">
                     <staff xml:id="m14s1" n="1">
                        <layer xml:id="m14s1l1" n="1">
                           <note xml:id="nso32n2" dur="4" pname="d" oct="2">
                              <verse xml:id="v1b4edge" n="1">
                                 <syl con="d" wordpos="m">ca</syl>
                              </verse>
                           </note>
                           <note xml:id="n8k935x" dur="8" pname="d" oct="2">
                              <verse xml:id="v15to6ia" n="1">
                                 <syl con="u" wordpos="t">do</syl>
                              </verse>
                           </note>
                           <note xml:id="ncdtn9x" dots="1" dur="4" pname="d" oct="2" />
                           <note xml:id="n6y6et3" dur="4" pname="a" oct="2">
                              <verse xml:id="vlj0rnk" n="1">
                                 <syl>mas</syl>
                              </verse>
                           </note>
                           <note xml:id="n1n6etkz" dur="8" pname="f" oct="2">
                              <verse xml:id="vhkg08q" n="1">
                                 <syl con="d" wordpos="i">a</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="td9pqrb" startid="#n8k935x" endid="#ncdtn9x" />
                  </measure>
                  <sb />
                  <measure xml:id="m11er7rc" n="15">
                     <staff xml:id="m15s1" n="1">
                        <layer xml:id="m15s1l1" n="1">
                           <note xml:id="n18iup8q" dur="4" pname="a" oct="2">
                              <verse xml:id="vf9bofe" n="1">
                                 <syl con="d" wordpos="m">go</syl>
                              </verse>
                           </note>
                           <note xml:id="nu8sjtl" dur="8" pname="a" oct="2">
                              <verse xml:id="v1llqapa" n="1">
                                 <syl con="u" wordpos="t">ra</syl>
                              </verse>
                           </note>
                           <note xml:id="n1l6law9" dots="1" dur="4" pname="a" oct="2" />
                           <note xml:id="n14y9mr0" dots="1" dur="4" pname="e" oct="2">
                              <verse xml:id="vypaf31" n="1">
                                 <syl>sou</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="tysgic7" startid="#nu8sjtl" endid="#n1l6law9" />
                  </measure>
                  <measure xml:id="m1tgo6n9" n="16">
                     <staff xml:id="m16s1" n="1">
                        <layer xml:id="m16s1l1" n="1">
                           <note xml:id="na9etyf" dur="4" pname="f" oct="2">
                              <verse xml:id="v9kfc65" n="1">
                                 <syl con="d" wordpos="i">li</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                           <note xml:id="n16abhui" dur="8" pname="f" oct="2">
                              <verse xml:id="v1xsbkni" n="1">
                                 <syl con="u" wordpos="t">vre</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                           <note xml:id="n10oni6b" dots="1" dur="4" pname="f" oct="2">
                              <accid accid.ges="s" />
                           </note>
                           <note xml:id="n6zkr12" dots="1" dur="4" pname="c" oct="3">
                              <verse xml:id="voyqjt7" n="1">
                                 <syl>o</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="tjuf04r" startid="#n16abhui" endid="#n10oni6b" />
                  </measure>
                  <measure xml:id="m1s6hacw" n="17">
                     <staff xml:id="m17s1" n="1">
                        <layer xml:id="m17s1l1" n="1">
                           <note xml:id="nmlmp6r" dots="1" dur="4" pname="b" oct="2">
                              <verse xml:id="vjbdoue" n="1">
                                 <syl con="d" wordpos="i">san</syl>
                              </verse>
                           </note>
                           <note xml:id="nx76vpd" dots="1" dur="4" pname="b" oct="1">
                              <verse xml:id="vrmd98c" n="1">
                                 <syl wordpos="t">gue</syl>
                              </verse>
                           </note>
                           <note xml:id="n14sinwm" dots="1" dur="4" pname="b" oct="1">
                              <verse xml:id="vv889sl" n="1">
                                 <syl>me</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m183xnbg" n="18">
                     <staff xml:id="m18s1" n="1">
                        <layer xml:id="m18s1l1" n="1">
                           <note xml:id="n1mh8nxl" dots="1" dur="2" pname="e" oct="2">
                              <verse xml:id="v13h71j3" n="1">
                                 <syl>li</syl>
                              </verse>
                           </note>
                           <note xml:id="n1fq0ftj" dots="1" dur="4" pname="e" oct="2">
                              <verse xml:id="vykpqgk" n="1">
                                 <syl con="d" wordpos="m">ber</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1yy3x1g" n="19">
                     <staff xml:id="m19s1" n="1">
                        <layer xml:id="m19s1l1" n="1">
                           <note xml:id="n9feped" dots="1" dur="2" pname="a" oct="2">
                              <verse xml:id="v1pyar5v" n="1">
                                 <syl con="u" wordpos="t">tou</syl>
                              </verse>
                           </note>
                           <note xml:id="npu1ct7" dots="1" dur="4" pname="a" oct="2" />
                        </layer>
                     </staff>
                     <tie xml:id="te4x15x" startid="#n9feped" endid="#npu1ct7" />
                  </measure>
                  <measure xml:id="mqrxe89" right="dbl" n="20">
                     <staff xml:id="m20s1" n="1">
                        <layer xml:id="m20s1l1" n="1">
                           <note xml:id="nai5zpy" dots="1" dur="4" pname="a" oct="2">
                              <verse xml:id="vha92dq" n="1">
                                 <syl>pm</syl>
                              </verse>
                           </note>
                           <note xml:id="npylkr" dots="1" dur="4" pname="g" oct="2">
                              <verse xml:id="v6blxgx" n="1">
                                 <syl>pm</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                           <note xml:id="nl84glk" dots="1" dur="4" pname="f" oct="2">
                              <verse xml:id="v15bt22i" n="1">
                                 <syl>pm.</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <sb />
                  <scoreDef keysig="2f" />
                  <measure xml:id="m5xon8b" n="21">
                     <staff xml:id="m21s1" n="1">
                        <layer xml:id="m21s1l1" n="1">
                           <note xml:id="n1biairy" dots="1" dur="2" pname="f" oct="2">
                              <verse xml:id="v1lubx53" n="1">
                                 <syl>Pm</syl>
                              </verse>
                           </note>
                           <rest xml:id="rs496k6" dur="4" />
                           <note xml:id="n1jry04c" dur="8" pname="c" oct="3">
                              <verse xml:id="vmzs8vi" n="1">
                                 <syl>pm</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                     <reh xml:id="rul3rch" startid="#n1biairy">C</reh>
                  </measure>
                  <measure xml:id="m12ihwes" n="22">
                     <staff xml:id="m22s1" n="1">
                        <layer xml:id="m22s1l1" n="1">
                           <note xml:id="nb8r7ze" dots="1" dur="2" pname="f" oct="2">
                              <verse xml:id="v1kh3pzm" n="1">
                                 <syl>pm</syl>
                              </verse>
                           </note>
                           <rest xml:id="rrzxlzv" dur="4" />
                           <note xml:id="n1dg4x44" dur="8" pname="f" oct="2">
                              <verse xml:id="v10j61lh" n="1">
                                 <syl>pm</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="me8jhqi" n="23">
                     <staff xml:id="m23s1" n="1">
                        <layer xml:id="m23s1l1" n="1">
                           <note xml:id="nfkpvi" dots="1" dur="2" pname="b" oct="2">
                              <verse xml:id="v1li09d6" n="1">
                                 <syl>pm</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <rest xml:id="rlzss2d" dur="4" />
                           <note xml:id="n1qw14f" dur="8" pname="f" oct="2">
                              <verse xml:id="vpoec1h" n="1">
                                 <syl>pm</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m3lmclk" n="24">
                     <staff xml:id="m24s1" n="1">
                        <layer xml:id="m24s1l1" n="1">
                           <note xml:id="n6ao2cd" dots="1" dur="4" pname="b" oct="2">
                              <verse xml:id="v1jaxdqq" n="1">
                                 <syl>pm</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="n1084y4z" dots="1" dur="4" pname="a" oct="2">
                              <verse xml:id="veg2f0j" n="1">
                                 <syl>pm</syl>
                              </verse>
                           </note>
                           <note xml:id="n1vpjebu" dots="1" dur="4" pname="g" oct="2">
                              <verse xml:id="v1fpfj0w" n="1">
                                 <syl>pm</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mpc3bi6" n="25">
                     <staff xml:id="m25s1" n="1">
                        <layer xml:id="m25s1l1" n="1">
                           <note xml:id="n1r6pzim" dots="1" dur="4" pname="f" oct="3">
                              <verse xml:id="v1odu7cg" n="1">
                                 <syl con="d" wordpos="i">gra</syl>
                              </verse>
                           </note>
                           <note xml:id="n1ew7ksx" dots="1" dur="4" pname="f" oct="3">
                              <verse xml:id="v7tqvbb" n="1">
                                 <syl wordpos="t">ça</syl>
                              </verse>
                           </note>
                           <rest xml:id="r7kqk26" dots="1" dur="4" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mjgq211" n="26">
                     <staff xml:id="m26s1" n="1">
                        <layer xml:id="m26s1l1" n="1">
                           <note xml:id="nwc61su" dots="1" dur="4" pname="f" oct="3">
                              <verse xml:id="v63chox" n="1">
                                 <syl>me</syl>
                              </verse>
                           </note>
                           <note xml:id="nmevo7e" dots="1" dur="4" pname="f" oct="3">
                              <verse xml:id="v1dunjnj" n="1">
                                 <syl con="d" wordpos="i">con</syl>
                              </verse>
                           </note>
                           <note xml:id="n1t0zjil" dots="1" dur="4" pname="f" oct="3">
                              <verse xml:id="vrvyrh4" n="1">
                                 <syl con="d" wordpos="m">ce</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mh8gjqj" n="27">
                     <staff xml:id="m27s1" n="1">
                        <layer xml:id="m27s1l1" n="1">
                           <note xml:id="n1j85qe1" dots="1" dur="2" pname="b" oct="2">
                              <verse xml:id="v5kv55j" n="1">
                                 <syl con="u" wordpos="t">deu</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="n4gvshg" dots="1" dur="4" pname="b" oct="2">
                              <accid accid.ges="f" />
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="t1ub8hy5" startid="#n1j85qe1" endid="#n4gvshg" />
                     <tie xml:id="t1x5rbov" startid="#n4gvshg" endid="#nogefhl" />
                  </measure>
                  <sb />
                  <measure xml:id="mrylr1z" n="28">
                     <staff xml:id="m28s1" n="1">
                        <layer xml:id="m28s1l1" n="1">
                           <note xml:id="nogefhl" dots="1" dur="2" pname="b" oct="2">
                              <accid accid.ges="f" />
                           </note>
                           <rest xml:id="r18wkqhm" dots="1" dur="4" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m86p49c" n="29">
                     <staff xml:id="m29s1" n="1">
                        <layer xml:id="m29s1l1" n="1">
                           <note xml:id="n1odcool" dots="1" dur="2" pname="e" oct="2">
                              <verse xml:id="v60xfnp" n="1">
                                 <syl con="u">Uh</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="nr923sh" dots="1" dur="4" pname="e" oct="2">
                              <accid accid.ges="f" />
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="ty7gmna" startid="#n1odcool" endid="#nr923sh" />
                  </measure>
                  <measure xml:id="m1v3tnih" n="30">
                     <staff xml:id="m30s1" n="1">
                        <layer xml:id="m30s1l1" n="1">
                           <note xml:id="nvxn6wv" dots="1" dur="2" pname="c" oct="2" />
                           <note xml:id="n1hp6p1d" dots="1" dur="4" pname="c" oct="2" />
                        </layer>
                     </staff>
                     <tie xml:id="tcsz320" startid="#nvxn6wv" endid="#n1hp6p1d" />
                  </measure>
                  <measure xml:id="m7olec1" n="31">
                     <staff xml:id="m31s1" n="1">
                        <layer xml:id="m31s1l1" n="1">
                           <note xml:id="n177d52r" dots="1" dur="2" pname="b" oct="1">
                              <verse xml:id="vge2b8w" n="1">
                                 <syl con="u">uh</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="n2imez6" dots="1" dur="4" pname="b" oct="1">
                              <accid accid.ges="f" />
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="twzg6bf" startid="#n177d52r" endid="#n2imez6" />
                     <slur xml:id="smzilmw" startid="#n2imez6" endid="#na5kyem" />
                  </measure>
                  <measure xml:id="m1orvjmv" n="32">
                     <staff xml:id="m32s1" n="1">
                        <layer xml:id="m32s1l1" n="1">
                           <note xml:id="na5kyem" dots="1" dur="2" pname="g" oct="2" />
                           <note xml:id="n1m8vwhv" dots="1" dur="4" pname="d" oct="3">
                              <verse xml:id="v19jcmcm" n="1">
                                 <syl>o</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="maxgex7" n="33">
                     <staff xml:id="m33s1" n="1">
                        <layer xml:id="m33s1l1" n="1">
                           <note xml:id="nuy6k8" dots="1" dur="4" pname="c" oct="3">
                              <verse xml:id="v1s5vlxy" n="1">
                                 <syl con="d" wordpos="i">san</syl>
                              </verse>
                           </note>
                           <note xml:id="n7jqaja" dots="1" dur="4" pname="c" oct="2">
                              <verse xml:id="v1uibgza" n="1">
                                 <syl wordpos="t">gue</syl>
                              </verse>
                           </note>
                           <note xml:id="n1as2yke" dots="1" dur="4" pname="c" oct="2">
                              <verse xml:id="vahg0vd" n="1">
                                 <syl>me</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mou07ow" n="34">
                     <staff xml:id="m34s1" n="1">
                        <layer xml:id="m34s1l1" n="1">
                           <note xml:id="nkhvu6d" dots="1" dur="2" pname="f" oct="2">
                              <verse xml:id="vwo2a4q" n="1">
                                 <syl con="d" wordpos="i">li</syl>
                              </verse>
                           </note>
                           <note xml:id="n1fb1saf" dots="1" dur="4" pname="f" oct="2">
                              <verse xml:id="vefei47" n="1">
                                 <syl wordpos="t">ber</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1l1irzw" n="35">
                     <staff xml:id="m35s1" n="1">
                        <layer xml:id="m35s1l1" n="1">
                           <note xml:id="n1j70jjf" dots="1" dur="2" pname="b" oct="2">
                              <verse xml:id="v1jw215z" n="1">
                                 <syl con="u">tou</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="n1e8tirv" dots="1" dur="4" pname="b" oct="2">
                              <accid accid.ges="f" />
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="t1cr32um" startid="#n1j70jjf" endid="#n1e8tirv" />
                  </measure>
                  <measure xml:id="m1jwoapx" right="dbl" n="36">
                     <staff xml:id="m36s1" n="1">
                        <layer xml:id="m36s1l1" n="1">
                           <note xml:id="n1ec3kf5" dots="1" dur="4" pname="f" oct="2">
                              <verse xml:id="vygi7a7" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                           <note xml:id="n13ryj7t" dots="1" dur="4" pname="g" oct="2">
                              <verse xml:id="vkb032i" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                           <note xml:id="n1dgpfsc" dots="1" dur="4" pname="a" oct="2">
                              <verse xml:id="v1tvdggy" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <sb />
                  <measure xml:id="m1072uyl" n="37">
                     <staff xml:id="m37s1" n="1">
                        <layer xml:id="m37s1l1" n="1">
                           <note xml:id="n18xf64q" dots="1" dur="4" pname="b" oct="2">
                              <verse xml:id="v1bmn79w" n="1">
                                 <syl>Tm</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="n1djyxgi" dots="1" dur="4" pname="d" oct="3">
                              <verse xml:id="vpeqvsn" n="1">
                                 <syl>tm</syl>
                              </verse>
                           </note>
                           <note xml:id="n1oc8is4" dots="1" dur="4" pname="b" oct="2">
                              <verse xml:id="vifccj7" n="1">
                                 <syl>tm</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                        </layer>
                     </staff>
                     <reh xml:id="rppffjz" startid="#n18xf64q">D</reh>
                  </measure>
                  <measure xml:id="mcdqgub" n="38">
                     <staff xml:id="m38s1" n="1">
                        <layer xml:id="m38s1l1" n="1">
                           <note xml:id="n25xpw0" dots="1" dur="4" pname="a" oct="2">
                              <verse xml:id="v7l3mc2" n="1">
                                 <syl>tm</syl>
                              </verse>
                           </note>
                           <note xml:id="n19bp4hw" dots="1" dur="4" pname="d" oct="3">
                              <verse xml:id="v14fym2z" n="1">
                                 <syl>tm</syl>
                              </verse>
                           </note>
                           <note xml:id="npcdvto" dots="1" dur="4" pname="a" oct="2">
                              <verse xml:id="vvnj2xs" n="1">
                                 <syl>tm</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1exsji" n="39">
                     <staff xml:id="m39s1" n="1">
                        <layer xml:id="m39s1l1" n="1">
                           <note xml:id="n1l9dfu0" dots="1" dur="4" pname="g" oct="2">
                              <verse xml:id="v1lz95pp" n="1">
                                 <syl>tm</syl>
                              </verse>
                           </note>
                           <note xml:id="nsyzuc" dots="1" dur="4" pname="a" oct="2">
                              <verse xml:id="v1wa829p" n="1">
                                 <syl>tm</syl>
                              </verse>
                           </note>
                           <note xml:id="njoldvh" dots="1" dur="4" pname="b" oct="2">
                              <verse xml:id="vyjoipz" n="1">
                                 <syl>tm</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1hykmlc" n="40">
                     <staff xml:id="m40s1" n="1">
                        <layer xml:id="m40s1l1" n="1">
                           <note xml:id="n1djnew2" dots="1" dur="4" pname="d" oct="3">
                              <verse xml:id="v1hebbhy" n="1">
                                 <syl>tm</syl>
                              </verse>
                           </note>
                           <note xml:id="n2bb5y3" dots="1" dur="4" pname="c" oct="3">
                              <verse xml:id="v14r64ua" n="1">
                                 <syl>tm</syl>
                              </verse>
                           </note>
                           <note xml:id="nby7q25" dots="1" dur="4" pname="b" oct="2">
                              <verse xml:id="v1irco01" n="1">
                                 <syl>tm</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m19r9gp8" n="41">
                     <staff xml:id="m41s1" n="1">
                        <layer xml:id="m41s1l1" n="1">
                           <note xml:id="nkz9xsd" dots="1" dur="2" pname="a" oct="2">
                              <verse xml:id="vmbjffg" n="1">
                                 <syl con="u">tm</syl>
                              </verse>
                           </note>
                           <note xml:id="nms6x7u" dots="1" dur="4" pname="a" oct="2" />
                        </layer>
                     </staff>
                     <tie xml:id="tp3ncir" startid="#nkz9xsd" endid="#nms6x7u" />
                  </measure>
                  <measure xml:id="m5wtxb1" n="42">
                     <staff xml:id="m42s1" n="1">
                        <layer xml:id="m42s1l1" n="1">
                           <note xml:id="n10x18dq" dots="1" dur="4" pname="d" oct="2">
                              <verse xml:id="vijv2q3" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                           <note xml:id="n1r56wwy" dots="1" dur="4" pname="e" oct="2">
                              <verse xml:id="vgr8fdm" n="1">
                                 <syl>tum</syl>
                              </verse>
                              <accid xml:id="a19udykr" accid="n" />
                           </note>
                           <note xml:id="n3vggt7" dots="1" dur="4" pname="g" oct="2">
                              <verse xml:id="vbeubwk" n="1">
                                 <syl>tum</syl>
                              </verse>
                              <accid xml:id="avno8ma" accid="f" />
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mtfezrj" n="43">
                     <staff xml:id="m43s1" n="1">
                        <layer xml:id="m43s1l1" n="1">
                           <note xml:id="n18n733" dots="1" dur="2" pname="g" oct="2">
                              <verse xml:id="v1bf0q2b" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                           <note xml:id="neqdu8m" dots="1" dur="4" pname="g" oct="2">
                              <verse xml:id="v10o6m1e" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <sb />
                  <measure xml:id="mvjpwsb" n="44">
                     <staff xml:id="m44s1" n="1">
                        <layer xml:id="m44s1l1" n="1">
                           <note xml:id="n1et2zrg" dots="1" dur="4" pname="f" oct="2">
                              <verse xml:id="v1mqvhh3" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                           <note xml:id="nlqwtl0" dur="4" pname="f" oct="2" />
                           <note xml:id="n1q8wlgn" dur="8" pname="b" oct="2">
                              <verse xml:id="vj1u35w" n="1">
                                 <syl con="d" wordpos="i">Tu</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="n1v3bxym" dur="4" pname="g" oct="2">
                              <verse xml:id="v1ojd0ah" n="1">
                                 <syl wordpos="t">u</syl>
                              </verse>
                           </note>
                           <note xml:id="nupadvc" dur="8" pname="f" oct="2">
                              <verse xml:id="vd6bgpn" n="1">
                                 <syl con="d" wordpos="i">du</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="t17kqiw5" startid="#n1et2zrg" endid="#nlqwtl0" />
                     <slur xml:id="skcfcdp" startid="#n1q8wlgn" endid="#n1v3bxym" />
                  </measure>
                  <measure xml:id="mqv1v9k" n="45">
                     <staff xml:id="m45s1" n="1">
                        <layer xml:id="m45s1l1" n="1">
                           <note xml:id="n17tt0w3" dur="4" pname="e" oct="2">
                              <verse xml:id="v1kzqvr5" n="1">
                                 <syl wordpos="t">dum</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <rest xml:id="r1v3rdf8" type="mscore-beam-none" dur="8" />
                           <rest xml:id="r1mkv3yd" dur="4" />
                           <note xml:id="n1q0o3uz" dur="8" pname="c" oct="3">
                              <verse xml:id="vzy7jtt" n="1">
                                 <syl con="u">tu</syl>
                              </verse>
                           </note>
                           <note xml:id="ns1eb1y" dur="4" pname="b" oct="2">
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="n1drn54v" dur="8" pname="c" oct="3">
                              <verse xml:id="v1owl4ri" n="1">
                                 <syl>du</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                     <slur xml:id="sxpc55p" startid="#n1q0o3uz" endid="#ns1eb1y" />
                  </measure>
                  <measure xml:id="m18gm6uo" n="46">
                     <staff xml:id="m46s1" n="1">
                        <layer xml:id="m46s1l1" n="1">
                           <note xml:id="ncccfio" dots="1" dur="4" pname="e" oct="2">
                              <verse xml:id="vkp2r26" n="1">
                                 <syl>dum</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <rest xml:id="ryc8mdx" dur="4" />
                           <note xml:id="nfdn39b" dur="8" pname="e" oct="2">
                              <verse xml:id="v1bgfynh" n="1">
                                 <syl con="u">tu</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="n13wi7yz" dur="4" pname="f" oct="2" />
                           <note xml:id="nc0v4i6" dur="8" pname="g" oct="2">
                              <verse xml:id="v1or8i5g" n="1">
                                 <syl>du</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                     <slur xml:id="s3o3991" startid="#nfdn39b" endid="#n13wi7yz" />
                  </measure>
                  <measure xml:id="mjom6z8" n="47">
                     <staff xml:id="m47s1" n="1">
                        <layer xml:id="m47s1l1" n="1">
                           <note xml:id="n12ayqw4" dots="1" dur="2" pname="b" oct="2">
                              <verse xml:id="va1o49k" n="1">
                                 <syl con="u">du</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <beam>
                              <note xml:id="n1oc04rq" dur="8" pname="c" oct="3">
                                 <verse xml:id="v1wb1ftj" n="1">
                                    <syl>fui</syl>
                                 </verse>
                              </note>
                              <note xml:id="n507eh2" dur="8" pname="b" oct="2">
                                 <verse xml:id="v1ilrvjr" n="1">
                                    <syl con="d" wordpos="i">re</syl>
                                 </verse>
                                 <accid accid.ges="f" />
                              </note>
                              <note xml:id="nnfkcp4" dur="8" pname="a" oct="2" />
                           </beam>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1kd0ere" n="48">
                     <staff xml:id="m48s1" n="1">
                        <layer xml:id="m48s1l1" n="1">
                           <note xml:id="n1t0za0n" dur="4" pname="g" oct="2">
                              <verse xml:id="v12bqmph" n="1">
                                 <syl wordpos="t">mi</syl>
                              </verse>
                           </note>
                           <note xml:id="n7s583z" dur="8" pname="g" oct="2" />
                           <note xml:id="n1wdjozn" dots="1" dur="4" pname="g" oct="2" />
                           <note xml:id="n1qzntxo" dots="1" dur="4" pname="d" oct="3">
                              <verse xml:id="vg0vcf9" n="1">
                                 <syl>o</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="tndk8a6" startid="#n7s583z" endid="#n1wdjozn" />
                  </measure>
                  <sb />
                  <measure xml:id="m8rkgln" n="49">
                     <staff xml:id="m49s1" n="1">
                        <layer xml:id="m49s1l1" n="1">
                           <note xml:id="nnfxl2x" dots="1" dur="4" pname="c" oct="3">
                              <verse xml:id="vvo5v72" n="1">
                                 <syl con="d" wordpos="i">san</syl>
                              </verse>
                           </note>
                           <note xml:id="n12v8zj7" dots="1" dur="4" pname="c" oct="2">
                              <verse xml:id="vjmlm5i" n="1">
                                 <syl wordpos="t">gue</syl>
                              </verse>
                           </note>
                           <note xml:id="nrfteou" dots="1" dur="4" pname="c" oct="2">
                              <verse xml:id="v9oly7e" n="1">
                                 <syl>me</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="my1vfmc" n="50">
                     <staff xml:id="m50s1" n="1">
                        <layer xml:id="m50s1l1" n="1">
                           <note xml:id="n17pvyoj" dots="1" dur="2" pname="f" oct="2">
                              <verse xml:id="v1wylbqn" n="1">
                                 <syl con="d" wordpos="i">li</syl>
                              </verse>
                           </note>
                           <note xml:id="n1fj9dhe" dots="1" dur="4" pname="f" oct="2">
                              <verse xml:id="v1uxbd0d" n="1">
                                 <syl con="d" wordpos="m">ber</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mfiqag0" n="51">
                     <staff xml:id="m51s1" n="1">
                        <layer xml:id="m51s1l1" n="1">
                           <note xml:id="n1uvfymb" dots="1" dur="2" pname="b" oct="2">
                              <verse xml:id="vbo11ev" n="1">
                                 <syl con="u" wordpos="t">tou</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <rest xml:id="rib5zul" dur="4" />
                           <rest xml:id="rd5dnle" type="mscore-beam-none" dur="8" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1vlztb9" right="dbl" n="52">
                     <staff xml:id="m52s1" n="1">
                        <layer xml:id="m52s1l1" n="1">
                           <note xml:id="n10a3bj8" dur="4" pname="g" oct="2">
                              <verse xml:id="v1q73lac" n="1">
                                 <syl con="d" wordpos="i">Uo</syl>
                              </verse>
                           </note>
                           <note xml:id="n9um24a" dur="8" pname="g" oct="2">
                              <verse xml:id="vpi7mis" n="1">
                                 <syl con="u" wordpos="t">ou!</syl>
                              </verse>
                           </note>
                           <note xml:id="nprfvm9" dur="4" pname="g" oct="2" />
                           <rest xml:id="ro8gn4n" type="mscore-beam-none" dur="8" />
                           <rest xml:id="rfhbgws" dots="1" dur="4" />
                        </layer>
                     </staff>
                     <tie xml:id="t4vakho" startid="#n9um24a" endid="#nprfvm9" />
                  </measure>
                  <measure xml:id="m1w0o18y" n="53">
                     <staff xml:id="m53s1" n="1">
                        <layer xml:id="m53s1l1" n="1">
                           <note xml:id="ncmfw5r" dots="1" dur="2" pname="f" oct="2">
                              <verse xml:id="vqr8lla" n="1">
                                 <syl>Tum</syl>
                              </verse>
                           </note>
                           <rest xml:id="r1qpo32" dur="4" />
                           <note xml:id="n46g3r1" dur="8" pname="c" oct="3">
                              <verse xml:id="v144r9qe" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1vp3kys" n="54">
                     <staff xml:id="m54s1" n="1">
                        <layer xml:id="m54s1l1" n="1">
                           <note xml:id="n1g1misx" dots="1" dur="2" pname="f" oct="2">
                              <verse xml:id="vyyl45j" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                           <rest xml:id="radj2r6" dur="4" />
                           <note xml:id="n1selw7j" dur="8" pname="f" oct="2">
                              <verse xml:id="vg7rz1n" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m7dwlvs" n="55">
                     <staff xml:id="m55s1" n="1">
                        <layer xml:id="m55s1l1" n="1">
                           <note xml:id="nexsosm" dots="1" dur="2" pname="b" oct="2">
                              <verse xml:id="v1ing4p2" n="1">
                                 <syl>tum</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <rest xml:id="rp2oq1s" dur="4" />
                           <note xml:id="n1lnuoyz" dur="8" pname="f" oct="2">
                              <verse xml:id="v2reixb" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <pb />
                  <measure xml:id="m8rl2i3" n="56">
                     <staff xml:id="m56s1" n="1">
                        <layer xml:id="m56s1l1" n="1">
                           <note xml:id="nt9gsh0" dots="1" dur="4" pname="b" oct="2">
                              <verse xml:id="v1eb4q5a" n="1">
                                 <syl>tum</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="np66x56" dots="1" dur="4" pname="a" oct="2">
                              <verse xml:id="v1qp7s65" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                           <note xml:id="n6xziif" dots="1" dur="4" pname="g" oct="2">
                              <verse xml:id="vpp1c72" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1q6jk0a" n="57">
                     <staff xml:id="m57s1" n="1">
                        <layer xml:id="m57s1l1" n="1">
                           <note xml:id="n9xld5e" dots="1" dur="4" pname="f" oct="3">
                              <verse xml:id="vk64od9" n="1">
                                 <syl con="d" wordpos="i">gra</syl>
                              </verse>
                           </note>
                           <note xml:id="n1n9hbys" dots="1" dur="4" pname="f" oct="3">
                              <verse xml:id="v1qpicj9" n="1">
                                 <syl wordpos="t">ça</syl>
                              </verse>
                           </note>
                           <rest xml:id="r1ye0lib" dots="1" dur="4" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mzqxoi2" n="58">
                     <staff xml:id="m58s1" n="1">
                        <layer xml:id="m58s1l1" n="1">
                           <note xml:id="n8wv5ub" dur="4" pname="f" oct="3">
                              <verse xml:id="v1r609sb" n="1">
                                 <syl>me</syl>
                              </verse>
                           </note>
                           <rest xml:id="r2020gc" type="mscore-beam-none" dur="8" />
                           <note xml:id="nrnat6t" dur="4" pname="f" oct="3">
                              <verse xml:id="v8om835" n="1">
                                 <syl con="d" wordpos="i">con</syl>
                              </verse>
                           </note>
                           <rest xml:id="r1gviz9s" type="mscore-beam-none" dur="8" />
                           <note xml:id="n1994k2j" dur="4" pname="f" oct="3">
                              <verse xml:id="v18yjifh" n="1">
                                 <syl wordpos="t">ce</syl>
                              </verse>
                           </note>
                           <rest xml:id="rm3lyne" type="mscore-beam-none" dur="8" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1ypjpj" n="59">
                     <staff xml:id="m59s1" n="1">
                        <layer xml:id="m59s1l1" n="1">
                           <note xml:id="nvwb6w1" dots="1" dur="2" pname="b" oct="2">
                              <verse xml:id="vhxwxdw" n="1">
                                 <syl con="u">deu</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="n1so0102" dots="1" dur="4" pname="b" oct="2">
                              <accid accid.ges="f" />
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="t1g3n5he" startid="#nvwb6w1" endid="#n1so0102" />
                     <tie xml:id="t11aadjw" startid="#n1so0102" endid="#nrhlj9v" />
                  </measure>
                  <measure xml:id="mfepqjm" n="60">
                     <staff xml:id="m60s1" n="1">
                        <layer xml:id="m60s1l1" n="1">
                           <note xml:id="nrhlj9v" dots="1" dur="2" pname="b" oct="2">
                              <accid accid.ges="f" />
                           </note>
                           <rest xml:id="r8e3hrs" dots="1" dur="4" />
                        </layer>
                     </staff>
                  </measure>
                  <sb type="mscore-manual" />
                  <measure xml:id="mfumvhz" left="rptstart" n="61">
                     <staff xml:id="m61s1" n="1">
                        <layer xml:id="m61s1l1" n="1">
                           <note xml:id="nduzyob" dots="1" dur="2" pname="e" oct="2">
                              <verse xml:id="v64nsvo" n="1">
                                 <syl con="u">Uh</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="n1gz9q9u" dots="1" dur="4" pname="e" oct="2">
                              <accid accid.ges="f" />
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="tqtkywk" startid="#nduzyob" endid="#n1gz9q9u" />
                  </measure>
                  <measure xml:id="m8ri2p" n="62">
                     <staff xml:id="m62s1" n="1">
                        <layer xml:id="m62s1l1" n="1">
                           <note xml:id="nz9lduo" dots="1" dur="2" pname="c" oct="2" />
                           <note xml:id="n1ae82ob" dots="1" dur="4" pname="c" oct="2" />
                        </layer>
                     </staff>
                     <tie xml:id="tvjqh83" startid="#nz9lduo" endid="#n1ae82ob" />
                  </measure>
                  <measure xml:id="m1ujty54" n="63">
                     <staff xml:id="m63s1" n="1">
                        <layer xml:id="m63s1l1" n="1">
                           <note xml:id="n18e6jkp" dots="1" dur="2" pname="b" oct="1">
                              <verse xml:id="v1tt1jaz" n="1">
                                 <syl con="u">uh</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="nzxvvuv" dots="1" dur="4" pname="b" oct="1">
                              <accid accid.ges="f" />
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="tq8ny22" startid="#n18e6jkp" endid="#nzxvvuv" />
                  </measure>
                  <measure xml:id="mbcxj3r" n="64">
                     <staff xml:id="m64s1" n="1">
                        <layer xml:id="m64s1l1" n="1">
                           <note xml:id="n1e4tm4n" dots="1" dur="2" pname="g" oct="2" />
                           <note xml:id="nbgaxjb" dots="1" dur="4" pname="d" oct="3">
                              <verse xml:id="v12w007l" n="1">
                                 <syl>o</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mi51tp9" n="65">
                     <staff xml:id="m65s1" n="1">
                        <layer xml:id="m65s1l1" n="1">
                           <note xml:id="nxw7hc8" dots="1" dur="4" pname="c" oct="3">
                              <verse xml:id="vcfof90" n="1">
                                 <syl con="d" wordpos="i">san</syl>
                              </verse>
                           </note>
                           <note xml:id="n10v57fx" dots="1" dur="4" pname="c" oct="2">
                              <verse xml:id="vrurl0c" n="1">
                                 <syl wordpos="t">gue</syl>
                              </verse>
                           </note>
                           <note xml:id="nj8o5fu" dots="1" dur="4" pname="c" oct="2">
                              <verse xml:id="vlvn2j9" n="1">
                                 <syl>me</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m13yedw8" n="66">
                     <staff xml:id="m66s1" n="1">
                        <layer xml:id="m66s1l1" n="1">
                           <note xml:id="n1dn1xqp" dots="1" dur="2" pname="f" oct="2">
                              <verse xml:id="v5mzqa4" n="1">
                                 <syl con="d" wordpos="i">li</syl>
                              </verse>
                           </note>
                           <note xml:id="n1l19le0" dots="1" dur="4" pname="f" oct="2">
                              <verse xml:id="v14bnmot" n="1">
                                 <syl con="d" wordpos="m">ber</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1bi21zr" n="67">
                     <staff xml:id="m67s1" n="1">
                        <layer xml:id="m67s1l1" n="1">
                           <note xml:id="n14a2j4g" dots="1" dur="2" pname="b" oct="2">
                              <verse xml:id="va1exrg" n="1">
                                 <syl con="u" wordpos="t">tou!</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="n15ky6ca" dots="1" dur="4" pname="b" oct="2">
                              <accid accid.ges="f" />
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="t1lhtdvg" startid="#n14a2j4g" endid="#n15ky6ca" />
                     <tie xml:id="tekwtn4" startid="#n15ky6ca" endid="#nuy5x9z" />
                  </measure>
                  <measure xml:id="mkkfefu" type="mscore-repeat-2" right="rptend" n="68">
                     <staff xml:id="m68s1" n="1">
                        <layer xml:id="m68s1l1" n="1">
                           <note xml:id="nuy5x9z" dots="1" dur="2" pname="b" oct="2">
                              <accid accid.ges="f" />
                           </note>
                           <rest xml:id="rc083ej" dots="1" dur="4" />
                        </layer>
                     </staff>
                  </measure>
               </section>
            </score>
         </mdiv>
      </body>
   </music>
</mei>"###;
