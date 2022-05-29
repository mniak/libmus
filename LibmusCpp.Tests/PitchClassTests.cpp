#include "pch.h"


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

	for (auto goodValue = 1; goodValue <= 6; goodValue++)
	{
		for (auto badValue = -1; badValue >= -20; badValue--)
		{
			pc.Step = goodValue;
			pc.Step = badValue;
			EXPECT_EQ(goodValue, pc.Step);
		}
	}
}
