# tests/conftest.py
import pytest
from typing import Optional
from unittest.mock import Mock
from ai_guardian import User, Config

@pytest.fixture
def user() -> User:
    return User(id=1, name="Test User", email="test@example.com")

@pytest.fixture
def config() -> Config:
    return Config(debug=True, db_url="test_db_url")

def test_user_creation(user: User) -> None:
    # Arrange
    expected_id = 1
    expected_name = "Test User"
    expected_email = "test@example.com"

    # Act
    actual_id = user.id
    actual_name = user.name
    actual_email = user.email

    # Assert
    assert actual_id == expected_id
    assert actual_name == expected_name
    assert actual_email == expected_email

def test_config_creation(config: Config) -> None:
    # Arrange
    expected_debug = True
    expected_db_url = "test_db_url"

    # Act
    actual_debug = config.debug
    actual_db_url = config.db_url

    # Assert
    assert actual_debug == expected_debug
    assert actual_db_url == expected_db_url

def test_user_negative_id() -> None:
    # Arrange
    with pytest.raises(ValueError):
        User(id=-1, name="Test User", email="test@example.com")

def test_config_invalid_db_url() -> None:
    # Arrange
    with pytest.raises(ValueError):
        Config(debug=True, db_url=None)

def test_user_mock_external_dependency(mocker: Mock) -> None:
    # Arrange
    mocker.patch("ai_guardian.User")
    User(id=1, name="Test User", email="test@example.com")

    # Act
    User.assert_called_once()
