import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { invoke } from '@tauri-apps/api/core';
// import verovio from 'verovio/wasm';
import verovio from 'verovio/esm/index.mjs';

@Component({
  selector: 'app-root',
  imports: [FormsModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  greetMessage = "";
  name = "";

  async ngOnInit() {
    // let v = await verovio()
    // v.
  }


  async submitForm(e: SubmitEvent) {
    e.preventDefault();
    this.greetMessage = await invoke("greet", { name: this.name });
  }
}
