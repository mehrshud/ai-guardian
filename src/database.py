from src.config import Config
from src.models import User, Vulnerability, PerformanceMetric
import logging
import sqlite3

class Database:
    def __init__(self, config: Config):
        self.config = config
        self.conn = None

    def connect(self) -> None:
        try:
            self.conn = sqlite3.connect(self.config.db_url)
            logging.info("Connected to database")
        except sqlite3.Error as e:
            logging.error(f"Failed to connect to database: {e}", exc_info=True)

    def create_tables(self) -> None:
        try:
            cursor = self.conn.cursor()
            cursor.execute("""
                CREATE TABLE IF NOT EXISTS users (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    email TEXT NOT NULL
                )
            """)
            cursor.execute("""
                CREATE TABLE IF NOT EXISTS vulnerabilities (
                    id INTEGER PRIMARY KEY,
                    description TEXT NOT NULL,
                    severity TEXT NOT NULL
                )
            """)
            cursor.execute("""
                CREATE TABLE IF NOT EXISTS performance_metrics (
                    id INTEGER PRIMARY KEY,
                    metric TEXT NOT NULL,
                    value REAL NOT NULL
                )
            """)
            self.conn.commit()
            logging.info("Tables created")
        except sqlite3.Error as e:
            logging.error(f"Failed to create tables: {e}", exc_info=True)

    def insert_user(self, user: User) -> None:
        try:
            cursor = self.conn.cursor()
            cursor.execute("INSERT INTO users (id, name, email) VALUES (?, ?, ?)", 
                           (user.id, user.name, user.email))
            self.conn.commit()
            logging.info("User inserted")
        except sqlite3.Error as e:
            logging.error(f"Failed to insert user: {e}", exc_info=True)