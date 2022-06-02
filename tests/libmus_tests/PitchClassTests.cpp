#include <gtest/gtest.h>
#include <libmus/PitchClass.h>
#include <map>

using namespace std;
using namespace libmus;

TEST(PitchClass_Step, Constructor_should_set_initial_values) {
    PitchClass pc;

    EXPECT_EQ(1, pc.Step);
    EXPECT_EQ(0, pc.Alteration);
}

TEST(PitchClass_Step, Normal_values_should_work) {
    PitchClass pc;

    pc.Step = 1;
    EXPECT_EQ(1, pc.Step);

    pc.Step = 2;
    EXPECT_EQ(2, pc.Step);

    pc.Step = 3;
    EXPECT_EQ(3, pc.Step);

    pc.Step = 4;
    EXPECT_EQ(4, pc.Step);

    pc.Step = 5;
    EXPECT_EQ(5, pc.Step);

    pc.Step = 6;
    EXPECT_EQ(6, pc.Step);

    pc.Step = 7;
    EXPECT_EQ(7, pc.Step);
}

TEST(PitchClass_Step, Bigger_values_should_be_normalized) {
    PitchClass pc;

    pc.Step = 8;
    EXPECT_EQ(1, pc.Step);

    pc.Step = 9;
    EXPECT_EQ(2, pc.Step);

    pc.Step = 10;
    EXPECT_EQ(3, pc.Step);

    pc.Step = 11;
    EXPECT_EQ(4, pc.Step);

    pc.Step = 12;
    EXPECT_EQ(5, pc.Step);

    pc.Step = 13;
    EXPECT_EQ(6, pc.Step);

    pc.Step = 14;
    EXPECT_EQ(7, pc.Step);

    pc.Step = 15;
    EXPECT_EQ(1, pc.Step);
}

TEST(PitchClass_Step, Attributing_zero_or_negative_should_do_nothing) {
    PitchClass pc;

    for (auto goodValue = 1; goodValue <= 6; goodValue++) {
        for (auto badValue = -1; badValue >= -20; badValue--) {
            pc.Step = goodValue;
            pc.Step = badValue;
            EXPECT_EQ(goodValue, pc.Step);
        }
    }
}

TEST(PitchClass_Alteration, When_value_is_in_range_store_the_same) {
    PitchClass pc;

    pc.Alteration = -2;
    EXPECT_EQ(-2, pc.Alteration);

    pc.Alteration = -1;
    EXPECT_EQ(-1, pc.Alteration);

    pc.Alteration = 0;
    EXPECT_EQ(0, pc.Alteration);

    pc.Alteration = 1;
    EXPECT_EQ(1, pc.Alteration);

    pc.Alteration = 2;
    EXPECT_EQ(2, pc.Alteration);
}

TEST(PitchClass_Alteration, When_value_is_smaller_than_limit_then_keep_min_value) {
    PitchClass pc;

    for (auto v = -12; v <= -2; v++) {
        pc.Alteration = 0;
        pc.Alteration = v;
        EXPECT_EQ(-2, pc.Alteration);
    }
}

TEST(PitchClass_Alteration, When_value_is_greater_than_limit_then_keep_max_value) {
    PitchClass pc;

    for (auto v = 2; v <= 12; v++) {
        pc.Alteration = 0;
        pc.Alteration = v;
        EXPECT_EQ(2, pc.Alteration);
    }
}

TEST(PitchClass_Random, Steps_should_have_a_good_distribution) {
    map<int, bool> steps;
    for (auto i = 1; i <= 7 * 5; i++) {
        auto pc = PitchClass::Random();
        steps[pc.Step] = true;
    }
    for (auto i = 1; i <= 7; i++) {
        auto pc = PitchClass::Random();
        EXPECT_TRUE(steps[i]);
    }
}

TEST(PitchClass_Random, Alterations_should_never_be_double) {
    map<int, bool> alterations;
    for (auto i = 1; i <= 3 * 5; i++) {
        auto pc = PitchClass::Random();
        alterations[pc.Alteration] = true;
    }
    for (auto i = -1; i <= 1; i++) {
        EXPECT_TRUE(alterations[i]);
    }
    for (auto v : {-2, 2}) {
        EXPECT_FALSE(alterations[v]);
    }
}

TEST(PitchClass_ExtendedRandom, Steps_should_have_a_good_distribution) {
    map<int, bool> steps;
    for (auto i = 1; i <= 7 * 5; i++) {
        auto pc = PitchClass::ExtendedRandom();
        steps[pc.Step] = true;
    }
    for (auto i = 1; i <= 7; i++) {
        auto pc = PitchClass::ExtendedRandom();
        EXPECT_TRUE(steps[i]);
    }
}

TEST(PitchClass_ExtendedRandom, Alterations_should_have_a_good_distribution) {
    map<int, bool> alterations;
    for (auto i = 1; i <= 5 * 5; i++) {
        auto pc = PitchClass::ExtendedRandom();
        alterations[pc.Alteration] = true;
    }
    for (auto i = -2; i <= 2; i++) {
        EXPECT_TRUE(alterations[i]);
    }
}

TEST(PitchClass_SimpleName, Without_alterations) {
    PitchClass pc;
    pc.Alteration = 0;
    vector<wstring> names = {L"C", L"D", L"E", L"F", L"G", L"A", L"B"};
    for (auto i = 1; i <= 7; i++) {
        pc.Step = i;
        auto expected = names[i - 1];
        auto actual = pc.Name();
        EXPECT_EQ(expected.c_str(), actual.c_str());
    }
}

// TEST(PitchClass_SimpleName, With_1_flat) {
//	PitchClass pc;
//	pc.Alteration = -1;
//	vector<wstring> names = { "Cb", "Db", "Eb", "Fb", "Gb", "Ab", "Bb" };
//	for (auto i = 1; i <= 7; i++) {
//		pc.Step = i;
//		auto	expected = names[i - 1];
//		auto actual = pc.Name();
//		EXPECT_EQ(expected, actual);
//	}
// }
//
// TEST(PitchClass_SimpleName, With_2_flats) {
//	PitchClass pc;
//	pc.Alteration = -2;
//	vector<wstring> names = { "Cbb", "Dbb", "Ebb", "Fbb", "Gbb", "Abb", "Bbb"
//}; 	for (auto i = 1; i <= 7; i++) { 		pc.Step = i;
// auto	expected = names[i - 1]; 		auto actual = pc.Name();
// EXPECT_EQ(expected, actual);
//	}
// }
//
// TEST(PitchClass_SimpleName, With_1_sharp) {
//	PitchClass pc;
//	pc.Alteration = 1;
//	vector<wstring> names = { "C#", "D#", "E#", "F#", "G#", "A#", "B#" };
//	for (auto i = 1; i <= 7; i++) {
//		pc.Step = i;
//		auto	expected = names[i - 1];
//		auto actual = pc.Name();
//		EXPECT_EQ(expected, actual);
//	}
// }
//
// TEST(PitchClass_SimpleName, With_2_sharps) {
//	PitchClass pc;
//	pc.Alteration = 2;
//	vector<wstring> names = { "C##", "D##", "E##", "F##", "G##", "A##", "B##"
//}; 	for (auto i = 1; i <= 7; i++) { 		pc.Step = i;
// auto	expected = names[i - 1]; 		auto actual = pc.Name();
// EXPECT_EQ(expected, actual);
//	}
// }
//
// TEST(PitchClass_PrettyName, Without_alterations) {
//	PitchClass pc;
//	pc.Alteration = 0;
//	vector<wstring> names = { "C", "D", "E", "F", "G", "A", "B" };
//	for (auto i = 1; i <= 7; i++) {
//		pc.Step = i;
//		auto	expected = names[i - 1];
//		auto actual = pc.PrettyName();
//		EXPECT_EQ(expected, actual);
//	}
// }
//
// TEST(PitchClass_PrettyName, With_1_flat) {
//	PitchClass pc;
//	pc.Alteration = -1;
//	vector<wstring> names = { "C♭", "D♭", "E♭", "F♭", "G♭", "A♭", "B♭" };
//	for (auto i = 1; i <= 7; i++) {
//		pc.Step = i;
//		auto	expected = names[i - 1];
//		auto actual = pc.PrettyName();
//		EXPECT_EQ(expected, actual);
//	}
// }
//
// TEST(PitchClass_PrettyName, With_2_flats) {
//	PitchClass pc;
//	pc.Alteration = -2;
//	vector<wstring> names = { "C𝄫", "D𝄫", "E𝄫", "F𝄫", "G𝄫", "A𝄫", "B𝄫" };
//	for (auto i = 1; i <= 7; i++) {
//		pc.Step = i;
//		auto	expected = names[i - 1];
//		auto actual = pc.PrettyName();
//		EXPECT_EQ(expected, actual);
//	}
// }
//
// TEST(PitchClass_PrettyName, With_1_sharp) {
//	PitchClass pc;
//	pc.Alteration = 1;
//	vector<wstring> names = { "C♯", "D♯", "E♯", "F♯", "G♯", "A♯", "B♯" };
//	for (auto i = 1; i <= 7; i++) {
//		pc.Step = i;
//		auto	expected = names[i - 1];
//		auto actual = pc.PrettyName();
//		EXPECT_EQ(expected, actual);
//	}
// }
//
// TEST(PitchClass_PrettyName, With_2_sharps) {
//	PitchClass pc;
//	pc.Alteration = 2;
//	vector<wstring> names = { "C𝄪", "D𝄪", "E𝄪", "F𝄪", "G𝄪", "A𝄪", "B𝄪" };
//	for (auto i = 1; i <= 7; i++) {
//		pc.Step = i;
//		auto	expected = names[i - 1];
//		auto actual = pc.PrettyName();
//		EXPECT_EQ(expected, actual);
//	}
// }

// TEST(PitchClass_FullName, Without_alterations) {
//	PitchClass pc;
//	pc.Alteration = 0;
//	vector<wstring> names = { "C", "D", "E", "F", "G", "A", "B" };
//	for (auto i = 1; i <= 7; i++) {
//		pc.Step = i;
//		auto	expected = names[i - 1];
//		auto actual = pc.FullName();
//		EXPECT_EQ(expected, actual);
//	}
// }
//
// TEST(PitchClass_FullName, With_1_flat) {
//	PitchClass pc;
//	pc.Alteration = -1;
//	vector<wstring> names = { "C flat", "D flat", "E flat", "F flat", "G
// flat", "A flat", "B flat" }; 	for (auto i = 1; i <= 7; i++) {
// pc.Step = i; 		auto
// expected = names[i - 1]; 		auto actual = pc.FullName();
// EXPECT_EQ(expected, actual);
//	}
// }
//
// TEST(PitchClass_FullName, With_2_flats) {
//	PitchClass pc;
//	pc.Alteration = -2;
//	vector<wstring> names = { "C double flat", "D double flat", "E double
// flat", "F double flat", "G double flat", "A double flat", "B double flat" };
// for (auto i = 1; i <= 7; i++) { 		pc.Step = i; 		auto
// expected = names[i - 1]; 		auto actual = pc.FullName();
// EXPECT_EQ(expected, actual);
//	}
// }
//
// TEST(PitchClass_FullName, With_1_sharp) {
//	PitchClass pc;
//	pc.Alteration = 1;
//	vector<wstring> names = { "C sharp", "D sharp", "E sharp", "F sharp", "G
// sharp", "A sharp", "B sharp" }; 	for (auto i = 1; i <= 7; i++) {
// pc.Step = i; 		auto	expected = names[i - 1];
// auto actual = pc.FullName();
//		EXPECT_EQ(expected, actual);
//	}
// }
//
// TEST(PitchClass_FullName, With_2_sharps) {
//	PitchClass pc;
//	pc.Alteration = 2;
//	vector<wstring> names = { "C double sharp", "D double sharp", "E double
// sharp", "F double sharp", "G double sharp", 			 "A double
// sharp", "B double sharp"
//}; 	for (auto i = 1; i <= 7; i++) { 		pc.Step = i;
// auto	expected = names[i - 1]; 		auto actual = pc.FullName();
//		EXPECT_EQ(expected, actual);
//	}
// }
//
// TEST(PitchClass_Parse, Simple_name) {
//	vector<wstring> alterationTexts = { L"bb", L"b", L"", L"#", L"##" };
//	vector<wstring> stepTexts = { L"C", L"D", L"E", L"F", L"G", L"A", L"B"
//};
//
//	for (auto ialt = 0; ialt < alterationTexts.size(); ialt++) {
//		auto alt = alterationTexts[ialt];
//
//		for (auto istep = 0; istep < stepTexts.size(); istep++) {
//			auto step = stepTexts[istep];
//
//			auto text = step + alt;
//			auto parsed = PitchClass::Parse(text);
//
//			EXPECT_EQ(istep, parsed.Step);
//			EXPECT_EQ(ialt - 3, parsed.Alteration);
//		}
//	}
// }
// TEST(PitchClass_Parse, Pretty_name) {
//	vector<wstring> alterationTexts = { L"𝄫", L"♭", L"", L"♯", L"𝄪" };
//	vector<wstring> stepTexts = { L"C", L"D", L"E", L"F", L"G", L"A", L"B"
//};
//
//	for (auto ialt = 0; ialt < alterationTexts.size(); ialt++) {
//		auto alt = alterationTexts[ialt];
//
//		for (auto istep = 0; istep < stepTexts.size(); istep++) {
//			auto step = stepTexts[istep];
//
//			auto text = step + alt;
//			auto parsed = PitchClass::Parse(text);
//
//			EXPECT_EQ(istep, parsed.Step);
//			EXPECT_EQ(ialt - 3, parsed.Alteration);
//		}
//	}
// }
