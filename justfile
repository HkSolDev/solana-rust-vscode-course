list:
    cargo run -p course-runner -- list

describe:
    cargo run -p course-runner -- describe

check exercise="01_keypairs":
    cargo run -p course-runner -- check {{exercise}}
