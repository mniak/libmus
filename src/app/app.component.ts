import { Component, signal } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { invoke } from '@tauri-apps/api/core';
import { VerovioDisplayComponent } from "./verovio-display/verovio-display.component";
import { HttpClient } from '@angular/common/http';

@Component({
   selector: 'app-root',
   imports: [FormsModule, VerovioDisplayComponent],
   templateUrl: './app.component.html',
   styleUrl: './app.component.css',
})
export class AppComponent {
   mei = signal("");
   page = 1;

   async ngOnInit() {
      let response = await fetch('mei.xml')
      let xml = await response.text();
      this.mei.set(xml);
   }
}
