# tests/test_core.py
import pytest
from core import User, Config
from typing import Dict

@pytest.fixture
def user_fixture() -> User:
    return User(id=1, name="John Doe", email="john@example.com")

@pytest.fixture
def config_fixture() -> Config:
    return Config(debug=True, db_url="localhost:5432")

def test_create_user_success(user_fixture: User) -> None:
    # Arrange
    expected_id: int = 1
    expected_name: str = "John Doe"
    expected_email: str = "john@example.com"

    # Act
    user: User = user_fixture

    # Assert
    assert user.id == expected_id
    assert user.name == expected_name
    assert user.email == expected_email

def test_create_config_success(config_fixture: Config) -> None:
    # Arrange
    expected_debug: bool = True
    expected_db_url: str = "localhost:5432"

    # Act
    config: Config = config_fixture

    # Assert
    assert config.debug == expected_debug
    assert config.db_url == expected_db_url

def test_create_user_invalid_id() -> None:
    # Arrange
    invalid_id: str = "abc"

    # Act and Assert
    with pytest.raises(TypeError):
        User(id=invalid_id, name="John Doe", email="john@example.com")

def test_create_config_invalid_db_url() -> None:
    # Arrange
    invalid_db_url: int = 123

    # Act and Assert
    with pytest.raises(TypeError):
        Config(debug=True, db_url=invalid_db_url)
