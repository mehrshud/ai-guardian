# tests/test_api.py
import pytest
from typing import Dict
from conftest import api_client, mock_database

def test_successful_user_registration(api_client: Dict, mock_database: Dict):
    # Arrange
    user_data = {"id": 1, "name": "John Doe", "email": "john@example.com"}
    
    # Act
    response = api_client.post("/register", json=user_data)
    
    # Assert
    assert response.status_code == 201
    assert mock_database["users"] == [user_data]

def test_invalid_user_registration(api_client: Dict, mock_database: Dict):
    # Arrange
    user_data = {"id": 1, "name": "John Doe"}
    
    # Act
    response = api_client.post("/register", json=user_data)
    
    # Assert
    assert response.status_code == 400
    assert "email" in response.json()["error"]

def test_duplicate_user_registration(api_client: Dict, mock_database: Dict):
    # Arrange
    user_data = {"id": 1, "name": "John Doe", "email": "john@example.com"}
    mock_database["users"] = [user_data]
    
    # Act
    response = api_client.post("/register", json=user_data)
    
    # Assert
    assert response.status_code == 409
    assert "already exists" in response.json()["error"]

def test_config_retrieval(api_client: Dict, mock_database: Dict):
    # Arrange
    config_data = {"debug": True, "db_url": "example.com"}
    mock_database["config"] = config_data
    
    # Act
    response = api_client.get("/config")
    
    # Assert
    assert response.status_code == 200
    assert response.json() == config_data
