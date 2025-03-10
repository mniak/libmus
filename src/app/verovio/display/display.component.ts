import { Component, ElementRef, ViewEncapsulation, WritableSignal, input, signal } from '@angular/core';
import { DomSanitizer, SafeHtml } from '@angular/platform-browser';
import { map } from 'rxjs';
import { toObservable } from '@angular/core/rxjs-interop';
import { VerovioToolkit } from 'verovio/esm';
import createVerovioModule from 'verovio/wasm';
import { ToolkitService, resizeObservable } from '../toolkit.service';

@Component({
   selector: 'VerovioDisplay',
   imports: [],
   templateUrl: './display.component.html',
   styleUrl: './display.component.css',
   encapsulation: ViewEncapsulation.None,
})
export class DisplayComponent {
   mei = input("")
   scale = input(1);
   page = input(1);
   rawContent: WritableSignal<SafeHtml> = signal("");

   async initVerovio() {
      let module = await createVerovioModule()
      return new VerovioToolkit(module);
   }

   constructor(sanitizer: DomSanitizer, elemRef: ElementRef<HTMLElement>, tkSvc: ToolkitService) {
      toObservable(this.mei).subscribe(tkSvc.mei.set);
      toObservable(this.scale).subscribe(tkSvc.scale.set);
      toObservable(this.page).subscribe(tkSvc.page.set);
      resizeObservable(elemRef.nativeElement).subscribe(tkSvc.size.set);

      toObservable(tkSvc.svg).pipe(
         map(svg => sanitizer.bypassSecurityTrustHtml(svg)),
      ).subscribe(this.rawContent.set);
   }
}
