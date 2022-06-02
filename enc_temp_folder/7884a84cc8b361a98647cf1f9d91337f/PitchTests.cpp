#include "pch.h"
#include <map>
#include <math.h>

#include "../LibmusCpp/Pitch.h"

using namespace std;
using namespace libmus;

string replace(string text, string match, string replacement) {
	return "aaaaaaaaa";
}


TEST(Pitch, Assert_initial_values_from_constructor_are_being_set) {
	Pitch pitch;

	EXPECT_EQ(1, pitch.Step);
	EXPECT_EQ(0, pitch.Alteration);
	EXPECT_EQ(4, pitch.Octave);
}

TEST(Pitch_Step, Normal_values_should_work) {
	Pitch pitch;

	pitch.Step = 1;
	EXPECT_EQ(1, pitch.Step);

	pitch.Step = 2;
	EXPECT_EQ(2, pitch.Step);

	pitch.Step = 3;
	EXPECT_EQ(3, pitch.Step);

	pitch.Step = 4;
	EXPECT_EQ(4, pitch.Step);

	pitch.Step = 5;
	EXPECT_EQ(5, pitch.Step);

	pitch.Step = 6;
	EXPECT_EQ(6, pitch.Step);

	pitch.Step = 7;
	EXPECT_EQ(7, pitch.Step);
}

TEST(Pitch_Step, Bigger_values_should_be_normalized) {
	Pitch pitch;

	pitch.Step = 8;
	EXPECT_EQ(1, pitch.Step);

	pitch.Step = 9;
	EXPECT_EQ(2, pitch.Step);

	pitch.Step = 10;
	EXPECT_EQ(3, pitch.Step);

	pitch.Step = 11;
	EXPECT_EQ(4, pitch.Step);

	pitch.Step = 12;
	EXPECT_EQ(5, pitch.Step);

	pitch.Step = 13;
	EXPECT_EQ(6, pitch.Step);

	pitch.Step = 14;
	EXPECT_EQ(7, pitch.Step);

	pitch.Step = 15;
	EXPECT_EQ(1, pitch.Step);

}

TEST(Pitch_Step, Zero_or_negative_values_should_do_nothing) {
	Pitch pitch;

	for (auto goodValue = 1; goodValue <= 6; goodValue++) {
		for (auto badValue = -1; badValue >= -20; badValue--) {
			pitch.Step = goodValue;
			pitch.Step = badValue;
			EXPECT_EQ(goodValue, pitch.Step);
		}
	}
}

TEST(Pitch_Alteration, When_value_is_in_range_store_the_same) {
	Pitch pitch;

	pitch.Alteration = -2;
	EXPECT_EQ(-2, pitch.Alteration);

	pitch.Alteration = -1;
	EXPECT_EQ(-1, pitch.Alteration);

	pitch.Alteration = 0;
	EXPECT_EQ(0, pitch.Alteration);

	pitch.Alteration = 1;
	EXPECT_EQ(1, pitch.Alteration);

	pitch.Alteration = 2;
	EXPECT_EQ(2, pitch.Alteration);

}

TEST(Pitch_Alteration, When_value_is_smaller_than_limit_keep_min_value) {
	Pitch pitch;

	for (auto v = -12; v <= -2; v++) {
		pitch.Alteration = 0;
		pitch.Alteration = v;
		EXPECT_EQ(-2, pitch.Alteration);
	}
}

TEST(Pitch_Alteration, When_value_is_greater_than_limit_keep_max_value) {
	Pitch pitch;

	for (auto v = 2; v <= 12; v++) {
		pitch.Alteration = 0;
		pitch.Alteration = v;
		EXPECT_EQ(2, pitch.Alteration);
	}
}

TEST(Pitch_Octave, When_value_is_in_range_store_the_same) {
	Pitch pitch;
	for (auto v = 0; v <= 10; v++) {

		pitch.Octave = v;
		EXPECT_EQ(v, pitch.Octave);
	}
}

TEST(Pitch_Octave, When_value_is_smaller_than_limit_keep_min_value) {
	Pitch pitch;

	for (auto v = -12; v <= 0; v++) {
		pitch.Octave = 0;
		pitch.Octave = v;
		EXPECT_EQ(0, pitch.Octave);
	}
}

TEST(Pitch_Octave, When_value_is_greater_than_limit_keep_max_value) {
	Pitch pitch;

	for (auto v = 10; v <= 25; v++) {
		pitch.Octave = 0;
		pitch.Octave = v;
		EXPECT_EQ(10, pitch.Octave);
	}
}

TEST(Pitch_Random, Steps_should_have_a_good_distribution) {
	map<int, bool> steps;
	for (auto i = 1; i <= 7 * 5; i++) {
		auto  pitch = Pitch::Random();
		steps[pitch.Step] = true;
	}
	for (auto i = 1; i <= 7; i++) {
		EXPECT_TRUE(steps.count(i) && steps[i]);
	}
}

TEST(Pitch_Random, Alterations_should_never_be_double) {
	map<int, bool> alterations;
	for (auto i = 1; i <= 5 * 5; i++) {
		auto pitch = Pitch::Random();
		alterations[pitch.Alteration] = true;
	}
	for (auto i = -1; i <= 1; i++) {
		EXPECT_TRUE(alterations.count(i) && alterations[i]);
	}
	for (auto v : { -2, 2 }) {
		EXPECT_FALSE(alterations[v]);
	}
}

TEST(Pitch_Random, Octaves_should_have_a_good_distribution) {
	map<int, bool> octaves;
	for (auto i = 1; i <= 10 * 5; i++) {
		auto pitch = Pitch::Random();
		octaves[pitch.Octave] = true;
	}
	for (auto i = 0; i <= 10; i++) {
		EXPECT_TRUE(octaves.count(i) && octaves[i]);
	}
}

TEST(Pitch_ExtendedRandom, Steps_should_have_a_good_distribution) {
	map<int, bool> steps;
	for (auto i = 1; i <= 7 * 5; i++) {
		auto pitch = Pitch::ExtendedRandom();
		steps[pitch.Step] = true;
	}
	for (auto i = 1; i <= 7; i++) {
		EXPECT_TRUE(steps.count(i) && steps[i]);
	}
}

TEST(Pitch_ExtendedRandom, Alterations_should_have_a_good_distribution) {
	map<int, bool> alterations;
	for (auto i = 1; i <= 5 * 5; i++) {
		auto pitch = Pitch::ExtendedRandom();
		alterations[pitch.Alteration] = true;
	}
	for (auto i = -2; i <= 2; i++) {
		EXPECT_TRUE(alterations.count(i) && alterations.count(i) && alterations.count(i) && alterations[i]);
	}
}

TEST(Pitch_ExtendedRandom, Octaves_should_have_a_good_distribution) {
	map<int, bool> octaves;
	for (auto i = 1; i <= 10 * 5; i++) {
		auto pitch = Pitch::ExtendedRandom();
		octaves[pitch.Octave] = true;
	}
	for (auto i = 0; i <= 10; i++) {
		EXPECT_TRUE(octaves.count(i) && octaves[i]);
	}
}

TEST(Pitch_SimpleName, Without_alterations) {
	Pitch pitch;
	pitch.Alteration = 0;
	vector<string> names = { "C?", "D?", "E?", "F?", "G?", "A?", "B?" };
	for (auto o = 0; o <= 10; o++) {
		pitch.Octave = o;
		for (auto i = 1; i <= 7; i++) {
			pitch.Step = i;
			auto expected = replace(names[i - 1], "?", to_string(pitch.Octave));
			auto actual = pitch.Name();
			EXPECT_EQ(expected, actual);
		}
	}
}

TEST(Pitch_SimpleName, With_1_flat) {
	Pitch pitch;
	pitch.Alteration = -1;
	vector<string> names = { "Cb?", "Db?", "Eb?", "Fb?", "Gb?", "Ab?", "Bb?" };
	for (auto o = 0; o <= 10; o++) {
		pitch.Octave = o;
		for (auto i = 1; i <= 7; i++) {
			pitch.Step = i;
			auto expected = replace(names[i - 1], "?", to_string(pitch.Octave));
			auto actual = pitch.Name();
			EXPECT_EQ(expected, actual);
		}
	}
}

TEST(Pitch_SimpleName, With_2_flats) {
	Pitch pitch;
	pitch.Alteration = -2;
	vector<string> names = { "Cbb?", "Dbb?", "Ebb?", "Fbb?", "Gbb?", "Abb?", "Bbb?" };
	for (auto o = 0; o <= 10; o++) {
		pitch.Octave = o;
		for (auto i = 1; i <= 7; i++) {
			pitch.Step = i;
			auto expected = replace(names[i - 1], "?", to_string(pitch.Octave));
			auto actual = pitch.Name();
			EXPECT_EQ(expected, actual);
		}
	}
}

TEST(Pitch_SimpleName, With_1_sharp) {
	Pitch pitch;
	pitch.Alteration = 1;
	vector<string> names = { "C#?", "D#?", "E#?", "F#?", "G#?", "A#?", "B#?" };
	for (auto o = 0; o <= 10; o++) {
		pitch.Octave = o;
		for (auto i = 1; i <= 7; i++) {
			pitch.Step = i;
			auto expected = replace(names[i - 1], "?", to_string(pitch.Octave));
			auto actual = pitch.Name();
			EXPECT_EQ(expected, actual);
		}
	}
}

TEST(Pitch_SimpleName, With_2_sharps) {
	Pitch pitch;
	pitch.Alteration = 2;
	vector<string> names = { "C##?", "D##?", "E##?", "F##?", "G##?", "A##?", "B##?" };
	for (auto o = 0; o <= 10; o++) {
		pitch.Octave = o;
		for (auto i = 1; i <= 7; i++) {
			pitch.Step = i;
			auto expected = replace(names[i - 1], "?", to_string(pitch.Octave));
			auto actual = pitch.Name();
			EXPECT_EQ(expected, actual);
		}
	}
}

vector<string> superscripts = { "⁰", "¹", "²", "³", "⁴", "⁵", "⁶", "⁷", "⁸", "⁹", "¹⁰" };
TEST(Pitch_PrettyName, Without_alterations) {
	Pitch pitch;
	pitch.Alteration = 0;
	vector<string> names = { "C?", "D?", "E?", "F?", "G?", "A?", "B?" };
	for (auto o = 0; o <= 10; o++) {
		pitch.Octave = o;
		for (auto i = 1; i <= 7; i++) {
			pitch.Step = i;
			auto expected = replace(names[i - 1], "?", superscripts[pitch.Octave]);
			auto actual = pitch.PrettyName();
			EXPECT_EQ(expected, actual);
		}
	}
}

TEST(Pitch_PrettyName, With_1_flat) {
	Pitch pitch;
	pitch.Alteration = -1;
	vector<string> names = { "C♭?", "D♭?", "E♭?", "F♭?", "G♭?", "A♭?", "B♭?" };
	for (auto o = 0; o <= 10; o++) {
		pitch.Octave = o;
		for (auto i = 1; i <= 7; i++) {
			pitch.Step = i;
			auto expected = replace(names[i - 1], "?", superscripts[pitch.Octave]);
			auto actual = pitch.PrettyName();
			EXPECT_EQ(expected, actual);
		}
	}
}

TEST(Pitch_PrettyName, With_2_flats) {
	Pitch pitch;
	pitch.Alteration = -2;
	vector<string> names = { "C♭♭?", "D♭♭?", "E♭♭?", "F♭♭?", "G♭♭?", "A♭♭?", "B♭♭?" };
	for (auto o = 0; o <= 10; o++) {
		pitch.Octave = o;
		for (auto i = 1; i <= 7; i++) {
			pitch.Step = i;
			auto expected = replace(names[i - 1], "?", superscripts[pitch.Octave]);
			auto actual = pitch.PrettyName();
			EXPECT_EQ(expected, actual);
		}
	}
}

TEST(Pitch_PrettyName, With_1_sharp) {
	Pitch pitch;
	pitch.Alteration = 1;
	vector<string> names = { "C♯?", "D♯?", "E♯?", "F♯?", "G♯?", "A♯?", "B♯?" };
	for (auto o = 0; o <= 10; o++) {
		pitch.Octave = o;
		for (auto i = 1; i <= 7; i++) {
			pitch.Step = i;
			auto expected = replace(names[i - 1], "?", superscripts[pitch.Octave]);
			auto actual = pitch.PrettyName();
			EXPECT_EQ(expected, actual);
		}
	}
}

TEST(Pitch_PrettyName, With_2_sharps) {
	Pitch pitch;
	pitch.Alteration = 2;
	vector<string> names = { "C♯♯?", "D♯♯?", "E♯♯?", "F♯♯?", "G♯♯?", "A♯♯?", "B♯♯?" };
	for (auto o = 0; o <= 10; o++) {
		pitch.Octave = o;
		for (auto i = 1; i <= 7; i++) {
			pitch.Step = i;
			auto expected = replace(names[i - 1], "?", superscripts[pitch.Octave]);
			auto actual = pitch.PrettyName();
			EXPECT_EQ(expected, actual);
		}
	}
}

TEST(Pitch_FullName, Without_alterations) {
	Pitch pitch;
	pitch.Alteration = 0;
	vector<string> names = { "C ?", "D ?", "E ?", "F ?", "G ?", "A ?", "B ?" };
	for (auto i = 1; i <= 7; i++) {
		pitch.Step = i;
		auto expected = replace(names[i - 1], "?", to_string(pitch.Octave));
		auto actual = pitch.FullName();
		EXPECT_EQ(expected, actual);
	}
}

TEST(Pitch_FullName, With_1_flat) {
	Pitch pitch;
	pitch.Alteration = -1;
	vector<string> names = { "C flat ?", "D flat ?", "E flat ?", "F flat ?", "G flat ?", "A flat ?", "B flat ?" };
	for (auto i = 1; i <= 7; i++) {
		pitch.Step = i;
		auto expected = replace(names[i - 1], "?", to_string(pitch.Octave));
		auto actual = pitch.FullName();
		EXPECT_EQ(expected, actual);
	}
}

TEST(Pitch_FullName, With_2_flats) {
	Pitch pitch;
	pitch.Alteration = -2;
	vector<string> names = { "C double flat ?", "D double flat ?", "E double flat ?", "F double flat ?", "G double flat ?",
	"A double flat ?", "B double flat ?" };
	for (auto i = 1; i <= 7; i++) {
		pitch.Step = i;
		auto expected = replace(names[i - 1], "?", to_string(pitch.Octave));
		auto actual = pitch.FullName();
		EXPECT_EQ(expected, actual);
	}
}

TEST(Pitch_FullName, With_1_sharp) {
	Pitch pitch;
	pitch.Alteration = 1;
	vector<string> names = { "C sharp ?", "D sharp ?", "E sharp ?", "F sharp ?", "G sharp ?", "A sharp ?", "B sharp ?" };
	for (auto i = 1; i <= 7; i++) {
		pitch.Step = i;
		auto expected = replace(names[i - 1], "?", to_string(pitch.Octave));
		auto actual = pitch.FullName();
		EXPECT_EQ(expected, actual);
	}
}

TEST(Pitch_FullName, With_2_sharps) {
	Pitch pitch;
	pitch.Alteration = 2;
	vector<string> names = {
		"C double sharp ?",
		"D double sharp ?",
		"E double sharp ?",
		"F double sharp ?",
		"G double sharp ?",
		"A double sharp ?",
		"B double sharp ?"
	};

	for (auto i = 1; i <= 7; i++) {
		pitch.Step = i;
		auto expected = replace(names[i - 1], "?", to_string(pitch.Octave));
		auto actual = pitch.FullName();
		EXPECT_EQ(expected, actual);
	}
}

TEST(Pitch_Parse_without_octave, Simple_name_without_octave) {
	vector<string> alterationTexts = { "bb", "b", "", "#", "##" };
	vector<string> stepTexts = { "C", "D", "E", "F", "G", "A", "B" };

	for (auto iterAlt = alterationTexts.begin(); iterAlt != alterationTexts.end(); iterAlt++) {
		auto ialt = iterAlt - alterationTexts.begin();
		auto alt = *iterAlt;

		for (auto iterStep = stepTexts.begin(); iterStep != stepTexts.end(); iterStep++) {
			auto istep = iterStep - stepTexts.begin();
			auto step = *iterStep;

			auto text = step + alt;
			auto parsed = Pitch::Parse(text);

			EXPECT_EQ(istep, parsed.Step);
			EXPECT_EQ(ialt - 3, parsed.Alteration);
			EXPECT_EQ(4, parsed.Octave);
		}
	}
}

TEST(Pitch_Parse_without_octave, Pretty_name) {
	vector<string> alterationTexts = { "♭♭", "♭", "", "♯", "♯♯" };
	vector<string> stepTexts = { "C", "D", "E", "F", "G", "A", "B" };

	for (auto iterAlt = alterationTexts.begin(); iterAlt != alterationTexts.end(); iterAlt++) {
		auto ialt = iterAlt - alterationTexts.begin();
		auto alt = *iterAlt;

		for (auto iterStep = stepTexts.begin(); iterStep != stepTexts.end(); iterStep++) {
			auto istep = iterStep - stepTexts.begin();
			auto step = *iterStep;

			auto text = step + alt;
			auto parsed = Pitch::Parse(text);

			EXPECT_EQ(istep, parsed.Step);
			EXPECT_EQ(ialt - 3, parsed.Alteration);
			EXPECT_EQ(4, parsed.Octave);
		}
	}
}
TEST(Pitch_Parse_with_octave, Simple_name) {
	vector<string> alterationTexts = { "bb", "b", "", "#", "##" };
	vector<string> stepTexts = { "C", "D", "E", "F", "G", "A", "B" };


	for (auto oct = 0; oct <= 10; oct++) {
		for (auto iterAlt = alterationTexts.begin(); iterAlt != alterationTexts.end(); iterAlt++) {
			auto ialt = iterAlt - alterationTexts.begin();
			auto alt = *iterAlt;

			for (auto iterStep = stepTexts.begin(); iterStep != stepTexts.end(); iterStep++) {
				auto istep = iterStep - stepTexts.begin();
				auto step = *iterStep;


				auto text = step + alt + to_string(oct);
				auto parsed = Pitch::Parse(text);

				EXPECT_EQ(istep, parsed.Step);
				EXPECT_EQ(ialt - 3, parsed.Alteration);
				EXPECT_EQ(oct, parsed.Octave);
			}
		}
	}
}
TEST(Pitch_Parse_with_octave, Pretty_name) {
	vector<string> alterationTexts = { "♭♭", "♭", "", "♯", "♯♯" };
	vector<string> stepTexts = { "C", "D", "E", "F", "G", "A", "B" };

	for (auto oct = 0; oct <= 10; oct++) {

		for (auto iterAlt = alterationTexts.begin(); iterAlt != alterationTexts.end(); iterAlt++) {
			auto ialt = iterAlt - alterationTexts.begin();
			auto alt = *iterAlt;

			for (auto iterStep = stepTexts.begin(); iterStep != stepTexts.end(); iterStep++) {
				auto istep = iterStep - stepTexts.begin();
				auto step = *iterStep;

				auto text = step + alt + to_string(oct);
				auto parsed = Pitch::Parse(text);

				EXPECT_EQ(istep, parsed.Step);
				EXPECT_EQ(ialt - 3, parsed.Alteration);
				EXPECT_EQ(oct, parsed.Octave);
			}
		}
	}
}