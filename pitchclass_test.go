package libmus

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_PitchClass_Step_Constructor_should_set_initial_values(t *testing.T) {
	pc := NewPitchClass()

	assert.Equal(t, 1, pc.GetStep())
	assert.Equal(t, 0, pc.GetAlteration())
}

func Test_PitchClass_Step_Normal_values_should_work(t *testing.T) {
	var pc PitchClass

	pc.SetStep(1)
	assert.Equal(t, 1, pc.GetStep())

	pc.SetStep(2)
	assert.Equal(t, 2, pc.GetStep())

	pc.SetStep(3)
	assert.Equal(t, 3, pc.GetStep())

	pc.SetStep(4)
	assert.Equal(t, 4, pc.GetStep())

	pc.SetStep(5)
	assert.Equal(t, 5, pc.GetStep())

	pc.SetStep(6)
	assert.Equal(t, 6, pc.GetStep())

	pc.SetStep(7)
	assert.Equal(t, 7, pc.GetStep())
}

func Test_PitchClass_Step_Bigger_values_should_be_normalized(t *testing.T) {
	var pc PitchClass

	pc.SetStep(8)
	assert.Equal(t, 1, pc.GetStep())

	pc.SetStep(9)
	assert.Equal(t, 2, pc.GetStep())

	pc.SetStep(10)
	assert.Equal(t, 3, pc.GetStep())

	pc.SetStep(11)
	assert.Equal(t, 4, pc.GetStep())

	pc.SetStep(12)
	assert.Equal(t, 5, pc.GetStep())

	pc.SetStep(13)
	assert.Equal(t, 6, pc.GetStep())

	pc.SetStep(14)
	assert.Equal(t, 7, pc.GetStep())

	pc.SetStep(15)
	assert.Equal(t, 1, pc.GetStep())
}

// func Test_PitchClass_Step_Attributing_zero_or_negative_should_do_nothing(t *testing.T) {
//     var  pc PitchClass;

//     for (auto goodValue = 1; goodValue <= 6; goodValue++) {
//         for (auto badValue = -1; badValue >= -20; badValue--) {
//             pc.SetStep(goodValue);
//             pc.SetStep(badValue);
//             assert.Equal(t, goodValue, pc.GetStep());
//         }
//     }
// }
// func Test_PitchClass_Alteration_When_value_is_in_range_store_the_same(t *testing.T) {
//     var  pc PitchClass;

//     pc.SetAlteration(-2);
//     assert.Equal(t, -2, pc.GetAlteration());

//     pc.SetAlteration(-1);
//     assert.Equal(t, -1, pc.GetAlteration());

//     pc.SetAlteration(0);
//     assert.Equal(t, 0, pc.GetAlteration());

//     pc.SetAlteration(1);
//     assert.Equal(t, 1, pc.GetAlteration());

//     pc.SetAlteration(2);
//     assert.Equal(t, 2, pc.GetAlteration());
// }
// func Test_PitchClass_Alteration_When_value_is_smaller_than_limit_then_keep_min_value(t *testing.T) {
//     var  pc PitchClass;

//     for (auto v = -12; v <= -2; v++) {
//         pc.SetAlteration(0);
//         pc.SetAlteration(v);
//         assert.Equal(t, -2, pc.GetAlteration());
//     }
// }
// func Test_PitchClass_Alteration_When_value_is_greater_than_limit_then_keep_max_value(t *testing.T) {
//     var  pc PitchClass;

//     for (auto v = 2; v <= 12; v++) {
//         pc.SetAlteration(0);
//         pc.SetAlteration(v);
//         assert.Equal(t, 2, pc.GetAlteration());
//     }
// }
// func Test_PitchClass_Random_Steps_should_have_a_good_distribution(t *testing.T) {
//     map<int, bool> steps;
//     for (auto i = 1; i <= 7 * 5; i++) {
//         auto pc = PitchClass::Random();
//         steps[pc.GetStep()] = true;
//     }
//     for (auto i = 1; i <= 7; i++) {
//         auto pc = PitchClass::Random();
//         EXPECT_TRUE(steps[i]);
//     }
// }
// func Test_PitchClass_Random_Alterations_should_never_be_double(t *testing.T) {
//     map<int, bool> alterations;
//     for (auto i = 1; i <= 3 * 5; i++) {
//         auto pc = PitchClass::Random();
//         alterations[pc.GetAlteration()] = true;
//     }
//     for (auto i = -1; i <= 1; i++) {
//         EXPECT_TRUE(alterations[i]);
//     }
//     for (auto v : {-2, 2}) {
//         EXPECT_FALSE(alterations[v]);
//     }
// }
// func Test_PitchClass_ExtendedRandom_Steps_should_have_a_good_distribution(t *testing.T) {
//     map<int, bool> steps;
//     for (auto i = 1; i <= 7 * 5; i++) {
//         auto pc = PitchClass::ExtendedRandom();
//         steps[pc.GetStep()] = true;
//     }
//     for (auto i = 1; i <= 7; i++) {
//         auto pc = PitchClass::ExtendedRandom();
//         EXPECT_TRUE(steps[i]);
//     }
// }
// func Test_PitchClass_ExtendedRandom_Alterations_should_have_a_good_distribution(t *testing.T) {
//     map<int, bool> alterations;
//     for (auto i = 1; i <= 5 * 5; i++) {
//         auto pc = PitchClass::ExtendedRandom();
//         alterations[pc.GetAlteration()] = true;
//     }
//     for (auto i = -2; i <= 2; i++) {
//         EXPECT_TRUE(alterations[i]);
//     }
// }
// func Test_PitchClass_SimpleName_Without_alterations(t *testing.T) {
//     var  pc PitchClass;
//     pc.SetAlteration(0);
//     vector<u32string> names = {U"C", U"D", U"E", U"F", U"G", U"A", U"B"};
//     for (auto i = 1; i <= 7; i++) {
//         pc.SetStep(i);
//         auto expected = names[i - 1];
//         auto actual = pc.Name();
//         assert.Equal(t, expected, actual);
//     }
// }
// func Test_PitchClass_SimpleName_With_1_flat(t *testing.T) {
//     var  pc PitchClass;
//     pc.SetAlteration(-1);
//     vector<u32string> names = {U"Cb", U"Db", U"Eb", U"Fb", U"Gb", U"Ab", U"Bb"};
//     for (auto i = 1; i <= 7; i++) {
//         pc.SetStep(i);
//         auto expected = names[i - 1];
//         auto actual = pc.Name();
//         assert.Equal(t, expected, actual);
//     }
// }
// func Test_PitchClass_SimpleName_With_2_flats(t *testing.T) {
//     var  pc PitchClass;
//     pc.SetAlteration(-2);
//     vector<u32string> names = {U"Cbb", U"Dbb", U"Ebb", U"Fbb", U"Gbb", U"Abb", U"Bbb"};
//     for (auto i = 1; i <= 7; i++) {
//         pc.SetStep(i);
//         auto expected = names[i - 1];
//         auto actual = pc.Name();
//         assert.Equal(t, expected, actual);
//     }
// }
// func Test_PitchClass_SimpleName_With_1_sharp(t *testing.T) {
//     var  pc PitchClass;
//     pc.SetAlteration(1);
//     vector<u32string> names = {U"C#", U"D#", U"E#", U"F#", U"G#", U"A#", U"B#"};
//     for (auto i = 1; i <= 7; i++) {
//         pc.SetStep(i);
//         auto expected = names[i - 1];
//         auto actual = pc.Name();
//         assert.Equal(t, expected, actual);
//     }
// }
// func Test_PitchClass_SimpleName_With_2_sharps(t *testing.T) {
//     var  pc PitchClass;
//     pc.SetAlteration(2);
//     vector<u32string> names = {U"C##", U"D##", U"E##", U"F##", U"G##", U"A##", U"B##"};
//     for (auto i = 1; i <= 7; i++) {
//         pc.SetStep(i);
//         auto expected = names[i - 1];
//         auto actual = pc.Name();
//         assert.Equal(t, expected, actual);
//     }
// }
// func Test_PitchClass_PrettyName_Without_alterations(t *testing.T) {
//     var  pc PitchClass;
//     pc.SetAlteration(0);
//     vector<u32string> names = {U"C", U"D", U"E", U"F", U"G", U"A", U"B"};
//     for (auto i = 1; i <= 7; i++) {
//         pc.SetStep(i);
//         auto expected = names[i - 1];
//         auto actual = pc.PrettyName();
//         assert.Equal(t, expected, actual);
//     }
// }
// func Test_PitchClass_PrettyName_With_1_flat(t *testing.T) {
//     var  pc PitchClass;
//     pc.SetAlteration(-1);
//     vector<u32string> names = {U"C♭", U"D♭", U"E♭", U"F♭", U"G♭", U"A♭", U"B♭"};
//     for (auto i = 1; i <= 7; i++) {
//         pc.SetStep(i);
//         auto expected = names[i - 1];
//         auto actual = pc.PrettyName();
//         assert.Equal(t, expected, actual);
//     }
// }
// func Test_PitchClass_PrettyName_With_2_flats(t *testing.T) {
//     var  pc PitchClass;
//     pc.SetAlteration(-2);
//     vector<u32string> names = {U"C𝄫", U"D𝄫", U"E𝄫", U"F𝄫", U"G𝄫", U"A𝄫", U"B𝄫"};
//     for (auto i = 1; i <= 7; i++) {
//         pc.SetStep(i);
//         auto expected = names[i - 1];
//         auto actual = pc.PrettyName();
//         assert.Equal(t, expected, actual);
//     }
// }
// func Test_PitchClass_PrettyName_With_1_sharp(t *testing.T) {
//     var  pc PitchClass;
//     pc.SetAlteration(1);
//     vector<u32string> names = {U"C♯", U"D♯", U"E♯", U"F♯", U"G♯", U"A♯", U"B♯"};
//     for (auto i = 1; i <= 7; i++) {
//         pc.SetStep(i);
//         auto expected = names[i - 1];
//         auto actual = pc.PrettyName();
//         assert.Equal(t, expected, actual);
//     }
// }
// func Test_PitchClass_PrettyName_With_2_sharps(t *testing.T) {
//     var  pc PitchClass;
//     pc.SetAlteration(2);
//     vector<u32string> names = {U"C𝄪", U"D𝄪", U"E𝄪", U"F𝄪", U"G𝄪", U"A𝄪", U"B𝄪"};
//     for (auto i = 1; i <= 7; i++) {
//         pc.SetStep(i);
//         auto expected = names[i - 1];
//         auto actual = pc.PrettyName();
//         assert.Equal(t, expected, actual);
//     }
// }
// func Test_PitchClass_FullName_Without_alterations(t *testing.T) {
//     var  pc PitchClass;
//     pc.SetAlteration(0);
//     vector<u32string> names = {U"C", U"D", U"E", U"F", U"G", U"A", U"B"};
//     for (auto i = 1; i <= 7; i++) {
//         pc.SetStep(i);
//         auto expected = names[i - 1];
//         auto actual = pc.FullName();
//         assert.Equal(t, expected, actual);
//     }
// }
// func Test_PitchClass_FullName_With_1_flat(t *testing.T) {
//     var  pc PitchClass;
//     pc.SetAlteration(-1);
//     vector<u32string> names = {U"C flat", U"D flat", U"E flat", U"F flat", U"G flat", U"A flat", U"B flat"};
//     for (auto i = 1; i <= 7; i++) {
//         pc.SetStep(i);
//         auto expected = names[i - 1];
//         auto actual = pc.FullName();
//         assert.Equal(t, expected, actual);
//     }
// }
// func Test_PitchClass_FullName_With_2_flats(t *testing.T) {
//     var  pc PitchClass;
//     pc.SetAlteration(-2);
//     vector<u32string> names = {U"C double flat", U"D double flat", U"E double flat", U"F double flat",
//                                U"G double flat", U"A double flat", U"B double flat"};
//     for (auto i = 1; i <= 7; i++) {
//         pc.SetStep(i);
//         auto expected = names[i - 1];
//         auto actual = pc.FullName();
//         assert.Equal(t, expected, actual);
//     }
// }
// func Test_PitchClass_FullName_With_1_sharp(t *testing.T) {
//     var  pc PitchClass;
//     pc.SetAlteration(1);
//     vector<u32string> names = {U"C sharp", U"D sharp", U"E sharp", U"F sharp", U"G sharp", U"A sharp", U"B sharp"};
//     for (auto i = 1; i <= 7; i++) {
//         pc.SetStep(i);
//         auto expected = names[i - 1];
//         auto actual = pc.FullName();
//         assert.Equal(t, expected, actual);
//     }
// }
// func Test_PitchClass_FullName_With_2_sharps(t *testing.T) {
//     var  pc PitchClass;
//     pc.SetAlteration(2);
//     vector<u32string> names = {U"C double sharp", U"D double sharp", U"E double sharp", U"F double sharp",
//                                U"G double sharp", U"A double sharp", U"B double sharp"};
//     for (auto i = 1; i <= 7; i++) {
//         pc.SetStep(i);
//         auto expected = names[i - 1];
//         auto actual = pc.FullName();
//         assert.Equal(t, expected, actual);
//     }
// }
// func Test_PitchClass_Parse_Simple_name(t *testing.T) {
//     vector<u32string> alterationTexts = {U"bb", U"b", U"", U"#", U"##"};
//     vector<u32string> stepTexts = {U"C", U"D", U"E", U"F", U"G", U"A", U"B"};

//     for (auto ialt = 0; ialt < alterationTexts.size(); ialt++) {
//         auto alt = alterationTexts[ialt];

//         for (auto istep = 0; istep < stepTexts.size(); istep++) {
//             auto step = stepTexts[istep];

//             auto text = step + alt;
//             auto parsed = PitchClass::Parse(text);

//             assert.Equal(t, istep + 1, parsed.GetStep());
//             assert.Equal(t, ialt - 2, parsed.GetAlteration());
//         }
//     }
// }func Test_PitchClass_Parse_Pretty_name(t *testing.T) {
//     vector<u32string> alterationTexts = {U"𝄫", U"♭", U"", U"♯", U"𝄪"};
//     vector<u32string> stepTexts = {U"C", U"D", U"E", U"F", U"G", U"A", U"B"};

//     for (auto ialt = 0; ialt < alterationTexts.size(); ialt++) {
//         auto alt = alterationTexts[ialt];

//         for (auto istep = 0; istep < stepTexts.size(); istep++) {
//             auto step = stepTexts[istep];

//             auto text = step + alt;
//             auto parsed = PitchClass::Parse(text);

//             assert.Equal(t, istep + 1, parsed.GetStep());
//             assert.Equal(t, ialt - 2, parsed.GetAlteration());
//         }
//     }
// }
