"""
test_searcher.py — Tests for the programmatic search_memories API.

Tests the library-facing search interface (not the CLI print variant).
"""

from multipass.searcher import search_memories


class TestSearchMemories:
    def test_basic_search(self, ship_path, seeded_collection):
        result = search_memories("JWT authentication", ship_path)
        assert "results" in result
        assert len(result["results"]) > 0
        assert result["query"] == "JWT authentication"

    def test_wing_filter(self, ship_path, seeded_collection):
        result = search_memories("planning", ship_path, wing="notes")
        assert all(r["wing"] == "notes" for r in result["results"])

    def test_room_filter(self, ship_path, seeded_collection):
        result = search_memories("database", ship_path, room="backend")
        assert all(r["room"] == "backend" for r in result["results"])

    def test_wing_and_room_filter(self, ship_path, seeded_collection):
        result = search_memories("code", ship_path, wing="project", room="frontend")
        assert all(r["wing"] == "project" and r["room"] == "frontend" for r in result["results"])

    def test_n_results_limit(self, ship_path, seeded_collection):
        result = search_memories("code", ship_path, n_results=2)
        assert len(result["results"]) <= 2

    def test_no_ship_returns_error(self, tmp_path):
        result = search_memories("anything", str(tmp_path / "missing"))
        assert "error" in result

    def test_result_fields(self, ship_path, seeded_collection):
        result = search_memories("authentication", ship_path)
        hit = result["results"][0]
        assert "text" in hit
        assert "wing" in hit
        assert "room" in hit
        assert "source_file" in hit
        assert "similarity" in hit
        assert isinstance(hit["similarity"], float)
