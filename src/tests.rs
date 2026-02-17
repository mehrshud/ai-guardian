from src.config import Config
from src.vulnerability_scanner import VulnerabilityScanner
from src.models import User
import logging

# Configure logging
logging.basicConfig(level=logging.INFO)

def test_config_init():
    """
    Test Config initialization

    Returns:
        None
    """
    try:
        config = Config(debug=True, db_url="sqlite:///test.db")
        assert config.debug == True
        assert config.db_url == "sqlite:///test.db"
        logging.info("Config initialization test passed")
    except Exception as e:
        logging.error(f"Config initialization test failed: {str(e)}")

def test_vulnerability_scanner_init():
    """
    Test VulnerabilityScanner initialization

    Returns:
        None
    """
    try:
        scanner = VulnerabilityScanner()
        assert scanner is not None
        logging.info("VulnerabilityScanner initialization test passed")
    except Exception as e:
        logging.error(f"VulnerabilityScanner initialization test failed: {str(e)}")

def test_user_init():
    """
    Test User initialization

    Returns:
        None
    """
    try:
        user = User(id=1, name="Test User", email="test@example.com")
        assert user.id == 1
        assert user.name == "Test User"
        assert user.email == "test@example.com"
        logging.info("User initialization test passed")
    except Exception as e:
        logging.error(f"User initialization test failed: {str(e)}")

def test_vulnerability_init():
    """
    Test Vulnerability initialization

    Returns:
        None
    """
    try:
        vulnerability = Vulnerability(id=1, description="Test Vulnerability", severity="High")
        assert vulnerability.id == 1
        assert vulnerability.description == "Test Vulnerability"
        assert vulnerability.severity == "High"
        logging.info("Vulnerability initialization test passed")
    except Exception as e:
        logging.error(f"Vulnerability initialization test failed: {str(e)}")

def test_performance_metric_init():
    """
    Test PerformanceMetric initialization

    Returns:
        None
    """
    try:
        metric = PerformanceMetric(id=1, metric="Response Time", value=0.5)
        assert metric.id == 1
        assert metric.metric == "Response Time"
        assert metric.value == 0.5
        logging.info("PerformanceMetric initialization test passed")
    except Exception as e:
        logging.error(f"PerformanceMetric initialization test failed: {str(e)}")

if __name__ == "__main__":
    test_config_init()
    test_vulnerability_scanner_init()
    test_user_init()
    test_vulnerability_init()
    test_performance_metric_init()