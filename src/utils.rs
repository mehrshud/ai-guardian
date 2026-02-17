from src.config import Config
from src.vulnerability_scanner import VulnerabilityScanner
from src.models import User
import logging

def load_config(file_path: str) -> Config:
    """
    Load configuration from a file.

    Args:
        file_path (str): Path to the configuration file.

    Returns:
        Config: Loaded configuration.
    """
    try:
        with open(file_path, 'r') as f:
            import json
            config_data = json.load(f)
            return Config(debug=config_data['debug'], db_url=config_data['db_url'])
    except Exception as e:
        logging.error(f"Error loading configuration: {str(e)}")
        raise

def scan_vulnerabilities(config: Config) -> [Vulnerability]:
    """
    Scan for vulnerabilities using the provided configuration.

    Args:
        config (Config): Configuration to use for scanning.

    Returns:
        [Vulnerability]: List of detected vulnerabilities.
    """
    try:
        scanner = VulnerabilityScanner(config)
        return scanner.scan()
    except Exception as e:
        logging.error(f"Error scanning for vulnerabilities: {str(e)}")
        raise

def get_user_metrics(user: User) -> [PerformanceMetric]:
    """
    Get performance metrics for a user.

    Args:
        user (User): User to get metrics for.

    Returns:
        [PerformanceMetric]: List of performance metrics.
    """
    try:
        # This is a placeholder for actual implementation, as the imports_map does not provide enough information
        # about how to get performance metrics for a user. In a real application, you would replace this with
        # actual code to get the metrics.
        metrics = []
        for i in range(5):
            metrics.append(PerformanceMetric(id=i, metric=f"metric_{i}", value=i * 10.0))
        return metrics
    except Exception as e:
        logging.error(f"Error getting user metrics: {str(e)}")
        raise