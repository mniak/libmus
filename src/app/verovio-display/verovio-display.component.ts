import { Component } from '@angular/core';
import { DomSanitizer, SafeHtml } from '@angular/platform-browser';
import { VerovioToolkit } from 'verovio/esm';
import createVerovioModule from 'verovio/wasm';

@Component({
  selector: 'app-verovio-display',
  imports: [],
  templateUrl: './verovio-display.component.html',
  styleUrl: './verovio-display.component.css'
})
export class VerovioDisplayComponent {
  private verovioToolkit: VerovioToolkit | undefined;
  rawContent : SafeHtml | undefined;

  constructor(private sanitizer: DomSanitizer) { }
  
  async ngOnInit() {
    this.verovioToolkit = await initVerovio();
    let svg = await getSheetSVG(this.verovioToolkit, 1);
    this.rawContent = this.sanitizer.bypassSecurityTrustHtml(svg);
  }
}
async function initVerovio() {
  let vmod = await createVerovioModule()
  return new VerovioToolkit(vmod);
}

async function getSheetSVG(tk: VerovioToolkit, page: number) {
  if (tk == undefined) {
    throw new Error("Verovio toolkit was not initialized");
  }
  let resp = await fetch('https://www.verovio.org/examples/downloads/Schubert_Lindenbaum.mei');
  let mei = await resp.text()
  if (!tk.loadData(mei)) {
    throw new Error("Failed to load data into verovio");
  }
  let svg = tk.renderToSVG(page);
  return svg
}

