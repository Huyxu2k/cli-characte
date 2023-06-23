build:
	cargo build
test-inspect-d:
	target\debug\cli-character.exe inspect A1B2C3456 -d
test-inspect:
	target\debug\cli-character.exe inspect A1B2C3456
test-reverse:
	target\debug\cli-character.exe reverse ABCDEF