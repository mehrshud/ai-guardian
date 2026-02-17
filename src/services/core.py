from src.models import User
from src.lib import Config, VulnerabilityScanner

class CoreService:
    def __init__(self, config: Config):
        self.config = config
        self.vulnerability_scanner = VulnerabilityScanner()

    def get_user(self, user_id: int) -> User:
        try:
            users = [
                User(1, "John Doe", "john@example.com"),
                User(2, "Jane Doe", "jane@example.com")
            ]
            user = next((u for u in users if u.id == user_id), None)
            if user is None:
                raise ValueError(f"User with ID {user_id} not found")
            return user
        except Exception as e:
            raise Exception(f"Error getting user: {e}")

    def scan_vulnerabilities(self) -> list:
        try:
            return self.vulnerability_scanner.scan()
        except Exception as e:
            raise Exception(f"Error scanning vulnerabilities: {e}")

    def get_performance_metrics(self) -> list:
        try:
            metrics = [
                {"id": 1, "metric": "response_time", "value": 100.0},
                {"id": 2, "metric": "memory_usage", "value": 50.0}
            ]
            return metrics
        except Exception as e:
            raise Exception(f"Error getting performance metrics: {e}")