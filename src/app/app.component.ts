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
  sangue = meiSangue;
  page = 1;
}


const meiSangue = `<?xml version="1.0" encoding="UTF-8"?>
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
            <date isodate="2025-03-07T08:47:22" />
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
                  <measure xml:id="m1p7ao57" n="1">
                     <staff xml:id="m1s1" n="1">
                        <layer xml:id="m1s1l1" n="1">
                           <note xml:id="n11tt0sl" dots="1" dur="2" pname="a" oct="3">
                              <verse xml:id="vicuzry" n="1">
                                 <syl con="u">Uh</syl>
                              </verse>
                           </note>
                           <note xml:id="nehf2sl" dots="1" dur="4" pname="e" oct="3" />
                        </layer>
                     </staff>
                     <tempo xml:id="t1o2f0xo" type="mscore-infer-from-text" tstamp="1.000000" midi.bpm="130.000000">
                        <rend glyph.auth="smufl"></rend> = 130</tempo>
                     <reh xml:id="r7t7kty" startid="#n11tt0sl">A</reh>
                  </measure>
                  <measure xml:id="m1yupxaq" n="2">
                     <staff xml:id="m2s1" n="1">
                        <layer xml:id="m2s1l1" n="1">
                           <note xml:id="nbuurwo" dur="2" pname="f" oct="3">
                              <accid accid.ges="s" />
                           </note>
                           <rest xml:id="r1drs8rp" dur="4" />
                           <note xml:id="nq3xsrg" dots="1" dur="4" pname="a" oct="3">
                              <verse xml:id="vm1t3g5" n="1">
                                 <syl con="u">uh</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m9tjzzo" n="3">
                     <staff xml:id="m3s1" n="1">
                        <layer xml:id="m3s1l1" n="1">
                           <note xml:id="n1bms731" dots="1" dur="2" pname="d" oct="2" />
                           <note xml:id="n19tco9o" dots="1" dur="4" pname="e" oct="2">
                              <verse xml:id="v1jwh6gl" n="1">
                                 <syl con="u">uh</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1ehi4hu" right="dbl" n="4">
                     <staff xml:id="m4s1" n="1">
                        <layer xml:id="m4s1l1" n="1">
                           <note xml:id="nty934z" dots="1" dur="2" pname="a" oct="2" />
                           <note xml:id="ny387fc" dots="1" dur="4" pname="a" oct="2" />
                        </layer>
                     </staff>
                     <tie xml:id="tfed5lt" startid="#nty934z" endid="#ny387fc" />
                  </measure>
                  <sb type="mscore-manual" />
                  <measure xml:id="m1cavh3d" n="5">
                     <staff xml:id="m5s1" n="1">
                        <layer xml:id="m5s1l1" n="1">
                           <note xml:id="n33nkn2" dots="1" dur="2" pname="a" oct="2">
                              <verse xml:id="v1vguxmb" n="1">
                                 <syl>Uh</syl>
                              </verse>
                           </note>
                           <rest xml:id="r1wpcand" dots="1" dur="4" />
                        </layer>
                     </staff>
                     <reh xml:id="r1yl0onm" startid="#n33nkn2">E1</reh>
                  </measure>
                  <measure xml:id="ma8nbjy" n="6">
                     <staff xml:id="m6s1" n="1">
                        <layer xml:id="m6s1l1" n="1">
                           <note xml:id="n16in47j" dots="1" dur="2" pname="e" oct="2">
                              <verse xml:id="v22x20f" n="1">
                                 <syl>uh</syl>
                              </verse>
                           </note>
                           <rest xml:id="rcgm14" dots="1" dur="4" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1jnfpaq" n="7">
                     <staff xml:id="m7s1" n="1">
                        <layer xml:id="m7s1l1" n="1">
                           <note xml:id="n1tl0jyn" dur="4" pname="d" oct="2">
                              <verse xml:id="v1jd6xsl" n="1">
                                 <syl>tsum</syl>
                              </verse>
                           </note>
                           <rest xml:id="r1wd4cc" type="mscore-beam-none" dur="8" />
                           <rest xml:id="rzryp42" dots="1" dur="4" />
                           <rest xml:id="r1a6yzeq" dur="4" />
                           <note xml:id="n17ui7ct" dur="8" pname="e" oct="2">
                              <verse xml:id="vurg8mq" n="1">
                                 <syl>tsu</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m182diml" n="8">
                     <staff xml:id="m8s1" n="1">
                        <layer xml:id="m8s1l1" n="1">
                           <note xml:id="n1sp418e" dur="4" pname="a" oct="2">
                              <verse xml:id="v16500uy" n="1">
                                 <syl>tsum</syl>
                              </verse>
                           </note>
                           <rest xml:id="r180uati" type="mscore-beam-none" dur="8" />
                           <note xml:id="n1wkivgo" dur="4" pname="b" oct="2">
                              <verse xml:id="v1ztwig" n="1">
                                 <syl>tsum</syl>
                              </verse>
                           </note>
                           <rest xml:id="r1gvub8o" type="mscore-beam-none" dur="8" />
                           <note xml:id="nfwzxsf" dur="4" pname="c" oct="3">
                              <verse xml:id="v10shnrw" n="1">
                                 <syl>tsum</syl>
                              </verse>
                              <accid xml:id="a1y24ran" accid="n" />
                           </note>
                           <rest xml:id="r1b6zc9s" type="mscore-beam-none" dur="8" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="me8sfgk" n="9">
                     <staff xml:id="m9s1" n="1">
                        <layer xml:id="m9s1l1" n="1">
                           <note xml:id="ne4lpga" dur="8" pname="c" oct="3">
                              <verse xml:id="v16bzrh3" n="1">
                                 <syl>tsum</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                           <rest xml:id="rs2y7fz" type="mscore-beam-none" dur="8" />
                           <rest xml:id="rm6mxn8" type="mscore-beam-none" dur="8" />
                           <note xml:id="n12nmpg6" dur="8" pname="c" oct="3">
                              <verse xml:id="v1h0376x" n="1">
                                 <syl>tsum</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                           <rest xml:id="r74a59s" type="mscore-beam-none" dur="8" />
                           <rest xml:id="r1o5m3rt" type="mscore-beam-none" dur="8" />
                           <note xml:id="n1ony19e" dur="8" pname="c" oct="3">
                              <verse xml:id="v7zjno9" n="1">
                                 <syl>tusm</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                           <rest xml:id="r1ek3r66" type="mscore-beam-none" dur="8" />
                           <rest xml:id="r17qwot2" type="mscore-beam-none" dur="8" />
                        </layer>
                     </staff>
                  </measure>
                  <sb />
                  <measure xml:id="m17t22zb" n="10">
                     <staff xml:id="m10s1" n="1">
                        <layer xml:id="m10s1l1" n="1">
                           <note xml:id="nqakwll" dur="4" pname="c" oct="4">
                              <verse xml:id="visq2rr" n="1">
                                 <syl con="u">E</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                           <note xml:id="nadyrru" dur="8" pname="b" oct="3" />
                           <note xml:id="n1mmzx43" dur="4" pname="g" oct="3">
                              <verse xml:id="vfyeuws" n="1">
                                 <syl con="u">le</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                           <note xml:id="n1tfj8oz" dur="8" pname="f" oct="3">
                              <accid accid.ges="s" />
                           </note>
                           <note xml:id="n1dxa8g0" dur="4" pname="e" oct="3">
                              <verse xml:id="vgzy50a" n="1">
                                 <syl con="u">mor</syl>
                              </verse>
                           </note>
                           <note xml:id="no1gi3f" dur="8" pname="c" oct="3">
                              <accid accid.ges="s" />
                           </note>
                        </layer>
                     </staff>
                     <slur xml:id="sty0nuf" startid="#nqakwll" endid="#nadyrru" />
                     <slur xml:id="sxspugq" startid="#n1mmzx43" endid="#n1tfj8oz" />
                     <slur xml:id="sryhcn5" startid="#n1dxa8g0" endid="#no1gi3f" />
                  </measure>
                  <measure xml:id="mmu3dyd" n="11">
                     <staff xml:id="m11s1" n="1">
                        <layer xml:id="m11s1l1" n="1">
                           <note xml:id="n1gvjdqa" dots="1" dur="4" pname="f" oct="3">
                              <verse xml:id="v1clx02v" n="1">
                                 <syl con="u">reu</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                           <note xml:id="n1cw6lf4" dots="1" dur="4" pname="e" oct="3" />
                           <note xml:id="n1ivetgj" dots="1" dur="4" pname="c" oct="3">
                              <verse xml:id="v1k8vu59" n="1">
                                 <syl>por</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                        </layer>
                     </staff>
                     <slur xml:id="s136brux" startid="#n1gvjdqa" endid="#n1cw6lf4" />
                  </measure>
                  <measure xml:id="m1nyhap4" n="12">
                     <staff xml:id="m12s1" n="1">
                        <layer xml:id="m12s1l1" n="1">
                           <note xml:id="nbyl2or" dur="4" pname="b" oct="2">
                              <verse xml:id="v1ifp83b" n="1">
                                 <syl con="u">mim</syl>
                              </verse>
                           </note>
                           <note xml:id="n193o9ai" dur="8" pname="a" oct="2" />
                           <note xml:id="n1nzu8tm" dur="4" pname="a" oct="2" />
                           <rest xml:id="rjy64ck" type="mscore-beam-none" dur="8" />
                           <note xml:id="n2a7dw9" dur="4" pname="a" oct="2">
                              <verse xml:id="v11vbag5" n="1">
                                 <syl con="d" wordpos="i">e</syl>
                              </verse>
                           </note>
                           <note xml:id="nh8kuxu" dur="8" pname="f" oct="2">
                              <verse xml:id="v5p8q48" n="1">
                                 <syl con="d" wordpos="m">ra es</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                        </layer>
                     </staff>
                     <slur xml:id="s1nznzcw" startid="#nbyl2or" endid="#n193o9ai" />
                     <tie xml:id="tj4r4t1" startid="#n193o9ai" endid="#n1nzu8tm" />
                  </measure>
                  <measure xml:id="m5shq3i" n="13">
                     <staff xml:id="m13s1" n="1">
                        <layer xml:id="m13s1l1" n="1">
                           <note xml:id="nojiybh" dur="4" pname="d" oct="2">
                              <verse xml:id="v4qmabs" n="1">
                                 <syl con="d" wordpos="m">cra</syl>
                              </verse>
                           </note>
                           <note xml:id="n1drz4nb" dur="8" pname="d" oct="2">
                              <verse xml:id="vvr2h0p" n="1">
                                 <syl con="u" wordpos="t">vo</syl>
                              </verse>
                           </note>
                           <note xml:id="n78lv86" dots="1" dur="4" pname="d" oct="2" />
                           <note xml:id="n1kkicz1" dur="4" pname="a" oct="2">
                              <verse xml:id="v1swhmfb" n="1">
                                 <syl>do</syl>
                              </verse>
                           </note>
                           <note xml:id="nzr6sym" dur="8" pname="f" oct="2">
                              <verse xml:id="v1bc6cie" n="1">
                                 <syl con="d" wordpos="i">pe</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="t1gc3www" startid="#n1drz4nb" endid="#n78lv86" />
                  </measure>
                  <measure xml:id="mfbenle" n="14">
                     <staff xml:id="m14s1" n="1">
                        <layer xml:id="m14s1l1" n="1">
                           <note xml:id="n1vsd6iz" dur="4" pname="d" oct="2">
                              <verse xml:id="v16nsxra" n="1">
                                 <syl con="d" wordpos="m">ca</syl>
                              </verse>
                           </note>
                           <note xml:id="n1kpfa6w" dur="8" pname="d" oct="2">
                              <verse xml:id="v1f6t57i" n="1">
                                 <syl con="u" wordpos="t">do</syl>
                              </verse>
                           </note>
                           <note xml:id="nc6ct4h" dots="1" dur="4" pname="d" oct="2" />
                           <note xml:id="n1dpiyyu" dur="4" pname="a" oct="2">
                              <verse xml:id="v45ugry" n="1">
                                 <syl>mas</syl>
                              </verse>
                           </note>
                           <note xml:id="njaxtap" dur="8" pname="f" oct="2">
                              <verse xml:id="vnzurxr" n="1">
                                 <syl con="d" wordpos="i">a</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="tx1y776" startid="#n1kpfa6w" endid="#nc6ct4h" />
                  </measure>
                  <sb />
                  <measure xml:id="m1pkmro5" n="15">
                     <staff xml:id="m15s1" n="1">
                        <layer xml:id="m15s1l1" n="1">
                           <note xml:id="n1syx5gr" dur="4" pname="a" oct="2">
                              <verse xml:id="v1rv63if" n="1">
                                 <syl con="d" wordpos="m">go</syl>
                              </verse>
                           </note>
                           <note xml:id="n1sdycz7" dur="8" pname="a" oct="2">
                              <verse xml:id="v44wwyl" n="1">
                                 <syl con="u" wordpos="t">ra</syl>
                              </verse>
                           </note>
                           <note xml:id="nu6lz3r" dots="1" dur="4" pname="a" oct="2" />
                           <note xml:id="n1gn9v0j" dots="1" dur="4" pname="e" oct="2">
                              <verse xml:id="v1cl6qlh" n="1">
                                 <syl>sou</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="t1320b7n" startid="#n1sdycz7" endid="#nu6lz3r" />
                  </measure>
                  <measure xml:id="m1x13ywv" n="16">
                     <staff xml:id="m16s1" n="1">
                        <layer xml:id="m16s1l1" n="1">
                           <note xml:id="nbplmlx" dur="4" pname="f" oct="2">
                              <verse xml:id="v1jsqeuc" n="1">
                                 <syl con="d" wordpos="i">li</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                           <note xml:id="n31e9kh" dur="8" pname="f" oct="2">
                              <verse xml:id="vuj2eyo" n="1">
                                 <syl con="u" wordpos="t">vre</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                           <note xml:id="n1qqxkk0" dots="1" dur="4" pname="f" oct="2">
                              <accid accid.ges="s" />
                           </note>
                           <note xml:id="n7y48x" dots="1" dur="4" pname="c" oct="3">
                              <verse xml:id="v18mhze" n="1">
                                 <syl>o</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="t1tzsjag" startid="#n31e9kh" endid="#n1qqxkk0" />
                  </measure>
                  <measure xml:id="m1oljiyo" n="17">
                     <staff xml:id="m17s1" n="1">
                        <layer xml:id="m17s1l1" n="1">
                           <note xml:id="n58ff6d" dots="1" dur="4" pname="b" oct="2">
                              <verse xml:id="v1e5ktbp" n="1">
                                 <syl con="d" wordpos="i">san</syl>
                              </verse>
                           </note>
                           <note xml:id="ngf9drv" dots="1" dur="4" pname="b" oct="1">
                              <verse xml:id="v1xbz34s" n="1">
                                 <syl wordpos="t">gue</syl>
                              </verse>
                           </note>
                           <note xml:id="n10thy6k" dots="1" dur="4" pname="b" oct="1">
                              <verse xml:id="v136woio" n="1">
                                 <syl>me</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m8qxioc" n="18">
                     <staff xml:id="m18s1" n="1">
                        <layer xml:id="m18s1l1" n="1">
                           <note xml:id="ng44ha" dots="1" dur="2" pname="e" oct="2">
                              <verse xml:id="vpe6lii" n="1">
                                 <syl>li</syl>
                              </verse>
                           </note>
                           <note xml:id="naubv4z" dots="1" dur="4" pname="e" oct="2">
                              <verse xml:id="vk48yus" n="1">
                                 <syl con="d" wordpos="m">ber</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m16nrgzs" n="19">
                     <staff xml:id="m19s1" n="1">
                        <layer xml:id="m19s1l1" n="1">
                           <note xml:id="n1fggh1o" dots="1" dur="2" pname="a" oct="2">
                              <verse xml:id="v19x2hu9" n="1">
                                 <syl con="u" wordpos="t">tou</syl>
                              </verse>
                           </note>
                           <note xml:id="n1pd98ea" dots="1" dur="4" pname="a" oct="2" />
                        </layer>
                     </staff>
                     <tie xml:id="tcmxjgj" startid="#n1fggh1o" endid="#n1pd98ea" />
                  </measure>
                  <measure xml:id="meu7joi" right="dbl" n="20">
                     <staff xml:id="m20s1" n="1">
                        <layer xml:id="m20s1l1" n="1">
                           <note xml:id="n1lpfo29" dots="1" dur="4" pname="a" oct="2">
                              <verse xml:id="v1qwy0km" n="1">
                                 <syl>pm</syl>
                              </verse>
                           </note>
                           <note xml:id="nqegsgi" dots="1" dur="4" pname="g" oct="2">
                              <verse xml:id="vqlu8ig" n="1">
                                 <syl>pm</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                           <note xml:id="nx0ki90" dots="1" dur="4" pname="f" oct="2">
                              <verse xml:id="vhknzhn" n="1">
                                 <syl>pm.</syl>
                              </verse>
                              <accid accid.ges="s" />
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <sb />
                  <scoreDef keysig="2f" />
                  <measure xml:id="mobg6jx" n="21">
                     <staff xml:id="m21s1" n="1">
                        <layer xml:id="m21s1l1" n="1">
                           <note xml:id="njhyt71" dots="1" dur="2" pname="f" oct="2">
                              <verse xml:id="vswb29t" n="1">
                                 <syl>Pm</syl>
                              </verse>
                           </note>
                           <rest xml:id="r1xm9ehi" dur="4" />
                           <note xml:id="n1fqgcko" dur="8" pname="c" oct="3">
                              <verse xml:id="velh2gy" n="1">
                                 <syl>pm</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                     <reh xml:id="r1oy1hmo" startid="#njhyt71">C</reh>
                  </measure>
                  <measure xml:id="m15cutnd" n="22">
                     <staff xml:id="m22s1" n="1">
                        <layer xml:id="m22s1l1" n="1">
                           <note xml:id="ny8nwkc" dots="1" dur="2" pname="f" oct="2">
                              <verse xml:id="v1iqjznp" n="1">
                                 <syl>pm</syl>
                              </verse>
                           </note>
                           <rest xml:id="r1229jhe" dur="4" />
                           <note xml:id="ng39byr" dur="8" pname="f" oct="2">
                              <verse xml:id="v1v427jc" n="1">
                                 <syl>pm</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m88uj28" n="23">
                     <staff xml:id="m23s1" n="1">
                        <layer xml:id="m23s1l1" n="1">
                           <note xml:id="njhlw5e" dots="1" dur="2" pname="b" oct="2">
                              <verse xml:id="vkz0wu5" n="1">
                                 <syl>pm</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <rest xml:id="rziu6xo" dur="4" />
                           <note xml:id="n2rjzu6" dur="8" pname="f" oct="2">
                              <verse xml:id="v1s328ox" n="1">
                                 <syl>pm</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mbjsu3w" n="24">
                     <staff xml:id="m24s1" n="1">
                        <layer xml:id="m24s1l1" n="1">
                           <note xml:id="n1szotrh" dots="1" dur="4" pname="b" oct="2">
                              <verse xml:id="v6f6a3j" n="1">
                                 <syl>pm</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="n10wtrqy" dots="1" dur="4" pname="a" oct="2">
                              <verse xml:id="v1vscc1n" n="1">
                                 <syl>pm</syl>
                              </verse>
                           </note>
                           <note xml:id="nos26tf" dots="1" dur="4" pname="g" oct="2">
                              <verse xml:id="v1hblhbd" n="1">
                                 <syl>pm</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mirkb8r" n="25">
                     <staff xml:id="m25s1" n="1">
                        <layer xml:id="m25s1l1" n="1">
                           <note xml:id="nw80a1t" dots="1" dur="4" pname="f" oct="3">
                              <verse xml:id="v1e9o358" n="1">
                                 <syl con="d" wordpos="i">gra</syl>
                              </verse>
                           </note>
                           <note xml:id="n1w3i4bo" dots="1" dur="4" pname="f" oct="3">
                              <verse xml:id="v6qr1ek" n="1">
                                 <syl wordpos="t">ça</syl>
                              </verse>
                           </note>
                           <rest xml:id="r1hyzww9" dots="1" dur="4" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m3bgw1r" n="26">
                     <staff xml:id="m26s1" n="1">
                        <layer xml:id="m26s1l1" n="1">
                           <note xml:id="n1vlsm6w" dots="1" dur="4" pname="f" oct="3">
                              <verse xml:id="vflgxkt" n="1">
                                 <syl>me</syl>
                              </verse>
                           </note>
                           <note xml:id="napu7xp" dots="1" dur="4" pname="f" oct="3">
                              <verse xml:id="v1yx8bmu" n="1">
                                 <syl con="d" wordpos="i">con</syl>
                              </verse>
                           </note>
                           <note xml:id="nrgyte" dots="1" dur="4" pname="f" oct="3">
                              <verse xml:id="v3kdrea" n="1">
                                 <syl con="d" wordpos="m">ce</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1aptj8n" n="27">
                     <staff xml:id="m27s1" n="1">
                        <layer xml:id="m27s1l1" n="1">
                           <note xml:id="n1lmlese" dots="1" dur="2" pname="b" oct="2">
                              <verse xml:id="v1lh0nng" n="1">
                                 <syl con="u" wordpos="t">deu</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="nbju4kx" dots="1" dur="4" pname="b" oct="2">
                              <accid accid.ges="f" />
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="t1iyayiw" startid="#n1lmlese" endid="#nbju4kx" />
                     <tie xml:id="t1kdvew5" startid="#nbju4kx" endid="#n1lr846n" />
                  </measure>
                  <sb />
                  <measure xml:id="m1b49hj6" n="28">
                     <staff xml:id="m28s1" n="1">
                        <layer xml:id="m28s1l1" n="1">
                           <note xml:id="n1lr846n" dots="1" dur="2" pname="b" oct="2">
                              <accid accid.ges="f" />
                           </note>
                           <rest xml:id="r1r9psn8" dots="1" dur="4" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mw7czu5" n="29">
                     <staff xml:id="m29s1" n="1">
                        <layer xml:id="m29s1l1" n="1">
                           <note xml:id="ns5b0n0" dots="1" dur="2" pname="e" oct="2">
                              <verse xml:id="v130eqgz" n="1">
                                 <syl con="u">Uh</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="n3nwy58" dots="1" dur="4" pname="e" oct="2">
                              <accid accid.ges="f" />
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="t1ghawuj" startid="#ns5b0n0" endid="#n3nwy58" />
                  </measure>
                  <measure xml:id="m7919g0" n="30">
                     <staff xml:id="m30s1" n="1">
                        <layer xml:id="m30s1l1" n="1">
                           <note xml:id="n1k9je6w" dots="1" dur="2" pname="c" oct="2" />
                           <note xml:id="nng8bjm" dots="1" dur="4" pname="c" oct="2" />
                        </layer>
                     </staff>
                     <tie xml:id="t151eqm1" startid="#n1k9je6w" endid="#nng8bjm" />
                  </measure>
                  <measure xml:id="mco98po" n="31">
                     <staff xml:id="m31s1" n="1">
                        <layer xml:id="m31s1l1" n="1">
                           <note xml:id="ndxosq2" dots="1" dur="2" pname="b" oct="1">
                              <verse xml:id="v6c69a1" n="1">
                                 <syl con="u">uh</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="n1dcugw6" dots="1" dur="4" pname="b" oct="1">
                              <accid accid.ges="f" />
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="t1bsjgqa" startid="#ndxosq2" endid="#n1dcugw6" />
                     <slur xml:id="sc5e2q4" startid="#n1dcugw6" endid="#n94olo1" />
                  </measure>
                  <measure xml:id="mvq34mf" n="32">
                     <staff xml:id="m32s1" n="1">
                        <layer xml:id="m32s1l1" n="1">
                           <note xml:id="n94olo1" dots="1" dur="2" pname="g" oct="2" />
                           <note xml:id="n15wwyh8" dots="1" dur="4" pname="d" oct="3">
                              <verse xml:id="v1q0fdaq" n="1">
                                 <syl>o</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1yr8zx1" n="33">
                     <staff xml:id="m33s1" n="1">
                        <layer xml:id="m33s1l1" n="1">
                           <note xml:id="n1u5z503" dots="1" dur="4" pname="c" oct="3">
                              <verse xml:id="v1yijkid" n="1">
                                 <syl con="d" wordpos="i">san</syl>
                              </verse>
                           </note>
                           <note xml:id="n27byct" dots="1" dur="4" pname="c" oct="2">
                              <verse xml:id="v1rvwjfi" n="1">
                                 <syl wordpos="t">gue</syl>
                              </verse>
                           </note>
                           <note xml:id="n1u0nf8d" dots="1" dur="4" pname="c" oct="2">
                              <verse xml:id="v1x7tu5h" n="1">
                                 <syl>me</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1u2jkx7" n="34">
                     <staff xml:id="m34s1" n="1">
                        <layer xml:id="m34s1l1" n="1">
                           <note xml:id="nw6omkp" dots="1" dur="2" pname="f" oct="2">
                              <verse xml:id="v11cgm2h" n="1">
                                 <syl con="d" wordpos="i">li</syl>
                              </verse>
                           </note>
                           <note xml:id="nz79nt9" dots="1" dur="4" pname="f" oct="2">
                              <verse xml:id="v6qodlw" n="1">
                                 <syl wordpos="t">ber</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1chb7bh" n="35">
                     <staff xml:id="m35s1" n="1">
                        <layer xml:id="m35s1l1" n="1">
                           <note xml:id="nnwpvqt" dots="1" dur="2" pname="b" oct="2">
                              <verse xml:id="v162typb" n="1">
                                 <syl con="u">tou</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="nwphoeo" dots="1" dur="4" pname="b" oct="2">
                              <accid accid.ges="f" />
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="tej2maq" startid="#nnwpvqt" endid="#nwphoeo" />
                  </measure>
                  <measure xml:id="mwxuy8b" right="dbl" n="36">
                     <staff xml:id="m36s1" n="1">
                        <layer xml:id="m36s1l1" n="1">
                           <note xml:id="nbps88l" dots="1" dur="4" pname="f" oct="2">
                              <verse xml:id="vzsbdvo" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                           <note xml:id="n3pzyhz" dots="1" dur="4" pname="g" oct="2">
                              <verse xml:id="v9cm1bd" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                           <note xml:id="n1osznp3" dots="1" dur="4" pname="a" oct="2">
                              <verse xml:id="v47v9bl" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <sb />
                  <measure xml:id="m1rrt6sp" n="37">
                     <staff xml:id="m37s1" n="1">
                        <layer xml:id="m37s1l1" n="1">
                           <note xml:id="nwv82pp" dots="1" dur="4" pname="b" oct="2">
                              <verse xml:id="vrq65om" n="1">
                                 <syl>Tm</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="nbu6m0f" dots="1" dur="4" pname="d" oct="3">
                              <verse xml:id="vnnm3io" n="1">
                                 <syl>tm</syl>
                              </verse>
                           </note>
                           <note xml:id="nw2pz7u" dots="1" dur="4" pname="b" oct="2">
                              <verse xml:id="v1s0pi95" n="1">
                                 <syl>tm</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                        </layer>
                     </staff>
                     <reh xml:id="rcjb6ah" startid="#nwv82pp">D</reh>
                  </measure>
                  <measure xml:id="m5yb5q1" n="38">
                     <staff xml:id="m38s1" n="1">
                        <layer xml:id="m38s1l1" n="1">
                           <note xml:id="n1xxvi02" dots="1" dur="4" pname="a" oct="2">
                              <verse xml:id="v4e3w0h" n="1">
                                 <syl>tm</syl>
                              </verse>
                           </note>
                           <note xml:id="nk5bxy5" dots="1" dur="4" pname="d" oct="3">
                              <verse xml:id="v18p214c" n="1">
                                 <syl>tm</syl>
                              </verse>
                           </note>
                           <note xml:id="n13g09fy" dots="1" dur="4" pname="a" oct="2">
                              <verse xml:id="v1s6rb21" n="1">
                                 <syl>tm</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mu879mx" n="39">
                     <staff xml:id="m39s1" n="1">
                        <layer xml:id="m39s1l1" n="1">
                           <note xml:id="ng0ztax" dots="1" dur="4" pname="g" oct="2">
                              <verse xml:id="v3crpsb" n="1">
                                 <syl>tm</syl>
                              </verse>
                           </note>
                           <note xml:id="n1rfzs5j" dots="1" dur="4" pname="a" oct="2">
                              <verse xml:id="vcb866s" n="1">
                                 <syl>tm</syl>
                              </verse>
                           </note>
                           <note xml:id="npbih7p" dots="1" dur="4" pname="b" oct="2">
                              <verse xml:id="v1gtg6cn" n="1">
                                 <syl>tm</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mwpp3t1" n="40">
                     <staff xml:id="m40s1" n="1">
                        <layer xml:id="m40s1l1" n="1">
                           <note xml:id="nyayem1" dots="1" dur="4" pname="d" oct="3">
                              <verse xml:id="v1i8lu66" n="1">
                                 <syl>tm</syl>
                              </verse>
                           </note>
                           <note xml:id="ntlth2h" dots="1" dur="4" pname="c" oct="3">
                              <verse xml:id="v8ra7uf" n="1">
                                 <syl>tm</syl>
                              </verse>
                           </note>
                           <note xml:id="n13oaen7" dots="1" dur="4" pname="b" oct="2">
                              <verse xml:id="vtldvts" n="1">
                                 <syl>tm</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mflskvr" n="41">
                     <staff xml:id="m41s1" n="1">
                        <layer xml:id="m41s1l1" n="1">
                           <note xml:id="n1h89w4u" dots="1" dur="2" pname="a" oct="2">
                              <verse xml:id="v539htd" n="1">
                                 <syl con="u">tm</syl>
                              </verse>
                           </note>
                           <note xml:id="nsaf69b" dots="1" dur="4" pname="a" oct="2" />
                        </layer>
                     </staff>
                     <tie xml:id="td2kblc" startid="#n1h89w4u" endid="#nsaf69b" />
                  </measure>
                  <measure xml:id="m2vijdl" n="42">
                     <staff xml:id="m42s1" n="1">
                        <layer xml:id="m42s1l1" n="1">
                           <note xml:id="ncdmuyc" dots="1" dur="4" pname="d" oct="2">
                              <verse xml:id="vbkybl8" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                           <note xml:id="nsjx23g" dots="1" dur="4" pname="e" oct="2">
                              <verse xml:id="vwx9hrr" n="1">
                                 <syl>tum</syl>
                              </verse>
                              <accid xml:id="a4ptdmj" accid="n" />
                           </note>
                           <note xml:id="n4246vd" dots="1" dur="4" pname="g" oct="2">
                              <verse xml:id="vzmgpwt" n="1">
                                 <syl>tum</syl>
                              </verse>
                              <accid xml:id="aunrab0" accid="f" />
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1f58e33" n="43">
                     <staff xml:id="m43s1" n="1">
                        <layer xml:id="m43s1l1" n="1">
                           <note xml:id="n1clq7hc" dots="1" dur="2" pname="g" oct="2">
                              <verse xml:id="vgn9b2y" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                           <note xml:id="nz16ike" dots="1" dur="4" pname="g" oct="2">
                              <verse xml:id="v1qiz5wx" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <sb />
                  <measure xml:id="mx3tjnd" n="44">
                     <staff xml:id="m44s1" n="1">
                        <layer xml:id="m44s1l1" n="1">
                           <note xml:id="neksmem" dots="1" dur="4" pname="f" oct="2">
                              <verse xml:id="vci619r" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                           <note xml:id="n1ehnja0" dur="4" pname="f" oct="2" />
                           <note xml:id="n1qzt5g9" dur="8" pname="b" oct="2">
                              <verse xml:id="v4rbocp" n="1">
                                 <syl con="d" wordpos="i">Tu</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="na53p7o" dur="4" pname="g" oct="2">
                              <verse xml:id="v1auz2bp" n="1">
                                 <syl wordpos="t">u</syl>
                              </verse>
                           </note>
                           <note xml:id="n1gjeb5d" dur="8" pname="f" oct="2">
                              <verse xml:id="v12m9b2x" n="1">
                                 <syl con="d" wordpos="i">du</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="t1uv5dyy" startid="#neksmem" endid="#n1ehnja0" />
                     <slur xml:id="su4hokb" startid="#n1qzt5g9" endid="#na53p7o" />
                  </measure>
                  <measure xml:id="muoxu2k" n="45">
                     <staff xml:id="m45s1" n="1">
                        <layer xml:id="m45s1l1" n="1">
                           <note xml:id="n12mamlu" dur="4" pname="e" oct="2">
                              <verse xml:id="v1cw5c1f" n="1">
                                 <syl wordpos="t">dum</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <rest xml:id="rnbcllp" type="mscore-beam-none" dur="8" />
                           <rest xml:id="r4tlgm5" dur="4" />
                           <note xml:id="nufno2j" dur="8" pname="c" oct="3">
                              <verse xml:id="vb3osko" n="1">
                                 <syl con="u">tu</syl>
                              </verse>
                           </note>
                           <note xml:id="n2j4rpn" dur="4" pname="b" oct="2">
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="n18hz9hg" dur="8" pname="c" oct="3">
                              <verse xml:id="v695u94" n="1">
                                 <syl>du</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                     <slur xml:id="sod3ifm" startid="#nufno2j" endid="#n2j4rpn" />
                  </measure>
                  <measure xml:id="m1dcpkc5" n="46">
                     <staff xml:id="m46s1" n="1">
                        <layer xml:id="m46s1l1" n="1">
                           <note xml:id="nc77784" dots="1" dur="4" pname="e" oct="2">
                              <verse xml:id="vomzzvb" n="1">
                                 <syl>dum</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <rest xml:id="r1q4i919" dur="4" />
                           <note xml:id="n17astrr" dur="8" pname="e" oct="2">
                              <verse xml:id="v8g213x" n="1">
                                 <syl con="u">tu</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="n1ek0q9o" dur="4" pname="f" oct="2" />
                           <note xml:id="nkigjbq" dur="8" pname="g" oct="2">
                              <verse xml:id="v19uuffl" n="1">
                                 <syl>du</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                     <slur xml:id="s19w0da1" startid="#n17astrr" endid="#n1ek0q9o" />
                  </measure>
                  <measure xml:id="mlougfl" n="47">
                     <staff xml:id="m47s1" n="1">
                        <layer xml:id="m47s1l1" n="1">
                           <note xml:id="n1thasjx" dots="1" dur="2" pname="b" oct="2">
                              <verse xml:id="vc6rnad" n="1">
                                 <syl con="u">du</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <beam>
                              <note xml:id="n1otqds6" dur="8" pname="c" oct="3">
                                 <verse xml:id="vc3nta" n="1">
                                    <syl>fui</syl>
                                 </verse>
                              </note>
                              <note xml:id="n14kt2m6" dur="8" pname="b" oct="2">
                                 <verse xml:id="v173xw41" n="1">
                                    <syl con="d" wordpos="i">re</syl>
                                 </verse>
                                 <accid accid.ges="f" />
                              </note>
                              <note xml:id="n1j8mmcf" dur="8" pname="a" oct="2" />
                           </beam>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mf6g55m" n="48">
                     <staff xml:id="m48s1" n="1">
                        <layer xml:id="m48s1l1" n="1">
                           <note xml:id="n1o533ff" dur="4" pname="g" oct="2">
                              <verse xml:id="v12vx6ru" n="1">
                                 <syl wordpos="t">mi</syl>
                              </verse>
                           </note>
                           <note xml:id="n1lrlre6" dur="8" pname="g" oct="2" />
                           <note xml:id="nkibr7l" dots="1" dur="4" pname="g" oct="2" />
                           <note xml:id="nn5e07r" dots="1" dur="4" pname="d" oct="3">
                              <verse xml:id="v1f34l3s" n="1">
                                 <syl>o</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="t1hoymh7" startid="#n1lrlre6" endid="#nkibr7l" />
                  </measure>
                  <sb />
                  <measure xml:id="m1vli24w" n="49">
                     <staff xml:id="m49s1" n="1">
                        <layer xml:id="m49s1l1" n="1">
                           <note xml:id="n1a58t9r" dots="1" dur="4" pname="c" oct="3">
                              <verse xml:id="v1xxyet2" n="1">
                                 <syl con="d" wordpos="i">san</syl>
                              </verse>
                           </note>
                           <note xml:id="nc5zlm1" dots="1" dur="4" pname="c" oct="2">
                              <verse xml:id="v1sfy22v" n="1">
                                 <syl wordpos="t">gue</syl>
                              </verse>
                           </note>
                           <note xml:id="nbwrooy" dots="1" dur="4" pname="c" oct="2">
                              <verse xml:id="v1u9cjnw" n="1">
                                 <syl>me</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m9oxqih" n="50">
                     <staff xml:id="m50s1" n="1">
                        <layer xml:id="m50s1l1" n="1">
                           <note xml:id="n1xnqyjz" dots="1" dur="2" pname="f" oct="2">
                              <verse xml:id="v91gdqd" n="1">
                                 <syl con="d" wordpos="i">li</syl>
                              </verse>
                           </note>
                           <note xml:id="nj0lxw4" dots="1" dur="4" pname="f" oct="2">
                              <verse xml:id="vybizug" n="1">
                                 <syl con="d" wordpos="m">ber</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1kissnk" n="51">
                     <staff xml:id="m51s1" n="1">
                        <layer xml:id="m51s1l1" n="1">
                           <note xml:id="n1fumhxc" dots="1" dur="2" pname="b" oct="2">
                              <verse xml:id="vv64eta" n="1">
                                 <syl con="u" wordpos="t">tou</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <rest xml:id="rj7ivfh" dur="4" />
                           <rest xml:id="r1mrithn" type="mscore-beam-none" dur="8" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mpgoko" right="dbl" n="52">
                     <staff xml:id="m52s1" n="1">
                        <layer xml:id="m52s1l1" n="1">
                           <note xml:id="nd6cjgx" dur="4" pname="g" oct="2">
                              <verse xml:id="v11lusfr" n="1">
                                 <syl con="d" wordpos="i">Uo</syl>
                              </verse>
                           </note>
                           <note xml:id="n1m0gvgm" dur="8" pname="g" oct="2">
                              <verse xml:id="vqp1csl" n="1">
                                 <syl con="u" wordpos="t">ou!</syl>
                              </verse>
                           </note>
                           <note xml:id="n1wlqck4" dur="4" pname="g" oct="2" />
                           <rest xml:id="rsabmhj" type="mscore-beam-none" dur="8" />
                           <rest xml:id="r1hkvy2h" dots="1" dur="4" />
                        </layer>
                     </staff>
                     <tie xml:id="t2585oo" startid="#n1m0gvgm" endid="#n1wlqck4" />
                  </measure>
                  <measure xml:id="m1u9mbcz" n="53">
                     <staff xml:id="m53s1" n="1">
                        <layer xml:id="m53s1l1" n="1">
                           <note xml:id="n1wuau4b" dots="1" dur="2" pname="f" oct="2">
                              <verse xml:id="v1acb3tz" n="1">
                                 <syl>Tum</syl>
                              </verse>
                           </note>
                           <rest xml:id="r36db55" dur="4" />
                           <note xml:id="n104t0ny" dur="8" pname="c" oct="3">
                              <verse xml:id="vi074zd" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1fpbav8" n="54">
                     <staff xml:id="m54s1" n="1">
                        <layer xml:id="m54s1l1" n="1">
                           <note xml:id="n4t15ou" dots="1" dur="2" pname="f" oct="2">
                              <verse xml:id="v1xzodzy" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                           <rest xml:id="r139nwf0" dur="4" />
                           <note xml:id="n1y26xes" dur="8" pname="f" oct="2">
                              <verse xml:id="vtp6h04" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="mweb3vl" n="55">
                     <staff xml:id="m55s1" n="1">
                        <layer xml:id="m55s1l1" n="1">
                           <note xml:id="naa1x2g" dots="1" dur="2" pname="b" oct="2">
                              <verse xml:id="vkojifu" n="1">
                                 <syl>tum</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <rest xml:id="r19sp39v" dur="4" />
                           <note xml:id="nze428f" dur="8" pname="f" oct="2">
                              <verse xml:id="vw3i5qj" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <pb />
                  <measure xml:id="m1w8zkvz" n="56">
                     <staff xml:id="m56s1" n="1">
                        <layer xml:id="m56s1l1" n="1">
                           <note xml:id="n1a3j9zb" dots="1" dur="4" pname="b" oct="2">
                              <verse xml:id="vm0gl55" n="1">
                                 <syl>tum</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="n171waj3" dots="1" dur="4" pname="a" oct="2">
                              <verse xml:id="v1nw7enz" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                           <note xml:id="n1upz9nm" dots="1" dur="4" pname="g" oct="2">
                              <verse xml:id="v1q92wv0" n="1">
                                 <syl>tum</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m5c3e17" n="57">
                     <staff xml:id="m57s1" n="1">
                        <layer xml:id="m57s1l1" n="1">
                           <note xml:id="n1voz31r" dots="1" dur="4" pname="f" oct="3">
                              <verse xml:id="v18e46r6" n="1">
                                 <syl con="d" wordpos="i">gra</syl>
                              </verse>
                           </note>
                           <note xml:id="nzgubbp" dots="1" dur="4" pname="f" oct="3">
                              <verse xml:id="v1951gug" n="1">
                                 <syl wordpos="t">ça</syl>
                              </verse>
                           </note>
                           <rest xml:id="r1t96i6g" dots="1" dur="4" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m16yxr2p" n="58">
                     <staff xml:id="m58s1" n="1">
                        <layer xml:id="m58s1l1" n="1">
                           <note xml:id="n1c8jcxq" dur="4" pname="f" oct="3">
                              <verse xml:id="v10c60md" n="1">
                                 <syl>me</syl>
                              </verse>
                           </note>
                           <rest xml:id="r1methm4" type="mscore-beam-none" dur="8" />
                           <note xml:id="n1yvfldb" dur="4" pname="f" oct="3">
                              <verse xml:id="v4xkwmx" n="1">
                                 <syl con="d" wordpos="i">con</syl>
                              </verse>
                           </note>
                           <rest xml:id="ry3oxib" type="mscore-beam-none" dur="8" />
                           <note xml:id="nbn5bu4" dur="4" pname="f" oct="3">
                              <verse xml:id="v1yo6ucj" n="1">
                                 <syl wordpos="t">ce</syl>
                              </verse>
                           </note>
                           <rest xml:id="r1vv0saj" type="mscore-beam-none" dur="8" />
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m11qnn22" n="59">
                     <staff xml:id="m59s1" n="1">
                        <layer xml:id="m59s1l1" n="1">
                           <note xml:id="nw00ie5" dots="1" dur="2" pname="b" oct="2">
                              <verse xml:id="v1l2z2j7" n="1">
                                 <syl con="u">deu</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="n4c547b" dots="1" dur="4" pname="b" oct="2">
                              <accid accid.ges="f" />
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="t1mxcq6o" startid="#nw00ie5" endid="#n4c547b" />
                     <tie xml:id="tn0apes" startid="#n4c547b" endid="#n1nzxpbx" />
                  </measure>
                  <measure xml:id="m1vg0r5m" n="60">
                     <staff xml:id="m60s1" n="1">
                        <layer xml:id="m60s1l1" n="1">
                           <note xml:id="n1nzxpbx" dots="1" dur="2" pname="b" oct="2">
                              <accid accid.ges="f" />
                           </note>
                           <rest xml:id="rbxgrq3" dots="1" dur="4" />
                        </layer>
                     </staff>
                  </measure>
                  <sb type="mscore-manual" />
                  <measure xml:id="m1uv7roc" left="rptstart" n="61">
                     <staff xml:id="m61s1" n="1">
                        <layer xml:id="m61s1l1" n="1">
                           <note xml:id="n1hosvo8" dots="1" dur="2" pname="e" oct="2">
                              <verse xml:id="v1tdm2m8" n="1">
                                 <syl con="u">Uh</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="n15utsmr" dots="1" dur="4" pname="e" oct="2">
                              <accid accid.ges="f" />
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="t13hkq8o" startid="#n1hosvo8" endid="#n15utsmr" />
                  </measure>
                  <measure xml:id="m1qitma2" n="62">
                     <staff xml:id="m62s1" n="1">
                        <layer xml:id="m62s1l1" n="1">
                           <note xml:id="n1m4z39q" dots="1" dur="2" pname="c" oct="2" />
                           <note xml:id="nfpolc5" dots="1" dur="4" pname="c" oct="2" />
                        </layer>
                     </staff>
                     <tie xml:id="t891err" startid="#n1m4z39q" endid="#nfpolc5" />
                  </measure>
                  <measure xml:id="m1wdfdm8" n="63">
                     <staff xml:id="m63s1" n="1">
                        <layer xml:id="m63s1l1" n="1">
                           <note xml:id="n1m4utt5" dots="1" dur="2" pname="b" oct="1">
                              <verse xml:id="v1enzmly" n="1">
                                 <syl con="u">uh</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="nea8ppd" dots="1" dur="4" pname="b" oct="1">
                              <accid accid.ges="f" />
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="t1a8ce04" startid="#n1m4utt5" endid="#nea8ppd" />
                  </measure>
                  <measure xml:id="m19ozodc" n="64">
                     <staff xml:id="m64s1" n="1">
                        <layer xml:id="m64s1l1" n="1">
                           <note xml:id="nm66h6r" dots="1" dur="2" pname="g" oct="2" />
                           <note xml:id="n1ryva7y" dots="1" dur="4" pname="d" oct="3">
                              <verse xml:id="v2wrgew" n="1">
                                 <syl>o</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1jktdpn" n="65">
                     <staff xml:id="m65s1" n="1">
                        <layer xml:id="m65s1l1" n="1">
                           <note xml:id="n1kszach" dots="1" dur="4" pname="c" oct="3">
                              <verse xml:id="vrfxzpc" n="1">
                                 <syl con="d" wordpos="i">san</syl>
                              </verse>
                           </note>
                           <note xml:id="nmw8vhc" dots="1" dur="4" pname="c" oct="2">
                              <verse xml:id="v1wp5o1a" n="1">
                                 <syl wordpos="t">gue</syl>
                              </verse>
                           </note>
                           <note xml:id="n1a5o85b" dots="1" dur="4" pname="c" oct="2">
                              <verse xml:id="v3vtzm5" n="1">
                                 <syl>me</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m6knw7w" n="66">
                     <staff xml:id="m66s1" n="1">
                        <layer xml:id="m66s1l1" n="1">
                           <note xml:id="nq5uvww" dots="1" dur="2" pname="f" oct="2">
                              <verse xml:id="v1jnjp72" n="1">
                                 <syl con="d" wordpos="i">li</syl>
                              </verse>
                           </note>
                           <note xml:id="njmscfb" dots="1" dur="4" pname="f" oct="2">
                              <verse xml:id="v1pqnjco" n="1">
                                 <syl con="d" wordpos="m">ber</syl>
                              </verse>
                           </note>
                        </layer>
                     </staff>
                  </measure>
                  <measure xml:id="m1sond31" n="67">
                     <staff xml:id="m67s1" n="1">
                        <layer xml:id="m67s1l1" n="1">
                           <note xml:id="ngdaum9" dots="1" dur="2" pname="b" oct="2">
                              <verse xml:id="v1g97st8" n="1">
                                 <syl con="u" wordpos="t">tou!</syl>
                              </verse>
                              <accid accid.ges="f" />
                           </note>
                           <note xml:id="n18hnmks" dots="1" dur="4" pname="b" oct="2">
                              <accid accid.ges="f" />
                           </note>
                        </layer>
                     </staff>
                     <tie xml:id="tuvwha3" startid="#ngdaum9" endid="#n18hnmks" />
                     <tie xml:id="t90980j" startid="#n18hnmks" endid="#nhp5435" />
                  </measure>
                  <measure xml:id="mjbiglo" type="mscore-repeat-2" right="rptend" n="68">
                     <staff xml:id="m68s1" n="1">
                        <layer xml:id="m68s1l1" n="1">
                           <note xml:id="nhp5435" dots="1" dur="2" pname="b" oct="2">
                              <accid accid.ges="f" />
                           </note>
                           <rest xml:id="r1skcgb9" dots="1" dur="4" />
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