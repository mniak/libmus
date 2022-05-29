#pragma once
class PitchClass
{
private:
	int step;
	int alteration;

public:
	PitchClass();

	__declspec(property(get = GetStep, put = SetStep)) int Step;
	int GetStep();
	void SetStep(int value);

	__declspec(property(get = GetAlteration, put = SetAlteration)) int Alteration;
	int GetAlteration();
	void SetAlteration(int value);
};

