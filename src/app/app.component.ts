import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { invoke } from '@tauri-apps/api/core';

@Component({
  selector: 'app-root',
  imports: [FormsModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  greetMessage = "";
  name = "";

  async submitForm(e: SubmitEvent) {
    e.preventDefault();
    this.greetMessage = await invoke("greet", { name: this.name });
  }
}
