import std.stdio;

void main()
{
	writeln("Edit source/app.d to start your project.");
}

int myAbs(int i) {
	return i;
}

unittest
{
	assert(myAbs(-1) == 1);
	assert(myAbs(1) == 1);

}
