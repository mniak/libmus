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
}