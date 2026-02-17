from dataclasses import dataclass
from typing import Optional
import logging
import os

@dataclass
class Config:
    """
    Configuration class for the AI Guardian system.
    
    Attributes:
    debug (bool): Debug mode flag.
    db_url (str): Database URL.
    """
    debug: bool
    db_url: str

def load_config() -> Config:
    """
    Loads configuration from environment variables.

    Returns:
    Config: Loaded configuration.

    Raises:
    ValueError: If the DB_URL environment variable is not set.
    """
    try:
        debug = os.environ.get('DEBUG', 'False').lower() == 'true'
        db_url = os.environ.get('DB_URL')
        if not db_url:
            raise ValueError('DB_URL environment variable is not set')
        return Config(debug, db_url)
    except Exception as e:
        logging.error(f'Error loading configuration: {str(e)}')
        raise

def get_config() -> Optional[Config]:
    """
    Gets the configuration.

    Returns:
    Config: Configuration instance, or None if not loaded.
    """
    try:
        return load_config()
    except Exception as e:
        logging.error(f'Error getting configuration: {str(e)}')
        return None

logging.basicConfig(level=logging.INFO)
logging.info('Loaded configuration module')