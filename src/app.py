from src.config import Config
from src.vulnerability_scanner import VulnerabilityScanner
from src.models import Vulnerability
from src.services.user import User

import logging

def main() -> None:
    try:
        # Initialize logging
        logging.basicConfig(level=logging.INFO)
        logging.info("AI Guardian application started")

        # Load application configuration
        config: Config = Config(debug=True, db_url="localhost:5432")

        # Initialize vulnerability scanner
        scanner: VulnerabilityScanner = VulnerabilityScanner(config)

        # Create a sample user
        user: User = User(id=1, name="John Doe", email="john@example.com")

        # Simulate a vulnerability scan
        try:
            vulnerabilities: list[Vulnerability] = scanner.scan(user)
        except Exception as e:
            logging.error(f"Error scanning user: {e}")
            return

        # Log the results
        for vulnerability in vulnerabilities:
            logging.warning(f"Vulnerability found: {vulnerability.id} - {vulnerability.description}")

    except Exception as e:
        logging.error(f"Application error: {e}")

if __name__ == "__main__":
    main()