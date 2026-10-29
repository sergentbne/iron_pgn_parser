.PHONY: test test-release

test:
	./ensure_file.sh
	uv run maturin develop
	uv run python -c "import iron_pgn_parser.native; iron_pgn_parser.native.stream_file('data/lichess_db_standard_rated_2013-03.pgn.zst')"

test-release:
	./ensure_file.sh
	uv run maturin develop --release
	uv run python -c "import iron_pgn_parser.native; iron_pgn_parser.native.stream_file('data/lichess_db_standard_rated_2013-03.pgn.zst')"
