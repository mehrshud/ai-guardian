from src.config.rs import Config
from src.vulnerability_scanner.rs import VulnerabilityScanner
import tensorflow as tf
import logging

# Set up logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class TensorflowModel:
    """Class for integrating TensorFlow models."""
    
    def __init__(self, config: Config):
        """
        Initializes the TensorFlow model.
        
        Args:
        config (Config): The configuration object.
        """
        self.config = config
        self.model = None

    def load_model(self):
        """Loads the TensorFlow model."""
        try:
            self.model = tf.keras.models.load_model('model.h5')
            logger.info("Model loaded successfully.")
        except Exception as e:
            logger.error(f"Failed to load model: {e}")

    def predict_vulnerability(self, user: User) -> Vulnerability:
        """
        Predicts the vulnerability using the TensorFlow model.
        
        Args:
        user (User): The user object.
        
        Returns:
        Vulnerability: The predicted vulnerability.
        """
        try:
            if self.model is None:
                self.load_model()
            input_data = tf.convert_to_tensor([[user.id, len(user.name), len(user.email)]])
            prediction = self.model.predict(input_data)
            vulnerability = Vulnerability(id=user.id, description="Predicted", severity="High")
            logger.info(f"Vulnerability predicted for user {user.id}.")
            return vulnerability
        except Exception as e:
            logger.error(f"Failed to predict vulnerability: {e}")
            return None

    def predict_performance_metric(self, user: User) -> PerformanceMetric:
        """
        Predicts the performance metric using the TensorFlow model.
        
        Args:
        user (User): The user object.
        
        Returns:
        PerformanceMetric: The predicted performance metric.
        """
        try:
            if self.model is None:
                self.load_model()
            input_data = tf.convert_to_tensor([[user.id, len(user.name), len(user.email)]])
            prediction = self.model.predict(input_data)
            performance_metric = PerformanceMetric(id=user.id, metric="Predicted", value=prediction[0][0])
            logger.info(f"Performance metric predicted for user {user.id}.")
            return performance_metric
        except Exception as e:
            logger.error(f"Failed to predict performance metric: {e}")
            return None