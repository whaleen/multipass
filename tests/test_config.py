import os
import json
import tempfile
from multipass.config import MultipassConfig


def test_default_config():
    cfg = MultipassConfig(config_dir=tempfile.mkdtemp())
    assert "ship" in cfg.ship_path
    assert cfg.collection_name == "multipass_crates"


def test_config_from_file():
    tmpdir = tempfile.mkdtemp()
    with open(os.path.join(tmpdir, "config.json"), "w") as f:
        json.dump({"ship_path": "/custom/ship"}, f)
    cfg = MultipassConfig(config_dir=tmpdir)
    assert cfg.ship_path == "/custom/ship"


def test_env_override():
    os.environ["MULTIPASS_SHIP_PATH"] = "/env/ship"
    cfg = MultipassConfig(config_dir=tempfile.mkdtemp())
    assert cfg.ship_path == "/env/ship"
    del os.environ["MULTIPASS_SHIP_PATH"]


def test_init():
    tmpdir = tempfile.mkdtemp()
    cfg = MultipassConfig(config_dir=tmpdir)
    cfg.init()
    assert os.path.exists(os.path.join(tmpdir, "config.json"))
