#include <iostream>
#include <libmus.hpp>

using namespace std;

int main()
{
    auto p = PitchStep::C;
    auto pname = PitchStep_ToString(p);
    auto pnum = int(PitchStep_ToNumber(p));
    cout << "The pitch " << pname << " has value " << pnum << endl;
    return 0;
}
