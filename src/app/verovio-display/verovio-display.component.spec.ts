import { ComponentFixture, TestBed } from '@angular/core/testing';

import { VerovioDisplayComponent } from './verovio-display.component';

describe('VerovioDisplayComponent', () => {
  let component: VerovioDisplayComponent;
  let fixture: ComponentFixture<VerovioDisplayComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [VerovioDisplayComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(VerovioDisplayComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
