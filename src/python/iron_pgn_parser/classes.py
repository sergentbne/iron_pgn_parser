from typing import Optional, override

import chess
from chess.pgn import BaseVisitor, SkipType


class NativeUncheckedVisitor(BaseVisitor):
    """
    A visitor with native unchecked parsing.
    This is mainly for known good data who requires just... parsing it.
    This is necessary due to the obnoxiously slow speed of :func:`chess.pgn.read_game()` in bulk reading.

    Use with :func:`chess.pgn.Game.accept()` or
    :func:`chess.pgn.GameNode.accept()` or :func:`chess.pgn.read_game()`.

    The methods are called in PGN order.
    """

    def __init__(self) -> None:
        super().__init__()

    @override
    def begin_game(self) -> Optional[SkipType]:
        """Called at the start of a game."""
        pass

    @override
    def begin_headers(self) -> Optional[Headers]:
        """Called before visiting game headers."""
        pass

    @override
    def visit_header(self, tagname: str, tagvalue: str) -> None:
        """Called for each game header."""
        pass

    @override
    def end_headers(self) -> Optional[SkipType]:
        """Called after visiting game headers."""
        pass

    @override
    def begin_parse_san(self, board: chess.Board, san: str) -> Optional[SkipType]:
        """
        When the visitor is used by a parser, this is called at the start of
        each standard algebraic notation detailing a move.
        """
        pass

    @override
    def parse_san(self, board: chess.Board, san: str) -> chess.Move:
        """
        When the visitor is used by a parser, this is called to parse a move
        in standard algebraic notation.

        You can override the default implementation to work around specific
        quirks of your input format.

        .. deprecated:: 1.1
            This method is very limited, because it is only called on moves
            that the parser recognizes in the first place. Instead of adding
            workarounds here, please report common quirks so that
            they can be handled for everyone.
        """
        return board.parse_san(san)

    def visit_move(self, board: chess.Board, move: chess.Move) -> None:
        """
        Called for each move.

        *board* is the board state before the move. The board state must be
        restored before the traversal continues.
        """
        pass

    def visit_board(self, board: chess.Board) -> None:
        """
        Called for the starting position of the game and after each move.

        The board state must be restored before the traversal continues.
        """
        pass

    def visit_comment(self, comment: str) -> None:
        """Called for each comment."""
        pass

    @override
    def visit_nag(self, nag: int) -> None:
        """Called for each NAG."""
        raise NotImplementedError

    @override
    def begin_variation(self) -> Optional[SkipType]:
        """
        Called at the start of a new variation. It is not called for the
        mainline of the game.
        """
        return NotImplemented

    @override
    def end_variation(self) -> None:
        """Concludes a variation."""
        pass

    @override
    def visit_result(self, result: str) -> None:
        """
        Called at the end of a game with the value from the ``Result`` header.
        """
        pass

    @override
    def end_game(self) -> None:
        """Called at the end of a game."""
        pass

    @override
    def result(self) -> ResultT:
        """Called to get the result of the visitor."""

    @override
    def handle_error(self, error: Exception) -> None:
        """Called for encountered errors. Defaults to raising an exception."""
        raise error
