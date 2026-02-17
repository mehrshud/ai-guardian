from src/config.rs import Config
from src/vulnerability_scanner.rs import VulnerabilityScanner
from src/models.rs import User

class ThreatDetector:
    def __init__(self, config: Config, vulnerability_scanner: VulnerabilityScanner) -> None:
        """
        Initialize the threat detector with the given configuration and vulnerability scanner.
        
        Args:
            config (Config): The configuration for the threat detector.
            vulnerability_scanner (VulnerabilityScanner): The vulnerability scanner to use for threat detection.
        """
        self.config = config
        self.vulnerability_scanner = vulnerability_scanner

    def detect_threats(self, user: User) -> list[Vulnerability]:
        """
        Detect threats for the given user.
        
        Args:
            user (User): The user to detect threats for.
        
        Returns:
            list[Vulnerability]: A list of detected vulnerabilities.
        """
        try:
            # Scan for vulnerabilities
            vulnerabilities = self.vulnerability_scanner.scan(user.id)
            
            # Log the detected vulnerabilities if debug mode is enabled
            if self.config.debug:
                print(f"Detected vulnerabilities for user {user.id}: {vulnerabilities}")
            
            return vulnerabilities
        except Exception as e:
            # Log any exceptions that occur during threat detection
            print(f"Error detecting threats for user {user.id}: {str(e)}")
            return []
        
def main() -> None:
    # Load the configuration
    config = Config(debug=True, db_url="localhost:5432")
    
    # Initialize the vulnerability scanner
    vulnerability_scanner = VulnerabilityScanner()
    
    # Initialize the threat detector
    threat_detector = ThreatDetector(config, vulnerability_scanner)
    
    # Detect threats for a user
    user = User(id=1, name="John Doe", email="john@example.com")
    vulnerabilities = threat_detector.detect_threats(user)
    
    # Print the detected vulnerabilities
    print(vulnerabilities)

if __name__ == "__main__":
    main()