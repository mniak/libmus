﻿#include <gtest/gtest.h>
#include <libmus/libmus.h>

#include <map>

using namespace std;
using namespace libmus;

TEST(PitchClass_Step, Constructor_should_set_initial_values) {
    PitchClass pc;

    EXPECT_EQ(1, pc.GetStep());
    EXPECT_EQ(0, pc.GetAlteration());
}

TEST(PitchClass_Step, Normal_values_should_work) {
    PitchClass pc;

    pc.SetStep(1);
    EXPECT_EQ(1, pc.GetStep());

    pc.SetStep(2);
    EXPECT_EQ(2, pc.GetStep());

    pc.SetStep(3);
    EXPECT_EQ(3, pc.GetStep());

    pc.SetStep(4);
    EXPECT_EQ(4, pc.GetStep());

    pc.SetStep(5);
    EXPECT_EQ(5, pc.GetStep());

    pc.SetStep(6);
    EXPECT_EQ(6, pc.GetStep());

    pc.SetStep(7);
    EXPECT_EQ(7, pc.GetStep());
}

TEST(PitchClass_Step, Bigger_values_should_be_normalized) {
    PitchClass pc;

    pc.SetStep(8);
    EXPECT_EQ(1, pc.GetStep());

    pc.SetStep(9);
    EXPECT_EQ(2, pc.GetStep());

    pc.SetStep(10);
    EXPECT_EQ(3, pc.GetStep());

    pc.SetStep(11);
    EXPECT_EQ(4, pc.GetStep());

    pc.SetStep(12);
    EXPECT_EQ(5, pc.GetStep());

    pc.SetStep(13);
    EXPECT_EQ(6, pc.GetStep());

    pc.SetStep(14);
    EXPECT_EQ(7, pc.GetStep());

    pc.SetStep(15);
    EXPECT_EQ(1, pc.GetStep());
}

TEST(PitchClass_Step, Attributing_zero_or_negative_should_do_nothing) {
    PitchClass pc;

    for (auto goodValue = 1; goodValue <= 6; goodValue++) {
        for (auto badValue = -1; badValue >= -20; badValue--) {
            pc.SetStep(goodValue);
            pc.SetStep(badValue);
            EXPECT_EQ(goodValue, pc.GetStep());
        }
    }
}

TEST(PitchClass_Alteration, When_value_is_in_range_store_the_same) {
    PitchClass pc;

    pc.SetAlteration(-2);
    EXPECT_EQ(-2, pc.GetAlteration());

    pc.SetAlteration(-1);
    EXPECT_EQ(-1, pc.GetAlteration());

    pc.SetAlteration(0);
    EXPECT_EQ(0, pc.GetAlteration());

    pc.SetAlteration(1);
    EXPECT_EQ(1, pc.GetAlteration());

    pc.SetAlteration(2);
    EXPECT_EQ(2, pc.GetAlteration());
}

TEST(PitchClass_Alteration, When_value_is_smaller_than_limit_then_keep_min_value) {
    PitchClass pc;

    for (auto v = -12; v <= -2; v++) {
        pc.SetAlteration(0);
        pc.SetAlteration(v);
        EXPECT_EQ(-2, pc.GetAlteration());
    }
}

TEST(PitchClass_Alteration, When_value_is_greater_than_limit_then_keep_max_value) {
    PitchClass pc;

    for (auto v = 2; v <= 12; v++) {
        pc.SetAlteration(0);
        pc.SetAlteration(v);
        EXPECT_EQ(2, pc.GetAlteration());
    }
}

TEST(PitchClass_Random, Steps_should_have_a_good_distribution) {
    map<int, bool> steps;
    for (auto i = 1; i <= 7 * 5; i++) {
        auto pc = PitchClass::Random();
        steps[pc.GetStep()] = true;
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
        alterations[pc.GetAlteration()] = true;
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
        steps[pc.GetStep()] = true;
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
        alterations[pc.GetAlteration()] = true;
    }
    for (auto i = -2; i <= 2; i++) {
        EXPECT_TRUE(alterations[i]);
    }
}

TEST(PitchClass_SimpleName, Without_alterations) {
    PitchClass pc;
    pc.SetAlteration(0);
    vector<wstring> names = {L"C", L"D", L"E", L"F", L"G", L"A", L"B"};
    for (auto i = 1; i <= 7; i++) {
        pc.SetStep(i);
        auto expected = names[i - 1];
        auto actual = pc.Name();
        EXPECT_STREQ(expected.c_str(), actual.c_str());
    }
}

TEST(PitchClass_SimpleName, With_1_flat) {
    PitchClass pc;
    pc.SetAlteration(-1);
    vector<wstring> names = {L"Cb", L"Db", L"Eb", L"Fb", L"Gb", L"Ab", L"Bb"};
    for (auto i = 1; i <= 7; i++) {
        pc.SetStep(i);
        auto expected = names[i - 1];
        auto actual = pc.Name();
        EXPECT_STREQ(expected.c_str(), actual.c_str());
    }
}

TEST(PitchClass_SimpleName, With_2_flats) {
    PitchClass pc;
    pc.SetAlteration(-2);
    vector<wstring> names = {L"Cbb", L"Dbb", L"Ebb", L"Fbb", L"Gbb", L"Abb", L"Bbb"};
    for (auto i = 1; i <= 7; i++) {
        pc.SetStep(i);
        auto expected = names[i - 1];
        auto actual = pc.Name();
        EXPECT_EQ(expected, actual);
    }
}

TEST(PitchClass_SimpleName, With_1_sharp) {
    PitchClass pc;
    pc.SetAlteration(1);
    vector<wstring> names = {L"C#", L"D#", L"E#", L"F#", L"G#", L"A#", L"B#"};
    for (auto i = 1; i <= 7; i++) {
        pc.SetStep(i);
        auto expected = names[i - 1];
        auto actual = pc.Name();
        EXPECT_EQ(expected, actual);
    }
}

TEST(PitchClass_SimpleName, With_2_sharps) {
    PitchClass pc;
    pc.SetAlteration(2);
    vector<wstring> names = {L"C##", L"D##", L"E##", L"F##", L"G##", L"A##", L"B##"};
    for (auto i = 1; i <= 7; i++) {
        pc.SetStep(i);
        auto expected = names[i - 1];
        auto actual = pc.Name();
        EXPECT_EQ(expected, actual);
    }
}

TEST(PitchClass_PrettyName, Without_alterations) {
    PitchClass pc;
    pc.SetAlteration(0);
    vector<wstring> names = {L"C", L"D", L"E", L"F", L"G", L"A", L"B"};
    for (auto i = 1; i <= 7; i++) {
        pc.SetStep(i);
        auto expected = names[i - 1];
        auto actual = pc.PrettyName();
        EXPECT_EQ(expected, actual);
    }
}

TEST(PitchClass_PrettyName, With_1_flat) {
    PitchClass pc;
    pc.SetAlteration(-1);
    vector<wstring> names = {L"C♭", L"D♭", L"E♭", L"F♭", L"G♭", L"A♭", L"B♭"};
    for (auto i = 1; i <= 7; i++) {
        pc.SetStep(i);
        auto expected = names[i - 1];
        auto actual = pc.PrettyName();
        EXPECT_EQ(expected, actual);
    }
}

TEST(PitchClass_PrettyName, With_2_flats) {
    PitchClass pc;
    pc.SetAlteration(-2);
    vector<wstring> names = {L"C𝄫", L"D𝄫", L"E𝄫", L"F𝄫", L"G𝄫", L"A𝄫", L"B𝄫"};
    for (auto i = 1; i <= 7; i++) {
        pc.SetStep(i);
        auto expected = names[i - 1];
        auto actual = pc.PrettyName();
        EXPECT_EQ(expected, actual);
    }
}

TEST(PitchClass_PrettyName, With_1_sharp) {
    PitchClass pc;
    pc.SetAlteration(1);
    vector<wstring> names = {L"C♯", L"D♯", L"E♯", L"F♯", L"G♯", L"A♯", L"B♯"};
    for (auto i = 1; i <= 7; i++) {
        pc.SetStep(i);
        auto expected = names[i - 1];
        auto actual = pc.PrettyName();
        EXPECT_EQ(expected, actual);
    }
}

TEST(PitchClass_PrettyName, With_2_sharps) {
    PitchClass pc;
    pc.SetAlteration(2);
    vector<wstring> names = {L"C𝄪", L"D𝄪", L"E𝄪", L"F𝄪", L"G𝄪", L"A𝄪", L"B𝄪"};
    for (auto i = 1; i <= 7; i++) {
        pc.SetStep(i);
        auto expected = names[i - 1];
        auto actual = pc.PrettyName();
        EXPECT_EQ(expected, actual);
    }
}

TEST(PitchClass_FullName, Without_alterations) {
    PitchClass pc;
    pc.SetAlteration(0);
    vector<wstring> names = {L"C", L"D", L"E", L"F", L"G", L"A", L"B"};
    for (auto i = 1; i <= 7; i++) {
        pc.SetStep(i);
        auto expected = names[i - 1];
        auto actual = pc.FullName();
        EXPECT_EQ(expected, actual);
    }
}

TEST(PitchClass_FullName, With_1_flat) {
    PitchClass pc;
    pc.SetAlteration(-1);
    vector<wstring> names = {L"C flat", L"D flat", L"E flat", L"F flat", L"G flat", L"A flat L", L"B flat"};
    for (auto i = 1; i <= 7; i++) {
        pc.SetStep(i);
        auto expected = names[i - 1];
        auto actual = pc.FullName();
        EXPECT_EQ(expected, actual);
    }
}

TEST(PitchClass_FullName, With_2_flats) {
    PitchClass pc;
    pc.SetAlteration(-2);
    vector<wstring> names = {L"C double flat", L"D double flat", L"E double flat", L"F double flat",
                             L"G double flat", L"A double flat", L"B double flat"};
    for (auto i = 1; i <= 7; i++) {
        pc.SetStep(i);
        auto expected = names[i - 1];
        auto actual = pc.FullName();
        EXPECT_EQ(expected, actual);
    }
}

TEST(PitchClass_FullName, With_1_sharp) {
    PitchClass pc;
    pc.SetAlteration(1);
    vector<wstring> names = {L"C sharp", L"D sharp", L"E sharp", L"F sharp", L"G sharp", L" A sharp", L"B sharp"};
    for (auto i = 1; i <= 7; i++) {
        pc.SetStep(i);
        auto expected = names[i - 1];
        auto actual = pc.FullName();
        EXPECT_EQ(expected, actual);
    }
}

TEST(PitchClass_FullName, With_2_sharps) {
    PitchClass pc;
    pc.SetAlteration(2);
    vector<wstring> names = {L"C double sharp",  L"D double sharp",  L"E double sharp", L"F double sharp",
                             L" G double sharp", L" A double sharp", L"B double sharp"};
    for (auto i = 1; i <= 7; i++) {
        pc.SetStep(i);
        auto expected = names[i - 1];
        auto actual = pc.FullName();
        EXPECT_EQ(expected, actual);
    }
}

TEST(PitchClass_Parse, Simple_name) {
    vector<wstring> alterationTexts = {L"bb", L"b", L"", L"#", L"##"};
    vector<wstring> stepTexts = {L"C", L"D", L"E", L"F", L"G", L"A", L"B"};

    for (auto ialt = 0; ialt < alterationTexts.size(); ialt++) {
        auto alt = alterationTexts[ialt];

        for (auto istep = 0; istep < stepTexts.size(); istep++) {
            auto step = stepTexts[istep];

            auto text = step + alt;
            auto parsed = PitchClass::Parse(text);

            EXPECT_EQ(istep, parsed.GetStep());
            EXPECT_EQ(ialt - 3, parsed.GetAlteration());
        }
    }
}
TEST(PitchClass_Parse, Pretty_name) {
    vector<wstring> alterationTexts = {L"𝄫", L"♭", L"", L"♯", L"𝄪"};
    vector<wstring> stepTexts = {L"C", L"D", L"E", L"F", L"G", L"A", L"B"};

    for (auto ialt = 0; ialt < alterationTexts.size(); ialt++) {
        auto alt = alterationTexts[ialt];

        for (auto istep = 0; istep < stepTexts.size(); istep++) {
            auto step = stepTexts[istep];

            auto text = step + alt;
            auto parsed = PitchClass::Parse(text);

            EXPECT_EQ(istep, parsed.GetStep());
            EXPECT_EQ(ialt - 3, parsed.GetAlteration());
        }
    }
}
