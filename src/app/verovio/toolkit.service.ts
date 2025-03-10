import { Injectable, signal } from '@angular/core';
import { Observable, combineLatestWith, debounceTime, distinctUntilChanged, filter, map } from 'rxjs';
import { toObservable } from '@angular/core/rxjs-interop';
import { VerovioOptions } from 'verovio';
import { VerovioToolkit } from 'verovio/esm';
import createVerovioModule from 'verovio/wasm';

@Injectable({
  providedIn: 'root'
})
export class ToolkitService {
  public mei = signal("");
  public scale = signal(1);
  public page = signal(1);
  public size = signal(new Size(0, 0));
  public svg = signal("");

  constructor() {
    let pageO = toObservable(this.page);
    let sizeO = toObservable(this.size).pipe(
      debounceTime(100),
      map(size => new Size(
        Math.max(size.width, 100),
        Math.max(size.height, 100),
      )),
    );

    let svgO = toObservable(this.mei).pipe(
      combineLatestWith(this.initVerovio()),
      filter(([_, tk]) => !!tk),
      map(([mei, tk]) => {
        tk.loadData(mei);
        return tk;
      }),
      combineLatestWith(sizeO),
      combineLatestWith(pageO),
      distinctUntilChanged(),
      map(([[tk, size], page]) => {
        let zoom = 50;
        let factor = 1
        let opt: VerovioOptions = {
          scale: zoom,
          pageHeight: (size.height * 100 / zoom) * factor,
          pageWidth: (size.width * 100 / zoom) * factor,
          footer: "none",
        };
        tk.setOptions(opt);
        tk.redoLayout();
        return tk.renderToSVG(page);
      }),
      filter(svg => !!svg),
    );

    svgO.subscribe(this.svg.set);
  }
  private async initVerovio() {
    let module = await createVerovioModule()
    return new VerovioToolkit(module);
  }
}

export function resizeObservable(elem: Element): Observable<Size> {
  return new Observable(s => {
    let resizeObserver = new ResizeObserver((entries) => {
      s.next(new Size(
        entries[0].contentRect.width,
        entries[0].contentRect.height,
      ));
    });
    resizeObserver.observe(elem);
  });
}

export class Size {
  constructor(public width: number, public height: number) { }
}