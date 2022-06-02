// #include <gtest/gtest.h>
// #include <libmus/libmus.h>
// #include <boost/algorithm/string/replace.hpp>

// #include <map>
// #include <math.h>

// using namespace std;
// using namespace libmus;
// using namespace boost;

// TEST(Pitch, Assert_initial_values_from_constructor_are_being_set) {
//     Pitch pitch;

//     EXPECT_EQ(1, pitch.GetStep());
//     EXPECT_EQ(0, pitch.GetAlteration());
//     EXPECT_EQ(4, pitch.GetOctave());
// }

// TEST(Pitch_Step, Normal_values_should_work) {
//     Pitch pitch;

//     pitch.SetStep(1);
//     EXPECT_EQ(1, pitch.GetStep());

//     pitch.SetStep(2);
//     EXPECT_EQ(2, pitch.GetStep());

//     pitch.SetStep(3);
//     EXPECT_EQ(3, pitch.GetStep());

//     pitch.SetStep(4);
//     EXPECT_EQ(4, pitch.GetStep());

//     pitch.SetStep(5);
//     EXPECT_EQ(5, pitch.GetStep());

//     pitch.SetStep(6);
//     EXPECT_EQ(6, pitch.GetStep());

//     pitch.SetStep(7);
//     EXPECT_EQ(7, pitch.GetStep());
// }

// TEST(Pitch_Step, Bigger_values_should_be_normalized) {
//     Pitch pitch;

//     pitch.SetStep(8);
//     EXPECT_EQ(1, pitch.GetStep());

//     pitch.SetStep(9);
//     EXPECT_EQ(2, pitch.GetStep());

//     pitch.SetStep(10);
//     EXPECT_EQ(3, pitch.GetStep());

//     pitch.SetStep(11);
//     EXPECT_EQ(4, pitch.GetStep());

//     pitch.SetStep(12);
//     EXPECT_EQ(5, pitch.GetStep());

//     pitch.SetStep(13);
//     EXPECT_EQ(6, pitch.GetStep());

//     pitch.SetStep(14);
//     EXPECT_EQ(7, pitch.GetStep());

//     pitch.SetStep(15);
//     EXPECT_EQ(1, pitch.GetStep());
// }

// TEST(Pitch_Step, Zero_or_negative_values_should_do_nothing) {
//     Pitch pitch;

//     for (auto goodValue = 1; goodValue <= 6; goodValue++) {
//         for (auto badValue = -1; badValue >= -20; badValue--) {
//             pitch.SetStep(goodValue);
//             pitch.SetStep(badValue);
//             EXPECT_EQ(goodValue, pitch.GetStep());
//         }
//     }
// }

// TEST(Pitch_Alteration, When_value_is_in_range_store_the_same) {
//     Pitch pitch;

//     pitch.SetAlteration(-2);
//     EXPECT_EQ(-2, pitch.GetAlteration());

//     pitch.SetAlteration(-1);
//     EXPECT_EQ(-1, pitch.GetAlteration());

//     pitch.SetAlteration(0);
//     EXPECT_EQ(0, pitch.GetAlteration());

//     pitch.SetAlteration(1);
//     EXPECT_EQ(1, pitch.GetAlteration());

//     pitch.SetAlteration(2);
//     EXPECT_EQ(2, pitch.GetAlteration());
// }

// TEST(Pitch_Alteration, When_value_is_smaller_than_limit_keep_min_value) {
//     Pitch pitch;

//     for (auto v = -12; v <= -2; v++) {
//         pitch.SetAlteration(0);
//         pitch.SetAlteration(v);
//         EXPECT_EQ(-2, pitch.GetAlteration());
//     }
// }

// TEST(Pitch_Alteration, When_value_is_greater_than_limit_keep_max_value) {
//     Pitch pitch;

//     for (auto v = 2; v <= 12; v++) {
//         pitch.SetAlteration(0);
//         pitch.SetAlteration(v);
//         EXPECT_EQ(2, pitch.GetAlteration());
//     }
// }

// TEST(Pitch_Octave, When_value_is_in_range_store_the_same) {
//     Pitch pitch;
//     for (auto v = 0; v <= 10; v++) {
//         pitch.SetOctave(v);
//         EXPECT_EQ(v, pitch.GetOctave());
//     }
// }

// TEST(Pitch_Octave, When_value_is_smaller_than_limit_keep_min_value) {
//     Pitch pitch;

//     for (auto v = -12; v <= 0; v++) {
//         pitch.SetOctave(0);
//         pitch.SetOctave(v);
//         EXPECT_EQ(0, pitch.GetOctave());
//     }
// }

// TEST(Pitch_Octave, When_value_is_greater_than_limit_keep_max_value) {
//     Pitch pitch;

//     for (auto v = 10; v <= 25; v++) {
//         pitch.SetOctave(0);
//         pitch.SetOctave(v);
//         EXPECT_EQ(10, pitch.GetOctave());
//     }
// }

// TEST(Pitch_Random, Steps_should_have_a_good_distribution) {
//     map<int, bool> steps;
//     for (auto i = 1; i <= 7 * 5; i++) {
//         auto pitch = Pitch::Random();
//         steps[pitch.GetStep()] = true;
//     }
//     for (auto i = 1; i <= 7; i++) {
//         EXPECT_TRUE(steps.count(i) && steps[i]);
//     }
// }

// TEST(Pitch_Random, Alterations_should_never_be_double) {
//     map<int, bool> alterations;
//     for (auto i = 1; i <= 5 * 5; i++) {
//         auto pitch = Pitch::Random();
//         alterations[pitch.GetAlteration()] = true;
//     }
//     for (auto i = -1; i <= 1; i++) {
//         EXPECT_TRUE(alterations.count(i) && alterations[i]);
//     }
//     for (auto v : {-2, 2}) {
//         EXPECT_FALSE(alterations[v]);
//     }
// }

// TEST(Pitch_Random, Octaves_should_have_a_good_distribution) {
//     map<int, bool> octaves;
//     for (auto i = 1; i <= 10 * 5; i++) {
//         auto pitch = Pitch::Random();
//         octaves[pitch.GetOctave()] = true;
//     }
//     for (auto i = 0; i <= 10; i++) {
//         EXPECT_TRUE(octaves.count(i) && octaves[i]);
//     }
// }

// TEST(Pitch_ExtendedRandom, Steps_should_have_a_good_distribution) {
//     map<int, bool> steps;
//     for (auto i = 1; i <= 7 * 5; i++) {
//         auto pitch = Pitch::ExtendedRandom();
//         steps[pitch.GetStep()] = true;
//     }
//     for (auto i = 1; i <= 7; i++) {
//         EXPECT_TRUE(steps.count(i) && steps[i]);
//     }
// }

// TEST(Pitch_ExtendedRandom, Alterations_should_have_a_good_distribution) {
//     map<int, bool> alterations;
//     for (auto i = 1; i <= 5 * 5; i++) {
//         auto pitch = Pitch::ExtendedRandom();
//         alterations[pitch.GetAlteration()] = true;
//     }
//     for (auto i = -2; i <= 2; i++) {
//         EXPECT_TRUE(alterations.count(i) && alterations.count(i) && alterations.count(i) && alterations[i]);
//     }
// }

// TEST(Pitch_ExtendedRandom, Octaves_should_have_a_good_distribution) {
//     map<int, bool> octaves;
//     for (auto i = 1; i <= 10 * 5; i++) {
//         auto pitch = Pitch::ExtendedRandom();
//         octaves[pitch.GetOctave()] = true;
//     }
//     for (auto i = 0; i <= 10; i++) {
//         EXPECT_TRUE(octaves.count(i) && octaves[i]);
//     }
// }

// // TEST(Pitch_SimpleName, Without_alterations) {
// //     Pitch pitch;
// //     pitch.SetAlteration(0);
// //     vector<wstring> names = {L"C?", L"D?", L"E?", L"F?", L"G?", L"A?", L"B?"};
// //     for (auto o = 0; o <= 10; o++) {
// //         pitch.SetOctave(o);
// //         for (auto i = 1; i <= 7; i++) {
// //             pitch.SetStep(i);
// //             auto expected = boost::algorithm::replace_all(names[i - 1], "?", to_string(pitch.GetOctave()));
// //             auto actual = pitch.Name();
// //             EXPECT_EQ(expected, actual);
// //         }
// //     }
// // }

// // TEST(Pitch_SimpleName, With_1_flat) {
// //     Pitch pitch;
// //     pitch.SetAlteration(-1);
// //     vector<wstring> names = {L"Cb?", L"Db?", L"Eb?", L"Fb?", L"Gb?", L"Ab?", L"Bb?"};
// //     for (auto o = 0; o <= 10; o++) {
// //         pitch.SetOctave(o);
// //         for (auto i = 1; i <= 7; i++) {
// //             pitch.SetStep(i);
// //             auto expected = boost::algorithm::replace_all(names[i - 1], "?", to_string(pitch.GetOctave()));
// //             auto actual = pitch.Name();
// //             EXPECT_STREQ(expected.c_str(), actual.c_str());
// //         }
// //     }
// // }

// // TEST(Pitch_SimpleName, With_2_flats) {
// //     Pitch pitch;
// //     pitch.SetAlteration(-2);
// //     vector<wstring> names = {"Cbb?", "Dbb?", "Ebb?", "Fbb?", "Gbb?", "Abb?", "Bbb?"};
// //     for (auto o = 0; o <= 10; o++) {
// //         pitch.SetOctave(o);
// //         for (auto i = 1; i <= 7; i++) {
// //             pitch.SetStep(i);
// //             auto expected = boost::algorithm::replace_all(names[i - 1], "?", to_string(pitch.GetOctave()));
// //             auto actual = pitch.Name();
// //             EXPECT_EQ(expected, actual);
// //         }
// //     }
// // }

// // TEST(Pitch_SimpleName, With_1_sharp) {
// //     Pitch pitch;
// //     pitch.SetAlteration(1);
// //     vector<wstring> names = {"C#?", "D#?", "E#?", "F#?", "G#?", "A#?", "B#?"};
// //     for (auto o = 0; o <= 10; o++) {
// //         pitch.SetOctave(o);
// //         for (auto i = 1; i <= 7; i++) {
// //             pitch.SetStep(i);
// //             auto expected = boost::algorithm::replace_all(names[i - 1], "?", to_string(pitch.GetOctave()));
// //             auto actual = pitch.Name();
// //             EXPECT_EQ(expected, actual);
// //         }
// //     }
// // }

// // TEST(Pitch_SimpleName, With_2_sharps) {
// //     Pitch pitch;
// //     pitch.SetAlteration(2);
// //     vector<wstring> names = {"C##?", "D##?", "E##?", "F##?", "G##?", "A##?", "B##?"};
// //     for (auto o = 0; o <= 10; o++) {
// //         pitch.SetOctave(o);
// //         for (auto i = 1; i <= 7; i++) {
// //             pitch.SetStep(i);
// //             auto expected = boost::algorithm::replace_all(names[i - 1], "?", to_string(pitch.GetOctave()));
// //             auto actual = pitch.Name();
// //             EXPECT_EQ(expected, actual);
// //         }
// //     }
// // }

// // vector<wstring> superscripts = {"⁰", "¹", "²", "³", "⁴", "⁵", "⁶", "⁷", "⁸", "⁹", "¹⁰"};
// // TEST(Pitch_PrettyName, Without_alterations) {
// //     Pitch pitch;
// //     pitch.SetAlteration(0);
// //     vector<wstring> names = {"C?", "D?", "E?", "F?", "G?", "A?", "B?"};
// //     for (auto o = 0; o <= 10; o++) {
// //         pitch.SetOctave(o);
// //         for (auto i = 1; i <= 7; i++) {
// //             pitch.SetStep(i);
// //             auto expected = boost::algorithm::replace_all(names[i - 1], "?", superscripts[pitch.GetOctave()]);
// //             auto actual = pitch.PrettyName();
// //             EXPECT_EQ(expected, actual);
// //         }
// //     }
// // }

// // TEST(Pitch_PrettyName, With_1_flat) {
// //     Pitch pitch;
// //     pitch.SetAlteration(-1);
// //     vector<wstring> names = {"C♭?", "D♭?", "E♭?", "F♭?", "G♭?", "A♭?", "B♭?"};
// //     for (auto o = 0; o <= 10; o++) {
// //         pitch.SetOctave(o);
// //         for (auto i = 1; i <= 7; i++) {
// //             pitch.SetStep(i);
// //             auto expected = boost::algorithm::replace_all(names[i - 1], "?", superscripts[pitch.GetOctave()]);
// //             auto actual = pitch.PrettyName();
// //             EXPECT_EQ(expected, actual);
// //         }
// //     }
// // }

// // TEST(Pitch_PrettyName, With_2_flats) {
// //     Pitch pitch;
// //     pitch.SetAlteration(-2);
// //     vector<wstring> names = {"C♭♭?", "D♭♭?", "E♭♭?", "F♭♭?", "G♭♭?", "A♭♭?", "B♭♭?"};
// //     for (auto o = 0; o <= 10; o++) {
// //         pitch.SetOctave(o);
// //         for (auto i = 1; i <= 7; i++) {
// //             pitch.SetStep(i);
// //             auto expected = boost::algorithm::replace_all(names[i - 1], "?", superscripts[pitch.GetOctave()]);
// //             auto actual = pitch.PrettyName();
// //             EXPECT_EQ(expected, actual);
// //         }
// //     }
// // }

// // TEST(Pitch_PrettyName, With_1_sharp) {
// //     Pitch pitch;
// //     pitch.SetAlteration(1);
// //     vector<wstring> names = {"C♯?", "D♯?", "E♯?", "F♯?", "G♯?", "A♯?", "B♯?"};
// //     for (auto o = 0; o <= 10; o++) {
// //         pitch.SetOctave(o);
// //         for (auto i = 1; i <= 7; i++) {
// //             pitch.SetStep(i);
// //             auto expected = boost::algorithm::replace_all(names[i - 1], "?", superscripts[pitch.GetOctave()]);
// //             auto actual = pitch.PrettyName();
// //             EXPECT_EQ(expected, actual);
// //         }
// //     }
// // }

// // TEST(Pitch_PrettyName, With_2_sharps) {
// //     Pitch pitch;
// //     pitch.SetAlteration(2);
// //     vector<wstring> names = {"C♯♯?", "D♯♯?", "E♯♯?", "F♯♯?", "G♯♯?", "A♯♯?", "B♯♯?"};
// //     for (auto o = 0; o <= 10; o++) {
// //         pitch.SetOctave(o);
// //         for (auto i = 1; i <= 7; i++) {
// //             pitch.SetStep(i);
// //             auto expected = boost::algorithm::replace_all(names[i - 1], "?", superscripts[pitch.GetOctave()]);
// //             auto actual = pitch.PrettyName();
// //             EXPECT_EQ(expected, actual);
// //         }
// //     }
// // }

// // TEST(Pitch_FullName, Without_alterations) {
// //     Pitch pitch;
// //     pitch.SetAlteration(0);
// //     vector<wstring> names = {"C ?", "D ?", "E ?", "F ?", "G ?", "A ?", "B ?"};
// //     for (auto i = 1; i <= 7; i++) {
// //         pitch.SetStep(i);
// //         auto expected = boost::algorithm::replace_all(names[i - 1], "?", to_string(pitch.GetOctave()));
// //         auto actual = pitch.FullName();
// //         EXPECT_EQ(expected, actual);
// //     }
// // }

// // TEST(Pitch_FullName, With_1_flat) {
// //     Pitch pitch;
// //     pitch.SetAlteration(-1);
// //     vector<wstring> names = {"C flat ?", "D flat ?", "E flat ?", "F flat ?", "G flat ?", "A flat ?", "B flat ?"};
// //     for (auto i = 1; i <= 7; i++) {
// //         pitch.SetStep(i);
// //         auto expected = boost::algorithm::replace_all(names[i - 1], "?", to_string(pitch.GetOctave()));
// //         auto actual = pitch.FullName();
// //         EXPECT_EQ(expected, actual);
// //     }
// // }

// // TEST(Pitch_FullName, With_2_flats) {
// //     Pitch pitch;
// //     pitch.SetAlteration(-2);
// //     vector<wstring> names = {"C double flat ?", "D double flat ?", "E double flat ?", "F double flat ?",
// //                              "G double flat ?", "A double flat ?", "B double flat ?"};
// //     for (auto i = 1; i <= 7; i++) {
// //         pitch.SetStep(i);
// //         auto expected = boost::algorithm::replace_all(names[i - 1], "?", to_string(pitch.GetOctave()));
// //         auto actual = pitch.FullName();
// //         EXPECT_EQ(expected, actual);
// //     }
// // }

// // TEST(Pitch_FullName, With_1_sharp) {
// //     Pitch pitch;
// //     pitch.SetAlteration(1);
// //     vector<wstring> names = {"C sharp ?", "D sharp ?", "E sharp ?", "F sharp ?", "G sharp ?", "A sharp ?", "B
// sharp
// //     ?"}; for (auto i = 1; i <= 7; i++) {
// //         pitch.SetStep(i);
// //         auto expected = boost::algorithm::replace_all(names[i - 1], "?", to_string(pitch.GetOctave()));
// //         auto actual = pitch.FullName();
// //         EXPECT_EQ(expected, actual);
// //     }
// // }

// // TEST(Pitch_FullName, With_2_sharps) {
// //     Pitch pitch;
// //     pitch.SetAlteration(2);
// //     vector<wstring> names = {"C double sharp ?", "D double sharp ?", "E double sharp ?", "F double sharp ?",
// //                              "G double sharp ?", "A double sharp ?", "B double sharp ?"};

// //     for (auto i = 1; i <= 7; i++) {
// //         pitch.SetStep(i);
// //         auto expected = boost::algorithm::replace_all(names[i - 1], "?", to_string(pitch.GetOctave()));
// //         auto actual = pitch.FullName();
// //         EXPECT_EQ(expected, actual);
// //     }
// // }

// // TEST(Pitch_Parse_without_octave, Simple_name_without_octave) {
// //     vector<wstring> alterationTexts = {"bb", "b", "", "#", "##"};
// //     vector<wstring> stepTexts = {"C", "D", "E", "F", "G", "A", "B"};

// //     for (auto iterAlt = alterationTexts.begin(); iterAlt != alterationTexts.end(); iterAlt++) {
// //         auto ialt = iterAlt - alterationTexts.begin();
// //         auto alt = *iterAlt;

// //         for (auto iterStep = stepTexts.begin(); iterStep != stepTexts.end(); iterStep++) {
// //             auto istep = iterStep - stepTexts.begin();
// //             auto step = *iterStep;

// //             auto text = step + alt;
// //             auto parsed = Pitch::Parse(text);

// //             EXPECT_EQ(istep, parsed.GetStep());
// //             EXPECT_EQ(ialt - 3, parsed.GetAlteration());
// //             EXPECT_EQ(4, parsed.GetOctave());
// //         }
// //     }
// // }

// // TEST(Pitch_Parse_without_octave, Pretty_name) {
// //     vector<wstring> alterationTexts = {"♭♭", "♭", "", "♯", "♯♯"};
// //     vector<wstring> stepTexts = {"C", "D", "E", "F", "G", "A", "B"};

// //     for (auto iterAlt = alterationTexts.begin(); iterAlt != alterationTexts.end(); iterAlt++) {
// //         auto ialt = iterAlt - alterationTexts.begin();
// //         auto alt = *iterAlt;

// //         for (auto iterStep = stepTexts.begin(); iterStep != stepTexts.end(); iterStep++) {
// //             auto istep = iterStep - stepTexts.begin();
// //             auto step = *iterStep;

// //             auto text = step + alt;
// //             auto parsed = Pitch::Parse(text);

// //             EXPECT_EQ(istep, parsed.GetStep());
// //             EXPECT_EQ(ialt - 3, parsed.GetAlteration());
// //             EXPECT_EQ(4, parsed.GetOctave());
// //         }
// //     }
// // }
// // TEST(Pitch_Parse_with_octave, Simple_name) {
// //     vector<wstring> alterationTexts = {"bb", "b", "", "#", "##"};
// //     vector<wstring> stepTexts = {"C", "D", "E", "F", "G", "A", "B"};

// //     for (auto oct = 0; oct <= 10; oct++) {
// //         for (auto iterAlt = alterationTexts.begin(); iterAlt != alterationTexts.end(); iterAlt++) {
// //             auto ialt = iterAlt - alterationTexts.begin();
// //             auto alt = *iterAlt;

// //             for (auto iterStep = stepTexts.begin(); iterStep != stepTexts.end(); iterStep++) {
// //                 auto istep = iterStep - stepTexts.begin();
// //                 auto step = *iterStep;

// //                 auto text = step + alt + to_string(oct);
// //                 auto parsed = Pitch::Parse(text);

// //                 EXPECT_EQ(istep, parsed.GetStep());
// //                 EXPECT_EQ(ialt - 3, parsed.GetAlteration());
// //                 EXPECT_EQ(oct, parsed.GetOctave());
// //             }
// //         }
// //     }
// // }
// // TEST(Pitch_Parse_with_octave, Pretty_name) {
// //     vector<wstring> alterationTexts = {"♭♭", "♭", "", "♯", "♯♯"};
// //     vector<wstring> stepTexts = {"C", "D", "E", "F", "G", "A", "B"};

// //     for (auto oct = 0; oct <= 10; oct++) {
// //         for (auto iterAlt = alterationTexts.begin(); iterAlt != alterationTexts.end(); iterAlt++) {
// //             auto ialt = iterAlt - alterationTexts.begin();
// //             auto alt = *iterAlt;

// //             for (auto iterStep = stepTexts.begin(); iterStep != stepTexts.end(); iterStep++) {
// //                 auto istep = iterStep - stepTexts.begin();
// //                 auto step = *iterStep;

// //                 auto text = step + alt + to_string(oct);
// //                 auto parsed = Pitch::Parse(text);

// //                 EXPECT_EQ(istep, parsed.GetStep());
// //                 EXPECT_EQ(ialt - 3, parsed.GetAlteration());
// //                 EXPECT_EQ(oct, parsed.GetOctave());
// //             }
// //         }
// //     }
// // }