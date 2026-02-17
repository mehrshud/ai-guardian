# tests/__init__.py
from typing import Dict
import pytest
from unittest.mock import MagicMock
from ai_guardian import User, Config

@pytest.fixture
def user_data() -> Dict:
    return {"id": 1, "name": "test", "email": "test@example.com"}

@pytest.fixture
def config_data() -> Dict:
    return {"debug": True, "db_url": "test.db"}

def test_create_user_success(user_data: Dict) -> None:
    # Arrange
    user = User(**user_data)
    # Act
    result = user.name
    # Assert
    assert result == user_data["name"]

def test_create_config_success(config_data: Dict) -> None:
    # Arrange
    config = Config(**config_data)
    # Act
    result = config.debug
    # Assert
    assert result == config_data["debug"]

def test_create_user_failure_missing_id() -> None:
    # Arrange
    data = {"name": "test", "email": "test@example.com"}
    # Act and Assert
    with pytest.raises(TypeError):
        User(**data)

def test_create_config_failure_missing_db_url() -> None:
    # Arrange
    data = {"debug": True}
    # Act and Assert
    with pytest.raises(TypeError):
        Config(**data)

def test_mock_external_dependency(mocker) -> None:
    # Arrange
    mock_dependency = mocker.patch("ai_guardian.external_dependency")
    mock_dependency.return_value = "mocked_result"
    # Act
    result = mock_dependency()
    # Assert
    assert result == "mocked_result"
