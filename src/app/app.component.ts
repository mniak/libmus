import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { invoke } from '@tauri-apps/api/core';
import { VerovioDisplayComponent } from "./verovio-display/verovio-display.component";

@Component({
   selector: 'app-root',
   imports: [FormsModule, VerovioDisplayComponent],
   templateUrl: './app.component.html',
   styleUrl: './app.component.css'
})
export class AppComponent {
   sangue = meiPozzoli1_2_4;
   page = 1;
}


const meiPozzoli1_2_4 = `<?xml version="1.0" encoding="UTF-8"?>
<?xml-model href="https://music-encoding.org/schema/5.0/mei-basic.rng" type="application/xml" schematypens="http://relaxng.org/ns/structure/1.0"?>
<?xml-model href="https://music-encoding.org/schema/5.0/mei-basic.rng" type="application/xml" schematypens="http://purl.oclc.org/dsdl/schematron"?>
<mei xmlns="http://www.music-encoding.org/ns/mei" meiversion="5.0+basic">
   <meiHead>
      <fileDesc>
         <titleStmt>
            <title type="main">Untitled score</title>
            <title type="subordinate">Subtitle</title>
            <respStmt>
               <persName role="composer">Composer / arranger</persName>
            </respStmt>
         </titleStmt>
         <pubStmt>
            <date isodate="2025-03-09T12:17:44" />
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
                        <rend type="title" fontsize="x-large">Pozzoli</rend>
                        <lb />
                        <rend type="subtitle" fontsize="large">Primeira série</rend>
                     </rend>
                  </pgHead>
                  <staffGrp>
                     <staffDef n="1" lines="5" lines.visible="false" meter.count="2" meter.unit="4">
                     </staffDef>
                  </staffGrp>
               </scoreDef>
               <section xml:id="s1">
                  <pb />
                  <measure xml:id="m42j4hb" n="1">
                     <staff xml:id="m1s1" n="1">
                        <layer xml:id="m1s1l1" n="1">
                           <note xml:id="n14c3kqh" dur="4" pname="e" oct="5" />
                           <note xml:id="n16dpotb" dur="4" pname="e" oct="5" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="msb0m6f" right="dbl" n="2">
                     <staff xml:id="m2s1" n="1">
                        <layer xml:id="m2s1l1" n="1">
                           <note xml:id="n5ve2xh" dur="4" pname="e" oct="5" />
                           <rest xml:id="r11z73rv" dur="4" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1812i3x" n="3">
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
                  <measure xml:id="m1xeafja" right="dbl" n="4">
                     <staff xml:id="m4s1" n="1">
                        <layer xml:id="m4s1l1" n="1">
                           <note xml:id="n1wm5mq1" dur="4" pname="e" oct="5" />
                           <rest xml:id="rz7u014" dur="4" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="me0t7qy" n="5">
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
                  <measure xml:id="mxcs6v2" right="dbl" n="6">
                     <staff xml:id="m6s1" n="1">
                        <layer xml:id="m6s1l1" n="1">
                           <note xml:id="n1rdrp60" dur="4" pname="e" oct="5" />
                           <rest xml:id="r8v92vr" dur="4" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mslkcwc" n="7">
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
                  <measure xml:id="mkj35u9" right="dbl" n="8">
                     <staff xml:id="m8s1" n="1">
                        <layer xml:id="m8s1l1" n="1">
                           <note xml:id="n15lthwj" dur="8" pname="e" oct="5" />
                           <rest xml:id="r1sczkh9" type="mscore-beam-none" dur="8" />
                           <rest xml:id="r1d4kz3w" dur="4" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mec8j3f" n="9">
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
                  <measure xml:id="mle3gt9" right="dbl" n="10">
                     <staff xml:id="m10s1" n="1">
                        <layer xml:id="m10s1l1" n="1">
                           <note xml:id="nq8e51o" dur="4" pname="e" oct="5" />
                           <note xml:id="n1bn585q" dur="8" pname="e" oct="5" />
                           <rest xml:id="r145bzbu" type="mscore-beam-none" dur="8" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1bqezlj" n="11">
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
                  <measure xml:id="m2jxael" right="dbl" n="12">
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
                  <measure xml:id="mlnnt80" n="13">
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
                  <measure xml:id="m1v0h2ps" right="dbl" n="14">
                     <staff xml:id="m14s1" n="1">
                        <layer xml:id="m14s1l1" n="1">
                           <note xml:id="n7qklim" dur="4" pname="e" oct="5" />
                           <note xml:id="n10qhaxr" dur="8" pname="e" oct="5" />
                           <rest xml:id="rnqk535" type="mscore-beam-none" dur="8" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m2fg2lx" n="15">
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
                  <measure xml:id="m2ssg4h" right="dbl" n="16">
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
                  <measure xml:id="m1moresd" n="17">
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
                  <measure xml:id="mh9gvwa" right="dbl" n="18">
                     <staff xml:id="m18s1" n="1">
                        <layer xml:id="m18s1l1" n="1">
                           <note xml:id="n601ehm" dur="4" pname="e" oct="5" />
                           <note xml:id="n8glv24" dur="8" pname="e" oct="5" />
                           <rest xml:id="r1t67s13" type="mscore-beam-none" dur="8" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mqsmzrr" n="19">
                     <staff xml:id="m19s1" n="1">
                        <layer xml:id="m19s1l1" n="1">
                           <note xml:id="n1l4er9p" dur="2" pname="e" oct="5" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m15xzjv5" right="dbl" n="20">
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
</mei>
`;