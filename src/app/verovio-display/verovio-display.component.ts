import { Component, ElementRef, Signal, SimpleChanges, WritableSignal, computed, input, signal } from '@angular/core';
import { DomSanitizer, SafeHtml } from '@angular/platform-browser';
import { outputFromObservable, toObservable, toSignal } from '@angular/core/rxjs-interop';
import { VerovioToolkit } from 'verovio/esm';
import createVerovioModule from 'verovio/wasm';
import { Observable, Subject, combineLatestWith, debounceTime, distinctUntilChanged, filter, map, mergeMap } from 'rxjs';
import { VerovioOptions } from 'verovio';

@Component({
   selector: 'app-verovio-display',
   imports: [],
   templateUrl: './verovio-display.component.html',
   styleUrl: './verovio-display.component.css'
})
export class VerovioDisplayComponent {

   mei = input("")
   scale = input(1);
   page = input(1);
   rawContent: WritableSignal<SafeHtml> = signal("");

   private verovioToolkit: VerovioToolkit | undefined;
   // private rawContent : Signal<SafeHtml> = signal("");
   private size = signal([0, 0]);
   // private meiLoaded signal("");
   // });

   constructor(private sanitizer: DomSanitizer, elemRef: ElementRef<HTMLElement>) {
      let meiO = toObservable(this.mei);
      let pageO = toObservable(this.page);
      let sizeO = observeResize(elemRef.nativeElement).pipe(debounceTime(100));
      let loadO: Observable<VerovioToolkit> = new Observable(loadS => {
         this.initVerovio().then(tk => {
            this.verovioToolkit = tk;
            meiO.subscribe(async mei => {
               await tk.loadData(mei);
               loadS.next(tk);
            });
         });
      });
      loadO.pipe(
         combineLatestWith(sizeO),
         combineLatestWith(pageO),
         // distinctUntilChanged(),
         map(([[tk, size], page]) => {
            let zoom = 50;
            let factor = 1
            let opt : VerovioOptions = {
               scale: zoom,
               pageHeight: (size.height * 100 / zoom) * factor,
               pageWidth: (size.width * 100 / zoom) * factor,
               footer: "none",
            };
            console.log("options", opt);
            tk.setOptions(opt);
            tk.redoLayout();
            return tk.renderToSVG(1);
         }),
         map(svg => sanitizer.bypassSecurityTrustHtml(svg)),
      ).subscribe(svg => {
         console.log("re render");
         this.rawContent.set(svg);
      });
   }

   private async initVerovio() {
      let module = await createVerovioModule()
      return new VerovioToolkit(module);
   }

}

async function loadMei(tk: VerovioToolkit) {
   if (tk == undefined) {
      throw new Error("Verovio toolkit was not initialized");
   }
   let resp = await fetch('https://www.verovio.org/examples/downloads/Schubert_Lindenbaum.mei');
   let mei = await resp.text()
   return mei;
}

async function render(tk: VerovioToolkit, mei: string, page: number) {
   if (!tk.loadData(mei)) {
      throw new Error("Failed to load data into verovio");
   }
   let svg = tk.renderToSVG(page);
   return svg
}


function observeResize(elem: Element): Observable<{ width: number, height: number }> {
   return new Observable(s => {
      let resizeObserver = new ResizeObserver((entries) => {
         s.next({
            width: entries[0].contentRect.width,
            height: entries[0].contentRect.height,
         });
      })
      resizeObserver.observe(elem);
   });
}