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
            <date isodate="2025-03-09T12:10:22" />
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
                     <staffDef n="1" lines="5" meter.count="2" meter.unit="4">
                        <clef shape="F" line="4" />
                     </staffDef>
                  </staffGrp>
               </scoreDef>
               <section xml:id="s1">
                  <pb />
                  <measure xml:id="m1k2e9qf" n="1">
                     <staff xml:id="m1s1" n="1">
                        <layer xml:id="m1s1l1" n="1">
                           <note xml:id="n16ao8z2" dur="4" pname="g" oct="3" />
                           <note xml:id="n819rax" dur="4" pname="g" oct="3" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m11nic11" right="dbl" n="2">
                     <staff xml:id="m2s1" n="1">
                        <layer xml:id="m2s1l1" n="1">
                           <note xml:id="n1gja4m9" dur="4" pname="g" oct="3" />
                           <rest xml:id="rmxvevk" dur="4" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1dj4d2q" n="3">
                     <staff xml:id="m3s1" n="1">
                        <layer xml:id="m3s1l1" n="1">
                           <note xml:id="n1eh2qsz" dur="4" pname="g" oct="3" />
                           <beam>
                              <note xml:id="n1wq3465" dur="8" pname="g" oct="3" />
                              <note xml:id="nwd1mi5" dur="8" pname="g" oct="3" />
                           </beam>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mp1flpp" right="dbl" n="4">
                     <staff xml:id="m4s1" n="1">
                        <layer xml:id="m4s1l1" n="1">
                           <note xml:id="nb01m04" dur="4" pname="g" oct="3" />
                           <rest xml:id="r1wughxx" dur="4" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mmq5av3" n="5">
                     <staff xml:id="m5s1" n="1">
                        <layer xml:id="m5s1l1" n="1">
                           <beam>
                              <note xml:id="naey92i" dur="8" pname="g" oct="3" />
                              <note xml:id="n1msaaop" dur="8" pname="g" oct="3" />
                           </beam>
                           <note xml:id="n1p2nspa" dur="4" pname="g" oct="3" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mki2c9q" right="dbl" n="6">
                     <staff xml:id="m6s1" n="1">
                        <layer xml:id="m6s1l1" n="1">
                           <note xml:id="nbp9tl2" dur="4" pname="g" oct="3" />
                           <rest xml:id="r167ysgk" dur="4" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1d0yqbu" n="7">
                     <staff xml:id="m7s1" n="1">
                        <layer xml:id="m7s1l1" n="1">
                           <beam>
                              <note xml:id="n1lkezqx" dur="8" pname="g" oct="3" />
                              <note xml:id="nxva85u" dur="8" pname="g" oct="3" />
                           </beam>
                           <beam>
                              <note xml:id="n1vgflse" dur="8" pname="g" oct="3" />
                              <note xml:id="n26s7z9" dur="8" pname="g" oct="3" />
                           </beam>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1innbhe" right="dbl" n="8">
                     <staff xml:id="m8s1" n="1">
                        <layer xml:id="m8s1l1" n="1">
                           <note xml:id="n4nxvu9" dur="8" pname="g" oct="3" />
                           <rest xml:id="rkpfs6o" type="mscore-beam-none" dur="8" />
                           <rest xml:id="rivhwcx" dur="4" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mut2l2p" n="9">
                     <staff xml:id="m9s1" n="1">
                        <layer xml:id="m9s1l1" n="1">
                           <beam>
                              <note xml:id="n1ezdabc" dur="8" pname="g" oct="3" />
                              <note xml:id="nfiiulb" dur="8" pname="g" oct="3" />
                           </beam>
                           <beam>
                              <note xml:id="nwhh4fd" dur="8" pname="g" oct="3" />
                              <note xml:id="n1k2fdst" dur="8" pname="g" oct="3" />
                           </beam>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1lhsrke" right="dbl" n="10">
                     <staff xml:id="m10s1" n="1">
                        <layer xml:id="m10s1l1" n="1">
                           <note xml:id="ni49ghd" dur="4" pname="g" oct="3" />
                           <note xml:id="n11pfp73" dur="8" pname="g" oct="3" />
                           <rest xml:id="r11j5e3a" type="mscore-beam-none" dur="8" />
                        </layer>
                     </staff>
                  </measure>
                  <sb />
                  <measure xml:id="m6d2y6y" n="11">
                     <staff xml:id="m11s1" n="1">
                        <layer xml:id="m11s1l1" n="1">
                           <note xml:id="n1vgpdzf" dur="4" pname="g" oct="3" />
                           <beam>
                              <note xml:id="niez9o7" dur="8" pname="g" oct="3" />
                              <note xml:id="n9tn2dj" dur="8" pname="g" oct="3" />
                           </beam>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m9670xd" right="dbl" n="12">
                     <staff xml:id="m12s1" n="1">
                        <layer xml:id="m12s1l1" n="1">
                           <beam>
                              <note xml:id="nbc11s7" dur="8" pname="g" oct="3" />
                              <note xml:id="n15kcsvo" dur="8" pname="g" oct="3" />
                           </beam>
                           <note xml:id="n8mvt5w" dur="8" pname="g" oct="3" />
                           <rest xml:id="r1cnn040" type="mscore-beam-none" dur="8" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m17gvhz5" n="13">
                     <staff xml:id="m13s1" n="1">
                        <layer xml:id="m13s1l1" n="1">
                           <beam>
                              <note xml:id="n14ilgll" dur="8" pname="g" oct="3" />
                              <note xml:id="n16mizqh" dur="8" pname="g" oct="3" />
                           </beam>
                           <note xml:id="ns8g2bo" dur="4" pname="g" oct="3" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mncssx3" right="dbl" n="14">
                     <staff xml:id="m14s1" n="1">
                        <layer xml:id="m14s1l1" n="1">
                           <note xml:id="n42qc6k" dur="4" pname="g" oct="3" />
                           <note xml:id="n1bxkhic" dur="8" pname="g" oct="3" />
                           <rest xml:id="r1ak510m" type="mscore-beam-none" dur="8" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mq69x29" n="15">
                     <staff xml:id="m15s1" n="1">
                        <layer xml:id="m15s1l1" n="1">
                           <beam>
                              <note xml:id="n1drt540" dur="8" pname="g" oct="3" />
                              <note xml:id="n177sjln" dur="8" pname="g" oct="3" />
                           </beam>
                           <beam>
                              <note xml:id="nhyr88y" dur="8" pname="g" oct="3" />
                              <note xml:id="nq1j1sn" dur="8" pname="g" oct="3" />
                           </beam>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="min82dl" right="dbl" n="16">
                     <staff xml:id="m16s1" n="1">
                        <layer xml:id="m16s1l1" n="1">
                           <beam>
                              <note xml:id="n1wtnfae" dur="8" pname="g" oct="3" />
                              <note xml:id="n1bhzuor" dur="8" pname="g" oct="3" />
                           </beam>
                           <note xml:id="n1uzryyy" dur="8" pname="g" oct="3" />
                           <rest xml:id="r1ai626q" type="mscore-beam-none" dur="8" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1oqv81g" n="17">
                     <staff xml:id="m17s1" n="1">
                        <layer xml:id="m17s1l1" n="1">
                           <note xml:id="nbwee2f" dur="4" pname="g" oct="3" />
                           <beam>
                              <note xml:id="nipoth4" dur="8" pname="g" oct="3" />
                              <note xml:id="n5lpnrz" dur="8" pname="g" oct="3" />
                           </beam>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mi6m2db" right="dbl" n="18">
                     <staff xml:id="m18s1" n="1">
                        <layer xml:id="m18s1l1" n="1">
                           <note xml:id="ng2md90" dur="4" pname="g" oct="3" />
                           <note xml:id="nhbhc8p" dur="8" pname="g" oct="3" />
                           <rest xml:id="r1fjuljj" type="mscore-beam-none" dur="8" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m19k3206" n="19">
                     <staff xml:id="m19s1" n="1">
                        <layer xml:id="m19s1l1" n="1">
                           <note xml:id="n1ovn336" dur="2" pname="g" oct="3" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m15p13wv" right="dbl" n="20">
                     <staff xml:id="m20s1" n="1">
                        <layer xml:id="m20s1l1" n="1">
                           <beam>
                              <note xml:id="nlzsce1" dur="8" pname="g" oct="3" />
                              <note xml:id="n42va17" dur="8" pname="g" oct="3" />
                           </beam>
                           <note xml:id="n19gd1l5" dur="8" pname="g" oct="3" />
                           <rest xml:id="rs0neha" type="mscore-beam-none" dur="8" />
                        </layer>
                     </staff>
                  </measure>
               </section>
            </score>
         </mdiv>
      </body>
   </music>
</mei>`;