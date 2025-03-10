import { Component, signal } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { DisplayComponent } from "./verovio/display/display.component";

@Component({
   selector: 'app-root',
   imports: [FormsModule, DisplayComponent],
   templateUrl: './app.component.html',
   styleUrl: './app.component.css',
})
export class AppComponent {
   mei = signal("");
   page = 1;

   async ngOnInit() {
      // let response = await fetch('mei.xml')
      // let xml = await response.text();
      // this.mei.set(xml);
      await listen<MeiChangedEvent>('meiChanged', (event) => {
         this.mei.set(event.payload.mei);
      });
   }
   async doAction() {
      console.log("invoking action");
      await invoke("action")
   }
}

type MeiChangedEvent = {
   mei: string;
};
