from src/models import User
from src import config
from src.vulnerability_scanner import VulnerabilityScanner

class UserService:
    def __init__(self, user: User):
        """
        Initialize the UserService with a User object.

        :param user: A User object
        """
        self.user = user

    def get_user(self) -> User:
        """
        Get the current User object.

        :return: The current User object
        """
        return self.user

class VulnerabilityService:
    def __init__(self, config: config.Config):
        """
        Initialize the VulnerabilityService with a Config object.

        :param config: A Config object
        """
        self.config = config
        self.scanner = VulnerabilityScanner()

    def scan_vulnerabilities(self) -> list:
        """
        Scan for vulnerabilities using the VulnerabilityScanner.

        :return: A list of Vulnerability objects
        """
        try:
            return self.scanner.scan(self.config.db_url)
        except Exception as e:
            print(f"Error scanning vulnerabilities: {e}")
            return []

class PerformanceService:
    def __init__(self, config: config.Config):
        """
        Initialize the PerformanceService with a Config object.

        :param config: A Config object
        """
        self.config = config

    def get_performance_metrics(self) -> list:
        """
        Get performance metrics from the database.

        :return: A list of PerformanceMetric objects
        """
        try:
            # Simulate retrieving performance metrics from the database
            metrics = [
                {"id": 1, "metric": "response_time", "value": 100.0},
                {"id": 2, "metric": "throughput", "value": 500.0}
            ]
            return [PerformanceMetric(**metric) for metric in metrics]
        except Exception as e:
            print(f"Error retrieving performance metrics: {e}")
            return []

class PerformanceMetric:
    def __init__(self, id: int, metric: str, value: float):
        """
        Initialize a PerformanceMetric object.

        :param id: The metric ID
        :param metric: The metric name
        :param value: The metric value
        """
        self.id = id
        self.metric = metric
        self.value = value

class Vulnerability:
    def __init__(self, id: int, description: str, severity: str):
        """
        Initialize a Vulnerability object.

        :param id: The vulnerability ID
        :param description: The vulnerability description
        :param severity: The vulnerability severity
        """
        self.id = id
        self.description = description
        self.severity = severity
        print(f"Vulnerability {id} detected with severity {severity}")