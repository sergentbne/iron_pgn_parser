import time
from compression import zstd
from pprint import pprint

import chess.pgn

import iron_pgn_parser.native

FILENAME = "data/lichess_db_standard_rated_2013-03.pgn.zst"


def main() -> None:
    start_mine = time.perf_counter()
    found_games: list[iron_pgn_parser.native.ChessData] = (
        iron_pgn_parser.native.stream_file(FILENAME)
    )
    end_mine = time.perf_counter()
    found_headers = list(map(lambda x: x.headers, found_games))
    known_good_headers = []
    start_other = time.perf_counter()
    with zstd.open(FILENAME, "rt") as f:
        while x := chess.pgn.read_headers(f):
            known_good_headers.append(dict(x))
    end_other = time.perf_counter()
    print(f"mine: {end_mine - start_mine}. other: {end_other - start_other}")

    diff_found = False
    diff_list = []
    for d1, d2 in zip(known_good_headers, found_headers):
        if len(d1) != len(d2):
            diff_found = True
            diff_list.append((d1, d2))

        else:
            # Compare each key and value
            for key in d1:
                if key not in d2 or d1[key] != d2[key]:
                    diff_found = True
                    diff_list.append((d1, d2))
                    break
    print(known_good_headers.index(diff_list[0][0]))
    pprint(len(diff_list))
    pprint(diff_list[0])
