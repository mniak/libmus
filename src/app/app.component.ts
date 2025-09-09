import { Component, input, signal } from "@angular/core";
import { FormsModule } from "@angular/forms";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { DisplayComponent } from "./verovio/display/display.component";

@Component({
  selector: "app-root",
  imports: [FormsModule, DisplayComponent],
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.css",
})
export class AppComponent {
  mei = signal("");
  page = 1;
  exercise = "Simple";

  async ngOnInit() {
    // let response = await fetch('mei.xml')
    // let xml = await response.text();
    // this.mei.set(xml);
    await listen<MeiChangedEvent>("meiChanged", (event) => {
      console.log("XML received from backend:", event.payload.mei);
      this.mei.set(event.payload.mei);
    });
  }
  async refresh() {
    await invoke("refresh", { exercise: this.exercise});
  }
}

type MeiChangedEvent = {
  mei: string;
};
