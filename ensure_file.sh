#!/usr/bin/env bash
HASH="89da64fc3c1fe3bfd571d7f626232189f3259aa728b46ea81e5cb8f3fdb34b9e"
DIRECTORY_OF_FILE="data"
FILE="lichess_db_standard_rated_2013-03.pgn.zst"
FULL_PATH="${DIRECTORY_OF_FILE}/${FILE}"
FILE_URL="https://database.lichess.org/standard/lichess_db_standard_rated_2013-03.pgn.zst"
if [ -f $FULL_PATH ]; then
	COMPUTED_HASH="$(sha256sum --quiet $FULL_PATH)"
	if [[ $COMPUTED_HASH == $HASH ]]; then
		exit 0
	else
		>&2 echo "Computed Hash differs from hardcoded hash for $FILE, deleting and recreating..."
		rm $FILE
	fi
fi

if [ ! -d $DIRECTORY_OF_FILE ];then
	mkdir $DIRECTORY_OF_FILE
fi

curl $FILE_URL -o $FULL_PATH
